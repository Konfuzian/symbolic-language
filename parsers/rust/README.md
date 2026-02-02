# SYM Rust Parser

A complete SYM parser implementation in Rust with CLI and library support.

## Installation

```bash
cargo build --release
```

## CLI Usage

```bash
# Parse and display as debug output
sym-parser config.sym

# Parse and output as JSON
sym-parser config.sym --json

# Read from stdin
cat config.sym | sym-parser -

# Convert from other formats
sym-parser --from-json config.json
sym-parser --from-yaml config.yaml
sym-parser --from-toml config.toml
```

## Library Usage

```rust
use sym_parser::{parse, parse_file, Value};

// Parse a string
let input = r#"
{ :name my-app
, :version 1.0.0
}
"#;
let value = parse(input)?;

// Parse a file
let value = parse_file("config.sym")?;

// Work with values
match value {
    Value::Object(map) => {
        for (key, val) in map {
            println!("{}: {:?}", key, val);
        }
    }
    _ => {}
}
```

## Value Types

The parser produces these value types:

```rust
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Symbol(String),      // :keyword
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}
```

## Features

- Full SYM specification support
- Variable substitution
- Multiline strings
- Comments (line and block)
- Escape sequences
- Conversion from JSON, YAML, TOML
- Preserves key order in objects

## Error Handling

The parser provides detailed error messages with line/column information:

```
Error at line 5, column 12: Unexpected character '}'
```
