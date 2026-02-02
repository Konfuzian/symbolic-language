//! SYM Format Parser
//!
//! A parser for the SYM data format - a less verbose, easier to read alternative to JSON.

use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

pub mod parser;
pub mod convert;

/// Errors that can occur during parsing
#[derive(Error, Debug)]
pub enum SymError {
    #[error("Parse error at line {line}, column {col}: {message}")]
    ParseError {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("Undefined variable: ${0}")]
    UndefinedVariable(String),

    #[error("Duplicate variable without override: ${0} (use ${{0}}! to override)")]
    DuplicateVariable(String),

    #[error("Duplicate key without override: :{0} (use :{0}! to override)")]
    DuplicateKey(String),

    #[error("Cannot append to non-array: :{0}")]
    AppendToNonArray(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SymError>;

/// A SYM value - the core AST type
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null value
    Null,

    /// Boolean value
    Bool(bool),

    /// Integer value
    Int(i64),

    /// Floating point value
    Float(f64),

    /// String value (unquoted in source)
    String(String),

    /// Symbol value (e.g., :active, :redis)
    Symbol(String),

    /// Array of values
    Array(Vec<Value>),

    /// Object with string keys
    Object(HashMap<String, Value>),
}

impl Value {
    /// Returns true if this value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns true if this value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns true if this value is an integer
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if this value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if this value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns true if this value is a symbol
    pub fn is_symbol(&self) -> bool {
        matches!(self, Value::Symbol(_))
    }

    /// Returns true if this value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Returns true if this value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Try to get as bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as i64
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Try to get as f64
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Try to get as string slice
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as symbol name
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Value::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as array slice
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Try to get as object
    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Deep merge another value into this one
    /// Used for import merging
    pub fn deep_merge(&mut self, other: Value) {
        match (self, other) {
            (Value::Object(base), Value::Object(overlay)) => {
                for (key, value) in overlay {
                    if let Some(base_value) = base.get_mut(&key) {
                        base_value.deep_merge(value);
                    } else {
                        base.insert(key, value);
                    }
                }
            }
            (this, other) => {
                *this = other;
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => {
                if n.is_nan() {
                    write!(f, "nan")
                } else if n.is_infinite() {
                    if *n > 0.0 {
                        write!(f, "inf")
                    } else {
                        write!(f, "-inf")
                    }
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Symbol(s) => write!(f, ":{}", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, ":{} {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// Parse a SYM string into a Value
pub fn parse(input: &str) -> Result<Value> {
    parser::Parser::new(input).parse()
}

/// Parse a SYM file into a Value
pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<Value> {
    let content = std::fs::read_to_string(path)?;
    parse(&content)
}

/// Parse a SYM string with a custom variable scope
pub fn parse_with_vars(input: &str, vars: HashMap<String, Value>) -> Result<Value> {
    parser::Parser::with_vars(input, vars).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_object() {
        let input = r#"{ :name Alice
, :age 28
}"#;
        let result = parse(input).unwrap();
        assert!(result.is_object());
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("name").unwrap().as_str(), Some("Alice"));
        assert_eq!(obj.get("age").unwrap().as_int(), Some(28));
    }

    #[test]
    fn test_array() {
        let input = r#"[ one
, two
, three
]"#;
        let result = parse(input).unwrap();
        assert!(result.is_array());
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_str(), Some("one"));
    }

    #[test]
    fn test_symbols() {
        let input = r#"{ :status :active
, :level :info
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("status").unwrap().as_symbol(), Some("active"));
        assert_eq!(obj.get("level").unwrap().as_symbol(), Some("info"));
    }

    #[test]
    fn test_variables() {
        let input = r#"{ $name Alice }
{ :greeting Hello
, :user $name
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("user").unwrap().as_str(), Some("Alice"));
    }

    #[test]
    fn test_numbers() {
        let input = r#"{ :int 42
, :negative -17
, :hex 0xff
, :binary 0b1010
, :octal 0o755
, :float 3.14
, :scientific 1e10
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("int").unwrap().as_int(), Some(42));
        assert_eq!(obj.get("negative").unwrap().as_int(), Some(-17));
        assert_eq!(obj.get("hex").unwrap().as_int(), Some(255));
        assert_eq!(obj.get("binary").unwrap().as_int(), Some(10));
        assert_eq!(obj.get("octal").unwrap().as_int(), Some(493));
        assert_eq!(obj.get("float").unwrap().as_float(), Some(3.14));
    }

    #[test]
    fn test_multiline_string() {
        let input = r#"{ :poem
    Roses are red
    Violets are blue
, :author Anonymous
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        let poem = obj.get("poem").unwrap().as_str().unwrap();
        assert!(poem.contains("Roses are red"));
        assert!(poem.contains("Violets are blue"));
    }

    #[test]
    fn test_inline_comma() {
        let input = r#"{ :greeting Hello, world
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("greeting").unwrap().as_str(), Some("Hello, world"));
    }

    #[test]
    fn test_nested_object() {
        let input = r#"{ :server
  { :host localhost
  , :port 8080
  }
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        let server = obj.get("server").unwrap().as_object().unwrap();
        assert_eq!(server.get("host").unwrap().as_str(), Some("localhost"));
        assert_eq!(server.get("port").unwrap().as_int(), Some(8080));
    }

    #[test]
    fn test_escaping() {
        let input = r#"{ :price \$99.99
, :num \42
, :bool \true
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("price").unwrap().as_str(), Some("$99.99"));
        assert_eq!(obj.get("num").unwrap().as_str(), Some("42"));
        assert_eq!(obj.get("bool").unwrap().as_str(), Some("true"));
    }

    #[test]
    fn test_comments() {
        let input = r#"// This is a comment
{ :name Alice  // inline comment
, :age 28
/* block
   comment */
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("name").unwrap().as_str(), Some("Alice"));
        assert_eq!(obj.get("age").unwrap().as_int(), Some(28));
    }

    #[test]
    fn test_url_not_comment() {
        let input = r#"{ :url https://example.com
}"#;
        let result = parse(input).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(
            obj.get("url").unwrap().as_str(),
            Some("https://example.com")
        );
    }
}
