# Quick Start Guide

## Prerequisites

Make sure you have Rust installed:

```bash
# Check if Rust is installed
rustc --version
cargo --version

# If not installed, install from https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running the Test Harness

### Option 1: Using the run script (recommended)

```bash
./run.sh
```

### Option 2: Using cargo directly

```bash
cargo run --release
```

### Option 3: Build then run

```bash
# Build the harness
cargo build --release

# Run it
./target/release/test-harness
```

## Expected Output

```
SYM Parser Test Harness
============================================================

7 test cases found

[✓ PASS] basic/empty-array
[✓ PASS] basic/empty-object
[✓ PASS] basic/multiple-keys
[✓ PASS] basic/simple-boolean
[✓ PASS] basic/simple-null
[✓ PASS] basic/simple-number
[✓ PASS] basic/simple-string

============================================================
Test Summary
============================================================
Total:  7 tests
Passed: 7 tests
Failed: 0 tests
Pass rate: 100.0%

All tests passed! 🎉
```

## Understanding Test Results

### Passing Tests
- Green checkmark `✓ PASS`
- Test name shows which test case passed
- No additional output

### Failing Tests
- Red X `✗ FAIL`
- Shows detailed error message
- Displays expected vs actual output
- For error tests, shows expected error pattern vs actual error

### Example Failure Output

```
[✗ FAIL] basic/complex-nested
  Output mismatch:
  Expected: {
    "server": {
      "host": "localhost",
      "port": 8080
    }
  }
  Got: {
    "server": {
      "host": "localhost",
      "port": "8080"
    }
  }
```

## Test Categories

The harness automatically tests:

- **basic/** - Simple core features (empty objects, strings, numbers, booleans, null)
- **arrays/** - Array parsing and nested arrays
- **objects/** - Object parsing and nested objects
- **strings/** - String handling including multiline and escaping
- **numbers/** - Integer, float, hex, binary, octal numbers
- **symbols/** - Symbol syntax (`:symbol`)
- **variables/** - Variable definitions and references (`$var`)
- **comments/** - Comment handling (single-line, block)
- **edge-cases/** - Special cases and corner cases
- **errors/** - Invalid syntax that should produce errors

## Troubleshooting

### Build Errors

If you get compilation errors, ensure the parser is available:

```bash
# Build the parser first
cd ../../../parsers/rust
cargo build
cd ../../tests/harness/rust
cargo build
```

### No Tests Found

Make sure you're in the correct directory:

```bash
# From repository root
cd tests/harness/rust
cargo run
```

### Wrong Path Errors

The harness uses relative paths from its own location. Make sure the directory structure is:

```
repository/
├── parsers/
│   └── rust/           # SYM parser
└── tests/
    ├── cases/          # Test cases
    └── harness/
        └── rust/       # This test harness
```

## Next Steps

- See [README.md](README.md) for detailed documentation
- Add your own test cases under `tests/cases/`
- Run the harness to validate parser changes
- Integrate into CI/CD pipeline

## CI/CD Integration

Add to your CI workflow:

```yaml
- name: Run SYM test harness
  run: |
    cd tests/harness/rust
    cargo run --release
```

Or for GitHub Actions:

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run test harness
        run: cd tests/harness/rust && cargo run --release
```
