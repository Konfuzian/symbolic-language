use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
struct TestCase {
    name: String,
    path: PathBuf,
    test_type: TestType,
}

#[derive(Debug)]
enum TestType {
    Success,  // Has expected.json
    Error,    // Has error.json
}

#[derive(Debug)]
struct TestResult {
    name: String,
    passed: bool,
    message: String,
}

fn main() {
    println!("{}", "SYM Parser Test Harness".bold().cyan());
    println!("{}", "=".repeat(60).cyan());
    println!();

    // Find the test cases directory relative to this binary
    let test_cases_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../cases");

    if !test_cases_dir.exists() {
        eprintln!("{}", format!("Error: Test cases directory not found at {:?}", test_cases_dir).red());
        std::process::exit(1);
    }

    // Discover all test cases
    let test_cases = discover_test_cases(&test_cases_dir);

    if test_cases.is_empty() {
        println!("{}", "No test cases found!".yellow());
        return;
    }

    println!("{} test cases found\n", test_cases.len());

    // Run all test cases
    let mut results = Vec::new();
    let mut passed_count = 0;
    let mut failed_count = 0;

    for test_case in test_cases {
        let result = run_test_case(&test_case);

        // Print result
        let status = if result.passed {
            "✓ PASS".green()
        } else {
            "✗ FAIL".red()
        };

        println!("[{}] {}", status, result.name);
        if !result.passed {
            println!("  {}", result.message.dimmed());
        }

        if result.passed {
            passed_count += 1;
        } else {
            failed_count += 1;
        }

        results.push(result);
    }

    // Print summary
    println!();
    println!("{}", "=".repeat(60).cyan());
    println!("{}", "Test Summary".bold().cyan());
    println!("{}", "=".repeat(60).cyan());

    let total = passed_count + failed_count;
    let pass_rate = if total > 0 {
        (passed_count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("Total:  {} tests", total);
    println!("Passed: {} tests", passed_count.to_string().green());
    println!("Failed: {} tests", failed_count.to_string().red());
    println!("Pass rate: {:.1}%", pass_rate);

    if failed_count > 0 {
        println!();
        println!("{}", "Failed tests:".yellow());
        for result in results.iter().filter(|r| !r.passed) {
            println!("  • {}", result.name);
            println!("    {}", result.message.dimmed());
        }
        std::process::exit(1);
    } else {
        println!();
        println!("{}", "All tests passed! 🎉".green().bold());
    }
}

fn discover_test_cases(base_dir: &Path) -> Vec<TestCase> {
    let mut test_cases = Vec::new();

    for entry in WalkDir::new(base_dir)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let path = entry.path();
        let input_file = path.join("input.sym");
        let expected_file = path.join("expected.json");
        let error_file = path.join("error.json");

        // Only include test cases that have input.sym and either expected.json or error.json
        if input_file.exists() {
            let test_type = if expected_file.exists() {
                TestType::Success
            } else if error_file.exists() {
                TestType::Error
            } else {
                // Skip test cases without expected or error files
                continue;
            };

            // Generate test name from path
            let name = path
                .strip_prefix(base_dir)
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace('\\', "/");

            test_cases.push(TestCase {
                name,
                path: path.to_path_buf(),
                test_type,
            });
        }
    }

    // Sort by name for consistent output
    test_cases.sort_by(|a, b| a.name.cmp(&b.name));
    test_cases
}

fn run_test_case(test_case: &TestCase) -> TestResult {
    let input_file = test_case.path.join("input.sym");

    // Read input
    let input = match fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            return TestResult {
                name: test_case.name.clone(),
                passed: false,
                message: format!("Failed to read input.sym: {}", e),
            };
        }
    };

    match test_case.test_type {
        TestType::Success => run_success_test(test_case, &input),
        TestType::Error => run_error_test(test_case, &input),
    }
}

fn run_success_test(test_case: &TestCase, input: &str) -> TestResult {
    let expected_file = test_case.path.join("expected.json");

    // Read expected output
    let expected_json = match fs::read_to_string(&expected_file) {
        Ok(content) => content,
        Err(e) => {
            return TestResult {
                name: test_case.name.clone(),
                passed: false,
                message: format!("Failed to read expected.json: {}", e),
            };
        }
    };

    // Parse expected JSON
    let expected: serde_json::Value = match serde_json::from_str(&expected_json) {
        Ok(v) => v,
        Err(e) => {
            return TestResult {
                name: test_case.name.clone(),
                passed: false,
                message: format!("Failed to parse expected.json: {}", e),
            };
        }
    };

    // Parse input SYM
    let parsed = match sym_parser::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return TestResult {
                name: test_case.name.clone(),
                passed: false,
                message: format!("Parser error: {}", e),
            };
        }
    };

    // Convert parsed SYM value to JSON for comparison
    let parsed_json = sym_value_to_json(&parsed);

    // Compare
    if values_equal(&expected, &parsed_json) {
        TestResult {
            name: test_case.name.clone(),
            passed: true,
            message: String::new(),
        }
    } else {
        TestResult {
            name: test_case.name.clone(),
            passed: false,
            message: format!(
                "Output mismatch:\n  Expected: {}\n  Got:      {}",
                serde_json::to_string_pretty(&expected).unwrap_or_default(),
                serde_json::to_string_pretty(&parsed_json).unwrap_or_default()
            ),
        }
    }
}

fn run_error_test(test_case: &TestCase, input: &str) -> TestResult {
    let error_file = test_case.path.join("error.json");

    // Read expected error
    let error_json = match fs::read_to_string(&error_file) {
        Ok(content) => content,
        Err(e) => {
            return TestResult {
                name: test_case.name.clone(),
                passed: false,
                message: format!("Failed to read error.json: {}", e),
            };
        }
    };

    // Parse error specification
    let error_spec: serde_json::Value = match serde_json::from_str(&error_json) {
        Ok(v) => v,
        Err(e) => {
            return TestResult {
                name: test_case.name.clone(),
                passed: false,
                message: format!("Failed to parse error.json: {}", e),
            };
        }
    };

    // Try to parse input - should fail
    let result = sym_parser::parse(input);

    match result {
        Ok(_) => TestResult {
            name: test_case.name.clone(),
            passed: false,
            message: "Expected parser error, but parsing succeeded".to_string(),
        },
        Err(e) => {
            // Check if error matches expected pattern
            let error_message = e.to_string();

            // Extract expected pattern from error.json
            let expected_pattern = if let Some(pattern) = error_spec.get("pattern") {
                pattern.as_str().unwrap_or("")
            } else if let Some(msg) = error_spec.get("message") {
                msg.as_str().unwrap_or("")
            } else {
                ""
            };

            if expected_pattern.is_empty() || error_message.contains(expected_pattern) {
                TestResult {
                    name: test_case.name.clone(),
                    passed: true,
                    message: String::new(),
                }
            } else {
                TestResult {
                    name: test_case.name.clone(),
                    passed: false,
                    message: format!(
                        "Error message mismatch:\n  Expected pattern: {}\n  Got: {}",
                        expected_pattern, error_message
                    ),
                }
            }
        }
    }
}

/// Convert a SYM Value to serde_json::Value for comparison
fn sym_value_to_json(value: &sym_parser::Value) -> serde_json::Value {
    match value {
        sym_parser::Value::Null => serde_json::Value::Null,
        sym_parser::Value::Bool(b) => serde_json::Value::Bool(*b),
        sym_parser::Value::Int(i) => serde_json::Value::Number((*i).into()),
        sym_parser::Value::Float(f) => {
            // Handle special float values
            if f.is_nan() || f.is_infinite() {
                serde_json::Value::String(f.to_string())
            } else {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(*f).unwrap_or_else(|| {
                        serde_json::Number::from(0)
                    })
                )
            }
        }
        sym_parser::Value::String(s) => serde_json::Value::String(s.clone()),
        sym_parser::Value::Symbol(s) => {
            // Symbols are represented as strings starting with ':'
            serde_json::Value::String(format!(":{}", s))
        }
        sym_parser::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(sym_value_to_json).collect())
        }
        sym_parser::Value::Object(obj) => {
            let map: serde_json::Map<String, serde_json::Value> = obj
                .iter()
                .map(|(k, v)| (k.clone(), sym_value_to_json(v)))
                .collect();
            serde_json::Value::Object(map)
        }
    }
}

/// Deep equality check for JSON values, handling floating point comparison
fn values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    match (a, b) {
        (serde_json::Value::Null, serde_json::Value::Null) => true,
        (serde_json::Value::Bool(a), serde_json::Value::Bool(b)) => a == b,
        (serde_json::Value::Number(a), serde_json::Value::Number(b)) => {
            // Handle float comparison with epsilon
            if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                (af - bf).abs() < 1e-10
            } else {
                a == b
            }
        }
        (serde_json::Value::String(a), serde_json::Value::String(b)) => a == b,
        (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(av, bv)| values_equal(av, bv))
        }
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            a.len() == b.len()
                && a.iter().all(|(k, av)| {
                    b.get(k).map_or(false, |bv| values_equal(av, bv))
                })
        }
        _ => false,
    }
}
