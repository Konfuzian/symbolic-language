//! Conversion from other formats to SYM

use crate::Value;

/// Convert a serde_json::Value to SYM Value
pub fn from_json(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.iter().map(from_json).collect())
        }
        serde_json::Value::Object(obj) => {
            Value::Object(obj.iter().map(|(k, v)| (k.clone(), from_json(v))).collect())
        }
    }
}

/// Parse JSON string to SYM Value
pub fn parse_json(input: &str) -> Result<Value, String> {
    let json: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| format!("JSON parse error: {}", e))?;
    Ok(from_json(&json))
}

/// Convert a serde_yaml::Value to SYM Value
pub fn from_yaml(yaml: &serde_yaml::Value) -> Value {
    match yaml {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(*b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::String(n.to_string())
            }
        }
        serde_yaml::Value::String(s) => Value::String(s.clone()),
        serde_yaml::Value::Sequence(arr) => {
            Value::Array(arr.iter().map(from_yaml).collect())
        }
        serde_yaml::Value::Mapping(obj) => {
            Value::Object(
                obj.iter()
                    .filter_map(|(k, v)| {
                        let key = match k {
                            serde_yaml::Value::String(s) => s.clone(),
                            serde_yaml::Value::Number(n) => n.to_string(),
                            serde_yaml::Value::Bool(b) => b.to_string(),
                            _ => return None,
                        };
                        Some((key, from_yaml(v)))
                    })
                    .collect(),
            )
        }
        serde_yaml::Value::Tagged(tagged) => from_yaml(&tagged.value),
    }
}

/// Parse YAML string to SYM Value
pub fn parse_yaml(input: &str) -> Result<Value, String> {
    let yaml: serde_yaml::Value = serde_yaml::from_str(input)
        .map_err(|e| format!("YAML parse error: {}", e))?;
    Ok(from_yaml(&yaml))
}

/// Convert a toml::Value to SYM Value
pub fn from_toml(toml_val: &toml::Value) -> Value {
    match toml_val {
        toml::Value::Boolean(b) => Value::Bool(*b),
        toml::Value::Integer(i) => Value::Int(*i),
        toml::Value::Float(f) => Value::Float(*f),
        toml::Value::String(s) => Value::String(s.clone()),
        toml::Value::Array(arr) => {
            Value::Array(arr.iter().map(from_toml).collect())
        }
        toml::Value::Table(obj) => {
            Value::Object(obj.iter().map(|(k, v)| (k.clone(), from_toml(v))).collect())
        }
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
    }
}

/// Parse TOML string to SYM Value
pub fn parse_toml(input: &str) -> Result<Value, String> {
    let toml_val: toml::Value = toml::from_str(input)
        .map_err(|e| format!("TOML parse error: {}", e))?;
    Ok(from_toml(&toml_val))
}

/// Format a SYM Value as SYM syntax
pub fn to_sym_string(value: &Value, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    let inner_prefix = "  ".repeat(indent + 1);
    
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => {
            if f.is_nan() {
                "nan".to_string()
            } else if f.is_infinite() {
                if *f > 0.0 { "inf".to_string() } else { "-inf".to_string() }
            } else {
                f.to_string()
            }
        }
        Value::String(s) => escape_sym_string(s),
        Value::Symbol(s) => format!(":{}", s),
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                let mut result = String::from("[");
                for (i, v) in arr.iter().enumerate() {
                    if i == 0 {
                        result.push(' ');
                    } else {
                        result.push_str(&format!("\n{}, ", inner_prefix));
                    }
                    result.push_str(&to_sym_string(v, indent + 1));
                }
                result.push_str(&format!("\n{}]", prefix));
                result
            }
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let mut result = String::from("{");
                let entries: Vec<_> = obj.iter().collect();
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i == 0 {
                        result.push(' ');
                    } else {
                        result.push_str(&format!("\n{}, ", inner_prefix));
                    }
                    result.push_str(&format!(":{} ", escape_key(k)));
                    result.push_str(&to_sym_string(v, indent + 1));
                }
                result.push_str(&format!("\n{}}}", prefix));
                result
            }
        }
    }
}

/// Escape a string for SYM output
fn escape_sym_string(s: &str) -> String {
    // Check if the string needs escaping at the start
    let needs_escape = s.starts_with(':')
        || s.starts_with('$')
        || s.starts_with('{')
        || s.starts_with('[')
        || s.starts_with('\\')
        || s == "true"
        || s == "false"
        || s == "null"
        || s == "inf"
        || s == "-inf"
        || s == "nan"
        || looks_like_number(s);
    
    if needs_escape {
        format!("\\{}", s)
    } else if s.is_empty() {
        // Empty string - just leave it empty (will be empty value)
        String::new()
    } else if s.contains('\n') {
        // Multiline string - indent each line
        s.to_string()
    } else {
        s.to_string()
    }
}

/// Check if a string looks like a number
fn looks_like_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    let s = s.trim();
    
    // Check for hex, binary, octal
    if s.starts_with("0x") || s.starts_with("0X") 
        || s.starts_with("0b") || s.starts_with("0B")
        || s.starts_with("0o") || s.starts_with("0O") {
        return true;
    }
    
    // Check for decimal number
    let mut chars = s.chars().peekable();
    
    // Optional minus
    if chars.peek() == Some(&'-') {
        chars.next();
    }
    
    // Must start with digit
    match chars.peek() {
        Some(c) if c.is_ascii_digit() => {}
        _ => return false,
    }
    
    // Rest can be digits, underscores, dot, e/E
    for c in chars {
        if !c.is_ascii_digit() && c != '_' && c != '.' && c != 'e' && c != 'E' && c != '+' && c != '-' {
            return false;
        }
    }
    
    true
}

/// Escape a key (remove invalid characters)
fn escape_key(k: &str) -> String {
    // Keys must be valid identifiers
    let mut result = String::new();
    let mut first = true;
    
    for c in k.chars() {
        if first {
            if c.is_alphabetic() || c == '_' {
                result.push(c);
                first = false;
            } else if c.is_ascii_digit() {
                result.push('_');
                result.push(c);
                first = false;
            }
        } else {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                result.push(c);
            } else if c == ' ' {
                result.push('-');
            }
        }
    }
    
    if result.is_empty() {
        "_".to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_sym() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let value = parse_json(json).unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn test_yaml_to_sym() {
        let yaml = "name: Alice\nage: 30";
        let value = parse_yaml(yaml).unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn test_toml_to_sym() {
        let toml = "name = \"Alice\"\nage = 30";
        let value = parse_toml(toml).unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn test_escape_number_string() {
        assert_eq!(escape_sym_string("42"), "\\42");
        assert_eq!(escape_sym_string("hello"), "hello");
        assert_eq!(escape_sym_string(":foo"), "\\:foo");
    }
}
