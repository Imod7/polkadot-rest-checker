use std::process::Command;
use std::time::Instant;
use tokio::sync::watch;
use tokio::task::JoinHandle;

/// Git commit information for a repository.
#[derive(Debug, Clone)]
pub struct GitInfo {
    pub branch: String,
    pub commit_short: String,
    pub commit_message: String,
}

impl GitInfo {
    /// Detect git branch and latest commit from a repo path.
    pub fn from_repo(repo_path: &str) -> Option<Self> {
        let branch = Command::new("git")
            .args(["-C", repo_path, "branch", "--show-current"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let log_output = Command::new("git")
            .args(["-C", repo_path, "log", "--oneline", "-1"])
            .output()
            .ok()
            .filter(|o| o.status.success())?;

        let line = String::from_utf8_lossy(&log_output.stdout).trim().to_string();
        let (commit_short, commit_message) = line.split_once(' ')
            .map(|(h, m)| (h.to_string(), m.to_string()))
            .unwrap_or_else(|| (line.clone(), String::new()));

        Some(GitInfo {
            branch,
            commit_short,
            commit_message,
        })
    }

    /// Format as a single-line summary.
    pub fn summary(&self) -> String {
        format!("{} ({}) {}", self.commit_short, self.branch, self.commit_message)
    }
}

/// A single memory sample for one process.
#[derive(Debug, Clone)]
struct MemorySample {
    rss_kb: u64,
}

/// Computed statistics for one process.
#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub label: String,
    pub pid: u32,
    pub baseline_kb: u64,
    pub peak_kb: u64,
    pub average_kb: u64,
    pub final_kb: u64,
    pub growth_kb: i64,
    pub growth_percent: f64,
}

/// Final memory comparison report for both servers.
#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub rust_api: Option<ProcessStats>,
    pub sidecar: Option<ProcessStats>,
    pub duration_secs: f64,
    pub sample_count: usize,
    pub rust_git: Option<GitInfo>,
    pub sidecar_git: Option<GitInfo>,
}

/// Handle to the running background monitor. Call `stop()` to get the report.
pub struct MemoryMonitor {
    handle: JoinHandle<MemoryReport>,
    shutdown: watch::Sender<bool>,
}

/// Extract port number from a URL string like "http://localhost:8080/v1".
fn extract_port(url: &str) -> Option<u16> {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let host_port = after_scheme.split('/').next().unwrap_or(after_scheme);
    host_port.rsplit(':').next()?.parse::<u16>().ok()
}

/// Find PID of process listening on the given TCP port using `lsof`.
async fn find_pid_by_port(port: u16) -> Option<u32> {
    let output = tokio::process::Command::new("lsof")
        .args([
            &format!("-iTCP:{}", port),
            "-sTCP:LISTEN",
            "-t",
        ])
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().next()?.trim().parse::<u32>().ok()
}

/// Get RSS (Resident Set Size) in KB for a process using `ps`.
async fn get_rss_kb(pid: u32) -> Option<u64> {
    let output = tokio::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.trim().parse::<u64>().ok()
}

fn compute_stats(
    label: &str,
    pid: u32,
    samples: &[MemorySample],
) -> Option<ProcessStats> {
    if samples.is_empty() {
        return None;
    }

    let baseline = samples.first().unwrap().rss_kb;
    let final_val = samples.last().unwrap().rss_kb;
    let peak = samples.iter().map(|s| s.rss_kb).max().unwrap();
    let avg = samples.iter().map(|s| s.rss_kb).sum::<u64>() / samples.len() as u64;
    let growth = final_val as i64 - baseline as i64;
    let growth_pct = if baseline > 0 {
        (growth as f64 / baseline as f64) * 100.0
    } else {
        0.0
    };

    Some(ProcessStats {
        label: label.to_string(),
        pid,
        baseline_kb: baseline,
        peak_kb: peak,
        average_kb: avg,
        final_kb: final_val,
        growth_kb: growth,
        growth_percent: growth_pct,
    })
}

impl MemoryMonitor {
    /// Start monitoring memory for both server processes.
    /// Returns `None` if neither PID could be detected.
    pub async fn start(
        rust_url: &str,
        sidecar_url: &str,
        interval_ms: u64,
    ) -> Option<Self> {
        let rust_pid_info = match extract_port(rust_url) {
            Some(port) => {
                let pid = find_pid_by_port(port).await;
                if pid.is_none() {
                    eprintln!(
                        "Warning: Could not find process on port {} (Rust API). Memory monitoring skipped for this server.",
                        port
                    );
                }
                pid.map(|p| (p, port))
            }
            None => {
                eprintln!("Warning: Could not extract port from Rust API URL");
                None
            }
        };

        let sidecar_pid_info = match extract_port(sidecar_url) {
            Some(port) => {
                let pid = find_pid_by_port(port).await;
                if pid.is_none() {
                    eprintln!(
                        "Warning: Could not find process on port {} (Sidecar). Memory monitoring skipped for this server.",
                        port
                    );
                }
                pid.map(|p| (p, port))
            }
            None => {
                eprintln!("Warning: Could not extract port from Sidecar URL");
                None
            }
        };

        if rust_pid_info.is_none() && sidecar_pid_info.is_none() {
            eprintln!("Warning: No server processes found. Memory monitoring disabled.");
            return None;
        }

        if let Some((pid, port)) = rust_pid_info {
            println!("Memory monitor: Rust API on port {} -> PID {}", port, pid);
        }
        if let Some((pid, port)) = sidecar_pid_info {
            println!("Memory monitor: Sidecar on port {} -> PID {}", port, pid);
        }

        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        let start_time = Instant::now();

        let handle = tokio::spawn(async move {
            let mut rust_samples: Vec<MemorySample> = Vec::new();
            let mut sidecar_samples: Vec<MemorySample> = Vec::new();
            let mut interval =
                tokio::time::interval(std::time::Duration::from_millis(interval_ms));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Some((pid, _)) = rust_pid_info {
                            if let Some(rss) = get_rss_kb(pid).await {
                                rust_samples.push(MemorySample { rss_kb: rss });
                            }
                        }

                        if let Some((pid, _)) = sidecar_pid_info {
                            if let Some(rss) = get_rss_kb(pid).await {
                                sidecar_samples.push(MemorySample { rss_kb: rss });
                            }
                        }
                    }
                    _ = shutdown_rx.changed() => {
                        break;
                    }
                }
            }

            let duration = start_time.elapsed();
            let sample_count = rust_samples.len().max(sidecar_samples.len());

            let rust_api = rust_pid_info
                .and_then(|(pid, _)| compute_stats("Rust API", pid, &rust_samples));
            let sidecar = sidecar_pid_info
                .and_then(|(pid, _)| compute_stats("Sidecar", pid, &sidecar_samples));

            MemoryReport {
                rust_api,
                sidecar,
                duration_secs: duration.as_secs_f64(),
                sample_count,
                rust_git: None,
                sidecar_git: None,
            }
        });

        Some(MemoryMonitor {
            handle,
            shutdown: shutdown_tx,
        })
    }

    /// Stop monitoring and return the memory report.
    pub async fn stop(self) -> MemoryReport {
        let _ = self.shutdown.send(true);
        self.handle.await.unwrap_or_else(|_| MemoryReport {
            rust_api: None,
            sidecar: None,
            duration_secs: 0.0,
            sample_count: 0,
            rust_git: None,
            sidecar_git: None,
        })
    }
}

impl MemoryReport {
    /// Print a formatted summary table to the terminal.
    pub fn print_summary(&self) {
        fn format_mb(kb: u64) -> String {
            format!("{:.1} MB", kb as f64 / 1024.0)
        }

        fn format_growth(stats: &ProcessStats) -> String {
            let sign = if stats.growth_kb >= 0 { "+" } else { "" };
            format!(
                "{}{:.1} MB ({}{:.1}%)",
                sign,
                stats.growth_kb as f64 / 1024.0,
                sign,
                stats.growth_percent
            )
        }

        println!("\n{}", "=".repeat(90));
        println!(
            "                              MEMORY CONSUMPTION REPORT"
        );
        println!("{}", "=".repeat(90));
        println!(
            "Monitoring duration: {:.1}s ({} samples)\n",
            self.duration_secs, self.sample_count
        );

        if let Some(ref git) = self.rust_git {
            println!("Rust API commit:  {}", git.summary());
        }
        if let Some(ref git) = self.sidecar_git {
            println!("Sidecar commit:   {}", git.summary());
        }
        if self.rust_git.is_some() || self.sidecar_git.is_some() {
            println!();
        }

        if self.rust_api.is_none() && self.sidecar.is_none() {
            println!("No memory data collected.");
            return;
        }

        println!(
            "{:<25} {:>12} {:>12} {:>12} {:>12} {:>18}",
            "Server", "Baseline", "Peak", "Average", "Final", "Growth"
        );
        println!("{}", "-".repeat(90));

        if let Some(ref stats) = self.rust_api {
            println!(
                "{:<25} {:>12} {:>12} {:>12} {:>12} {:>18}",
                format!("Rust API (PID {})", stats.pid),
                format_mb(stats.baseline_kb),
                format_mb(stats.peak_kb),
                format_mb(stats.average_kb),
                format_mb(stats.final_kb),
                format_growth(stats),
            );
        }

        if let Some(ref stats) = self.sidecar {
            println!(
                "{:<25} {:>12} {:>12} {:>12} {:>12} {:>18}",
                format!("Sidecar (PID {})", stats.pid),
                format_mb(stats.baseline_kb),
                format_mb(stats.peak_kb),
                format_mb(stats.average_kb),
                format_mb(stats.final_kb),
                format_growth(stats),
            );
        }

        println!("{}", "-".repeat(90));
    }

    /// Generate a markdown table for file output.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("## Git Commits\n\n");
        if let Some(ref git) = self.rust_git {
            md.push_str(&format!("- **Rust API**: `{}` ({}) {}\n", git.commit_short, git.branch, git.commit_message));
        }
        if let Some(ref git) = self.sidecar_git {
            md.push_str(&format!("- **Sidecar**: `{}` ({}) {}\n", git.commit_short, git.branch, git.commit_message));
        }
        if self.rust_git.is_none() && self.sidecar_git.is_none() {
            md.push_str("- No git info available\n");
        }
        md.push_str("\n## Memory Consumption\n\n");
        md.push_str(&format!(
            "Monitoring duration: {:.1}s ({} samples)\n\n",
            self.duration_secs, self.sample_count
        ));

        md.push_str("| Server | PID | Baseline | Peak | Average | Final | Growth |\n");
        md.push_str("|--------|-----|----------|------|---------|-------|--------|\n");

        for stats_opt in [&self.rust_api, &self.sidecar] {
            if let Some(stats) = stats_opt {
                let sign = if stats.growth_kb >= 0 { "+" } else { "" };
                md.push_str(&format!(
                    "| {} | {} | {:.1} MB | {:.1} MB | {:.1} MB | {:.1} MB | {}{:.1} MB ({}{:.1}%) |\n",
                    stats.label,
                    stats.pid,
                    stats.baseline_kb as f64 / 1024.0,
                    stats.peak_kb as f64 / 1024.0,
                    stats.average_kb as f64 / 1024.0,
                    stats.final_kb as f64 / 1024.0,
                    sign,
                    stats.growth_kb as f64 / 1024.0,
                    sign,
                    stats.growth_percent,
                ));
            }
        }

        md.push('\n');
        md
    }
}
