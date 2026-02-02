//! SYM Parser CLI

use std::env;
use std::fs;
use std::io::{self, Read};
use sym_parser::{parse, convert, Value};

fn print_usage(program: &str) {
    eprintln!("Usage: {} [OPTIONS] <file>", program);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -e '<sym>'        Parse SYM expression");
    eprintln!("  -                 Read from stdin");
    eprintln!("  --json            Output as JSON");
    eprintln!("  --from-json       Convert JSON to SYM");
    eprintln!("  --from-yaml       Convert YAML to SYM");
    eprintln!("  --from-toml       Convert TOML to SYM");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} config.sym                    Parse SYM file", program);
    eprintln!("  {} --json config.sym             Output as JSON", program);
    eprintln!("  {} --from-json config.json       Convert JSON to SYM", program);
    eprintln!("  {} --from-yaml config.yaml       Convert YAML to SYM", program);
    eprintln!("  {} --from-toml config.toml       Convert TOML to SYM", program);
    eprintln!("  cat file.json | {} --from-json - Convert stdin JSON to SYM", program);
}

#[derive(PartialEq)]
enum InputFormat {
    Sym,
    Json,
    Yaml,
    Toml,
}

#[derive(PartialEq)]
enum OutputFormat {
    Sym,
    Json,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        std::process::exit(1);
    }

    let mut input_format = InputFormat::Sym;
    let mut output_format = OutputFormat::Sym;
    let mut input_source: Option<String> = None;
    let mut expression: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_usage(program);
                std::process::exit(0);
            }
            "--json" => {
                output_format = OutputFormat::Json;
            }
            "--from-json" => {
                input_format = InputFormat::Json;
            }
            "--from-yaml" => {
                input_format = InputFormat::Yaml;
            }
            "--from-toml" => {
                input_format = InputFormat::Toml;
            }
            "-e" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: Expected expression after -e");
                    std::process::exit(1);
                }
                expression = Some(args[i].clone());
            }
            arg => {
                input_source = Some(arg.to_string());
            }
        }
        i += 1;
    }

    // Get input content
    let input = if let Some(expr) = expression {
        expr
    } else if let Some(source) = input_source {
        if source == "-" {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => buffer,
                Err(e) => {
                    eprintln!("Error reading stdin: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            match fs::read_to_string(&source) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", source, e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        eprintln!("Error: No input specified");
        print_usage(program);
        std::process::exit(1);
    };

    // Parse input based on format
    let value = match input_format {
        InputFormat::Sym => {
            match parse(&input) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("SYM parse error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        InputFormat::Json => {
            match convert::parse_json(&input) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        InputFormat::Yaml => {
            match convert::parse_yaml(&input) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        InputFormat::Toml => {
            match convert::parse_toml(&input) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
    };

    // Output based on format
    match output_format {
        OutputFormat::Sym => {
            println!("{}", convert::to_sym_string(&value, 0));
        }
        OutputFormat::Json => {
            print_json(&value, 0);
            println!();
        }
    }
}

fn print_json(value: &Value, indent: usize) {
    let prefix = "  ".repeat(indent);
    match value {
        Value::Null => print!("null"),
        Value::Bool(b) => print!("{}", b),
        Value::Int(i) => print!("{}", i),
        Value::Float(f) => {
            if f.is_nan() {
                print!("null")
            } else if f.is_infinite() {
                print!("null")
            } else {
                print!("{}", f)
            }
        }
        Value::String(s) => print!("\"{}\"", escape_json_string(s)),
        Value::Symbol(s) => print!("\"{}\"", escape_json_string(&format!(":{}", s))),
        Value::Array(arr) => {
            if arr.is_empty() {
                print!("[]");
            } else {
                println!("[");
                for (i, v) in arr.iter().enumerate() {
                    print!("{}", "  ".repeat(indent + 1));
                    print_json(v, indent + 1);
                    if i < arr.len() - 1 {
                        println!(",");
                    } else {
                        println!();
                    }
                }
                print!("{}]", prefix);
            }
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                print!("{{}}");
            } else {
                println!("{{");
                let entries: Vec<_> = obj.iter().collect();
                for (i, (k, v)) in entries.iter().enumerate() {
                    print!("{}\"{}\": ", "  ".repeat(indent + 1), escape_json_string(k));
                    print_json(v, indent + 1);
                    if i < entries.len() - 1 {
                        println!(",");
                    } else {
                        println!();
                    }
                }
                print!("{}}}", prefix);
            }
        }
    }
}

fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}
