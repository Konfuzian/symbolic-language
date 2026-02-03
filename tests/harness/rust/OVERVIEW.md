# Test Harness Overview

## Summary

This is a comprehensive test harness for the SYM format parser written in Rust. It automatically discovers and executes all test cases, comparing parser output against expected results.

## Current Test Coverage

Based on the repository structure:
- **54 total test cases** in the repository
- **32 success tests** (with `expected.json`)
- **22 error tests** (with `error.json`)

## What's Included

### Core Implementation

1. **`src/main.rs`** (372 lines)
   - Automatic test case discovery
   - Test execution engine
   - Success and error test handling
   - Pretty-printed, colorized output
   - Deep equality comparison with float handling
   - Symbol representation (`:symbol` format)

### Configuration

2. **`Cargo.toml`**
   - Project metadata
   - Dependencies (sym-parser, serde_json, colored, walkdir)
   - Binary configuration

3. **`.gitignore`**
   - Excludes build artifacts
   - Keeps repository clean

### Documentation

4. **`README.md`**
   - Comprehensive documentation
   - Usage instructions
   - Architecture overview
   - How it works

5. **`QUICKSTART.md`**
   - Quick start guide
   - Prerequisites
   - Running instructions
   - Expected output format
   - CI/CD integration examples

6. **`TEST_FORMAT.md`**
   - Detailed test case format specification
   - JSON representation rules
   - How to create new tests
   - Common patterns and examples
   - Debugging tips

7. **`EXAMPLE_OUTPUT.md`**
   - Example outputs for various scenarios
   - Success and failure cases
   - Colorization examples
   - Exit codes

8. **`OVERVIEW.md`** (this file)
   - High-level overview
   - File structure
   - Quick reference

### Helper Scripts

9. **`run.sh`**
   - Convenience script to build and run
   - Makes running tests easy: `./run.sh`

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Test Harness                          │
│                                                         │
│  ┌──────────────┐         ┌──────────────┐            │
│  │   Discovery  │────────▶│  Execution   │            │
│  │              │         │              │            │
│  │ • Walk dirs  │         │ • Parse .sym │            │
│  │ • Find tests │         │ • Compare    │            │
│  │ • Categorize │         │ • Report     │            │
│  └──────────────┘         └──────────────┘            │
│         │                        │                     │
│         ▼                        ▼                     │
│  ┌──────────────┐         ┌──────────────┐            │
│  │ Test Cases   │         │   Results    │            │
│  │              │         │              │            │
│  │ • input.sym  │         │ • Pass/Fail  │            │
│  │ • expected   │         │ • Messages   │            │
│  │ • error      │         │ • Summary    │            │
│  └──────────────┘         └──────────────┘            │
└─────────────────────────────────────────────────────────┘
                     │
                     ▼
          ┌──────────────────────┐
          │    SYM Parser        │
          │   (parsers/rust/)    │
          └──────────────────────┘
```

## How It Works

### 1. Discovery Phase
```rust
discover_test_cases(&test_cases_dir)
  ├─ Walk through tests/cases/
  ├─ Find directories with input.sym
  ├─ Check for expected.json or error.json
  └─ Create TestCase structs
```

### 2. Execution Phase
```rust
for test_case in test_cases {
    if has_expected_json {
        run_success_test()
          ├─ Parse input.sym with sym-parser
          ├─ Convert to JSON
          └─ Compare with expected.json
    } else if has_error_json {
        run_error_test()
          ├─ Try to parse input.sym
          ├─ Expect error
          └─ Match error pattern
    }
}
```

### 3. Reporting Phase
```rust
print_results()
  ├─ Show pass/fail for each test
  ├─ Show detailed errors for failures
  ├─ Print summary statistics
  └─ Exit with appropriate code
```

## Key Features

### Automatic Discovery
- No manual test registration
- Just add files to `tests/cases/`
- Harness finds them automatically

### Two Test Types
- **Success tests**: Verify correct parsing
- **Error tests**: Verify error handling

### Rich Output
- Colorized terminal output
- Clear pass/fail indicators
- Detailed error messages
- Summary statistics

### Accurate Comparison
- Deep equality checking
- Float comparison with epsilon
- Handle special float values (inf, nan)
- Symbol representation (`:symbol`)

### CI/CD Ready
- Standard exit codes (0=pass, 1=fail)
- Machine-readable output possible
- Fast execution
- Standalone binary

## File Structure

```
tests/harness/rust/
├── Cargo.toml              # Rust project configuration
├── .gitignore              # Ignore build artifacts
├── run.sh                  # Convenience script
├── README.md               # Main documentation
├── QUICKSTART.md           # Getting started guide
├── TEST_FORMAT.md          # Test case format spec
├── EXAMPLE_OUTPUT.md       # Example outputs
├── OVERVIEW.md             # This file
└── src/
    └── main.rs             # Main implementation
```

## Dependencies

### Direct Dependencies
- **sym-parser**: The parser being tested (local path)
- **serde_json**: JSON parsing and serialization
- **colored**: Terminal color output
- **walkdir**: Recursive directory traversal

### System Requirements
- Rust 1.70+ (2021 edition)
- Cargo (Rust package manager)
- Unix-like environment (Linux, macOS, WSL)

## Usage Summary

### Quick Start
```bash
cd tests/harness/rust
./run.sh
```

### Manual Build
```bash
cargo build --release
./target/release/test-harness
```

### During Development
```bash
cargo run
```

## Test Case Categories

The repository includes tests for:

- **basic/** - Core functionality (objects, arrays, primitives)
- **arrays/** - Array parsing and nesting
- **objects/** - Object parsing and nesting
- **strings/** - String handling (multiline, escaping)
- **numbers/** - Number formats (int, float, hex, binary, octal)
- **symbols/** - Symbol syntax (`:symbol`)
- **variables/** - Variable definitions and references (`$var`)
- **comments/** - Comment handling (single-line, block)
- **edge-cases/** - Special cases and corner cases
- **errors/** - Invalid syntax error handling

## Metrics

- **Total lines of code**: ~372 lines (main.rs)
- **Documentation pages**: 5 markdown files
- **Test coverage**: 54 test cases
- **Success rate**: Run to find out!

## Integration Points

### With Parser
```rust
use sym_parser;

// Parse input
let result = sym_parser::parse(input);

// Convert to JSON for comparison
let json = sym_value_to_json(&result);
```

### With Test Cases
```
tests/cases/
└── category/
    └── test-name/
        ├── input.sym       → Read by harness
        ├── expected.json   → Compared against
        └── meta.json       → Documentation only
```

## Next Steps

1. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Build the harness**
   ```bash
   cd tests/harness/rust
   cargo build --release
   ```

3. **Run the tests**
   ```bash
   cargo run --release
   ```

4. **Add more tests** as needed in `tests/cases/`

5. **Integrate with CI/CD** for automated testing

## Troubleshooting

See [README.md](README.md) for detailed troubleshooting information.

Common issues:
- **Rust not installed**: Install from https://rustup.rs/
- **Parser not found**: Build parser first in `parsers/rust/`
- **No tests found**: Check directory structure
- **Compilation errors**: Update Rust toolchain

## Contributing

To add new test cases:
1. Read [TEST_FORMAT.md](TEST_FORMAT.md)
2. Create directory under `tests/cases/`
3. Add `input.sym` and `expected.json`/`error.json`
4. Run harness to verify

## License

Same as parent project (MIT).

## References

- **Main Documentation**: [README.md](README.md)
- **Quick Start**: [QUICKSTART.md](QUICKSTART.md)
- **Test Format**: [TEST_FORMAT.md](TEST_FORMAT.md)
- **Example Output**: [EXAMPLE_OUTPUT.md](EXAMPLE_OUTPUT.md)
- **SYM Parser**: `/mnt/c/Users/sebas/dev/symbolic-language/parsers/rust/`
- **Test Cases**: `/mnt/c/Users/sebas/dev/symbolic-language/tests/cases/`
