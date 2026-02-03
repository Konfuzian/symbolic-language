# SYM Parser Test Harness

A comprehensive test harness for the SYM format parser that automatically discovers and runs all test cases.

## Overview

This test harness:
- Automatically discovers all test cases in `tests/cases/`
- Runs both success tests (with `expected.json`) and error tests (with `error.json`)
- Provides clear, colorized output showing which tests pass or fail
- Compares parser output against expected JSON output
- Validates that error cases produce expected errors

## Test Case Structure

Each test case is a directory under `tests/cases/` containing:

### Success Tests
- `input.sym` - The SYM format input to parse
- `expected.json` - The expected JSON output after parsing
- `meta.json` (optional) - Test metadata and description

### Error Tests
- `input.sym` - The SYM format input that should fail to parse
- `error.json` - Expected error pattern (contains `pattern` or `message` field)
- `meta.json` (optional) - Test metadata and description

## Usage

### Running All Tests

From this directory:

```bash
cargo run
```

Or build and run the binary:

```bash
cargo build --release
./target/release/test-harness
```

### Running from Anywhere

From the repository root:

```bash
cd tests/harness/rust
cargo run
```

## Output Format

The test harness provides colorized output:

```
SYM Parser Test Harness
============================================================

7 test cases found

[✓ PASS] basic/empty-object
[✓ PASS] basic/simple-string
[✗ FAIL] basic/complex-nested
  Output mismatch:
  Expected: {...}
  Got:      {...}

============================================================
Test Summary
============================================================
Total:  7 tests
Passed: 6 tests
Failed: 1 tests
Pass rate: 85.7%
```

## How It Works

1. **Test Discovery**: The harness walks through `tests/cases/` and finds all directories containing:
   - `input.sym` + `expected.json` (success tests), or
   - `input.sym` + `error.json` (error tests)

2. **Test Execution**:
   - **Success tests**: Parse `input.sym` and compare the result with `expected.json`
   - **Error tests**: Try to parse `input.sym` and verify it produces an error matching the pattern in `error.json`

3. **Comparison**:
   - The parser's `Value` type is converted to `serde_json::Value`
   - Deep equality comparison handles floating-point precision
   - Symbols are represented as strings with `:` prefix in JSON

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed, or test harness error

## Dependencies

- `sym-parser` - The SYM parser being tested (from `parsers/rust/`)
- `serde_json` - JSON parsing and comparison
- `colored` - Colorized terminal output
- `walkdir` - Recursive directory traversal

## Adding New Tests

To add a new test case:

1. Create a new directory under `tests/cases/category/test-name/`
2. Add `input.sym` with your test input
3. Add `expected.json` (for success) or `error.json` (for errors)
4. Optionally add `meta.json` with test description
5. Run the harness - the test will be automatically discovered

Example success test:

```
tests/cases/numbers/hex-numbers/
  ├── input.sym         # { :value 0xFF }
  ├── expected.json     # { "value": 255 }
  └── meta.json         # { "description": "Hexadecimal numbers" }
```

Example error test:

```
tests/cases/errors/invalid-syntax/
  ├── input.sym         # { :key :value
  ├── error.json        # { "pattern": "expected }" }
  └── meta.json         # { "description": "Unclosed object" }
```

## Development

### Building

```bash
cargo build
```

### Running in Debug Mode

```bash
cargo run
```

### Running Tests

The harness itself can be tested:

```bash
cargo test
```

### Code Structure

- `src/main.rs` - Main test harness implementation
  - `discover_test_cases()` - Finds all test cases
  - `run_test_case()` - Executes a single test
  - `run_success_test()` - Handles success test cases
  - `run_error_test()` - Handles error test cases
  - `sym_value_to_json()` - Converts SYM values to JSON for comparison
  - `values_equal()` - Deep equality with float handling

## Troubleshooting

### "Test cases directory not found"

Make sure you're running from the correct directory or that the path to `tests/cases/` is correct.

### "No test cases found"

Check that:
- Test case directories are under `tests/cases/`
- Each test has `input.sym`
- Each test has either `expected.json` or `error.json`

### Parser Not Found

If you get a compilation error about `sym-parser`, ensure the parser is built:

```bash
cd ../../../parsers/rust
cargo build
```

## License

Same as the parent project.
