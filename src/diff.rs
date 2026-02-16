use serde_json::Value;

/// Represents a single difference between two JSON values
#[derive(Debug, Clone)]
pub struct JsonDiff {
    /// Path to the differing field (e.g., "at.height" or "extrinsics[0].method")
    pub path: String,
    /// Value from Rust API (None if field missing)
    pub rust_value: Option<Value>,
    /// Value from Sidecar API (None if field missing)
    pub sidecar_value: Option<Value>,
    /// Type of difference
    pub diff_type: DiffType,
}

#[derive(Debug, Clone)]
pub enum DiffType {
    /// Values are different
    ValueMismatch,
    /// Field exists in Rust but not in Sidecar
    MissingInSidecar,
    /// Field exists in Sidecar but not in Rust
    MissingInRust,
    /// Array lengths differ
    ArrayLengthMismatch,
    /// Type mismatch (e.g., string vs number)
    TypeMismatch,
}

impl std::fmt::Display for JsonDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.diff_type {
            DiffType::ValueMismatch => {
                write!(
                    f,
                    "{}: rust={} vs sidecar={}",
                    self.path,
                    self.rust_value
                        .as_ref()
                        .map_or("null".to_string(), |v| truncate_value(v, 100)),
                    self.sidecar_value
                        .as_ref()
                        .map_or("null".to_string(), |v| truncate_value(v, 100))
                )
            }
            DiffType::MissingInSidecar => {
                write!(
                    f,
                    "{}: missing in sidecar (rust={})",
                    self.path,
                    self.rust_value
                        .as_ref()
                        .map_or("null".to_string(), |v| truncate_value(v, 100))
                )
            }
            DiffType::MissingInRust => {
                write!(
                    f,
                    "{}: missing in rust (sidecar={})",
                    self.path,
                    self.sidecar_value
                        .as_ref()
                        .map_or("null".to_string(), |v| truncate_value(v, 100))
                )
            }
            DiffType::ArrayLengthMismatch => {
                write!(
                    f,
                    "{}: array length mismatch (rust={} vs sidecar={})",
                    self.path,
                    self.rust_value
                        .as_ref()
                        .and_then(|v| v.as_array())
                        .map_or(0, |a| a.len()),
                    self.sidecar_value
                        .as_ref()
                        .and_then(|v| v.as_array())
                        .map_or(0, |a| a.len())
                )
            }
            DiffType::TypeMismatch => {
                write!(
                    f,
                    "{}: type mismatch (rust={} vs sidecar={})",
                    self.path,
                    value_type_name(self.rust_value.as_ref()),
                    value_type_name(self.sidecar_value.as_ref())
                )
            }
        }
    }
}

/// Get a short type name for a JSON value
fn value_type_name(v: Option<&Value>) -> &'static str {
    match v {
        None => "null",
        Some(Value::Null) => "null",
        Some(Value::Bool(_)) => "bool",
        Some(Value::Number(_)) => "number",
        Some(Value::String(_)) => "string",
        Some(Value::Array(_)) => "array",
        Some(Value::Object(_)) => "object",
    }
}

/// Truncate a JSON value to a maximum length for display
fn truncate_value(v: &Value, max_len: usize) -> String {
    let s = match v {
        Value::String(s) => format!("\"{}\"", s),
        _ => v.to_string(),
    };
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s
    }
}

/// Compare two JSON values for equality, ignoring field order and string case
pub fn json_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            if a_map.len() != b_map.len() {
                return false;
            }
            a_map.iter().all(|(key, a_val)| {
                b_map
                    .get(key)
                    .map_or(false, |b_val| json_equal(a_val, b_val))
            })
        }
        (Value::Array(a_arr), Value::Array(b_arr)) => {
            if a_arr.len() != b_arr.len() {
                return false;
            }
            a_arr
                .iter()
                .zip(b_arr.iter())
                .all(|(a_val, b_val)| json_equal(a_val, b_val))
        }
        // Case-insensitive string comparison
        (Value::String(a_str), Value::String(b_str)) => {
            a_str.to_lowercase() == b_str.to_lowercase()
        }
        _ => a == b,
    }
}

/// Find all differences between two JSON values.
/// Results are sorted so non-TypeMismatch diffs appear first,
/// making it easier to spot real value/structural differences
/// before the expected type mismatches (e.g., number vs string).
pub fn json_diff(rust: &Value, sidecar: &Value) -> Vec<JsonDiff> {
    let mut diffs = Vec::new();
    json_diff_recursive(rust, sidecar, String::new(), &mut diffs);
    diffs.sort_by_key(|d| match d.diff_type {
        DiffType::TypeMismatch => 1,
        _ => 0,
    });
    diffs
}

/// Recursively find differences between two JSON values
fn json_diff_recursive(rust: &Value, sidecar: &Value, path: String, diffs: &mut Vec<JsonDiff>) {
    match (rust, sidecar) {
        (Value::Object(rust_map), Value::Object(sidecar_map)) => {
            // Check for fields in rust but not in sidecar
            for (key, rust_val) in rust_map {
                let field_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                match sidecar_map.get(key) {
                    Some(sidecar_val) => {
                        json_diff_recursive(rust_val, sidecar_val, field_path, diffs);
                    }
                    None => {
                        diffs.push(JsonDiff {
                            path: field_path,
                            rust_value: Some(rust_val.clone()),
                            sidecar_value: None,
                            diff_type: DiffType::MissingInSidecar,
                        });
                    }
                }
            }

            // Check for fields in sidecar but not in rust
            for (key, sidecar_val) in sidecar_map {
                if !rust_map.contains_key(key) {
                    let field_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    diffs.push(JsonDiff {
                        path: field_path,
                        rust_value: None,
                        sidecar_value: Some(sidecar_val.clone()),
                        diff_type: DiffType::MissingInRust,
                    });
                }
            }
        }

        (Value::Array(rust_arr), Value::Array(sidecar_arr)) => {
            if rust_arr.len() != sidecar_arr.len() {
                diffs.push(JsonDiff {
                    path: path.clone(),
                    rust_value: Some(Value::Array(rust_arr.clone())),
                    sidecar_value: Some(Value::Array(sidecar_arr.clone())),
                    diff_type: DiffType::ArrayLengthMismatch,
                });
                // Still compare elements up to the shorter length
            }

            let min_len = rust_arr.len().min(sidecar_arr.len());
            for i in 0..min_len {
                let elem_path = if path.is_empty() {
                    format!("[{}]", i)
                } else {
                    format!("{}[{}]", path, i)
                };
                json_diff_recursive(&rust_arr[i], &sidecar_arr[i], elem_path, diffs);
            }
        }

        // Case-insensitive string comparison
        (Value::String(rust_str), Value::String(sidecar_str)) => {
            if rust_str.to_lowercase() != sidecar_str.to_lowercase() {
                diffs.push(JsonDiff {
                    path,
                    rust_value: Some(rust.clone()),
                    sidecar_value: Some(sidecar.clone()),
                    diff_type: DiffType::ValueMismatch,
                });
            }
        }

        // Type mismatch
        (_, _) if std::mem::discriminant(rust) != std::mem::discriminant(sidecar) => {
            diffs.push(JsonDiff {
                path,
                rust_value: Some(rust.clone()),
                sidecar_value: Some(sidecar.clone()),
                diff_type: DiffType::TypeMismatch,
            });
        }

        // Same type, different value
        (_, _) if rust != sidecar => {
            diffs.push(JsonDiff {
                path,
                rust_value: Some(rust.clone()),
                sidecar_value: Some(sidecar.clone()),
                diff_type: DiffType::ValueMismatch,
            });
        }

        // Equal values - no diff
        _ => {}
    }
}
