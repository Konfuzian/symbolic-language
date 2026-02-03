# Quick Reference Card

## Run Tests

```bash
cd tests/harness/rust
./run.sh              # Easiest method
cargo run --release   # Direct cargo
cargo run             # Debug mode
```

## File Structure

```
tests/harness/rust/
├── src/main.rs          # Main implementation (372 lines)
├── Cargo.toml           # Project config
├── run.sh               # Run script
├── README.md            # Full docs
├── QUICKSTART.md        # Getting started
├── TEST_FORMAT.md       # Test case format
├── EXAMPLE_OUTPUT.md    # Output examples
├── OVERVIEW.md          # High-level overview
└── QUICK_REFERENCE.md   # This file
```

## Test Case Format

### Success Test
```
tests/cases/category/test-name/
├── input.sym         # Input to parse
├── expected.json     # Expected output
└── meta.json         # Optional description
```

### Error Test
```
tests/cases/category/test-name/
├── input.sym         # Invalid input
├── error.json        # Expected error pattern
└── meta.json         # Optional description
```

## Example Test

**input.sym:**
```sym
{ :name Alice
, :age 28
}
```

**expected.json:**
```json
{
  "name": "Alice",
  "age": 28
}
```

## Output Interpretation

```
[✓ PASS] test-name          → Test passed
[✗ FAIL] test-name          → Test failed
  Error message here        → Why it failed
```

## Key Features

- ✓ Auto-discovers tests
- ✓ Compares parser output with expected JSON
- ✓ Validates error cases
- ✓ Colorized output
- ✓ Pass/fail summary
- ✓ CI/CD ready (exit codes)

## Commands

```bash
# Build
cargo build --release

# Run tests
cargo run --release

# Build and run
./run.sh

# Clean build
cargo clean
```

## Exit Codes

- `0` - All tests passed
- `1` - Tests failed or error

## Dependencies

- `sym-parser` - Parser being tested
- `serde_json` - JSON comparison
- `colored` - Terminal colors
- `walkdir` - Directory traversal

## Adding Tests

1. Create directory: `tests/cases/category/test-name/`
2. Add `input.sym`
3. Add `expected.json` or `error.json`
4. Run harness - auto-discovered!

## Documentation

| File | Purpose |
|------|---------|
| README.md | Full documentation |
| QUICKSTART.md | Getting started |
| TEST_FORMAT.md | How to write tests |
| EXAMPLE_OUTPUT.md | Example outputs |
| OVERVIEW.md | Architecture overview |
| QUICK_REFERENCE.md | This cheat sheet |

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Rust not found | Install from https://rustup.rs/ |
| Compilation error | Build parser: `cd parsers/rust && cargo build` |
| No tests found | Check you're in correct directory |
| Tests failing | Check test format in TEST_FORMAT.md |

## JSON Representation

| SYM | JSON |
|-----|------|
| `null` | `null` |
| `true` | `true` |
| `42` | `42` |
| `3.14` | `3.14` |
| `hello` | `"hello"` |
| `:symbol` | `":symbol"` |
| `[a, b]` | `["a", "b"]` |
| `{:k v}` | `{"k": "v"}` |

## CI/CD Integration

```yaml
# GitHub Actions
- name: Run SYM tests
  run: cd tests/harness/rust && cargo run --release
```

```yaml
# GitLab CI
test:
  script:
    - cd tests/harness/rust
    - cargo run --release
```

## Statistics

- **Implementation**: 372 lines
- **Documentation**: 1,403 lines
- **Total**: 1,775 lines
- **Test cases**: 54 (32 success, 22 error)

## Quick Links

- Parser: `/mnt/c/Users/sebas/dev/symbolic-language/parsers/rust/`
- Tests: `/mnt/c/Users/sebas/dev/symbolic-language/tests/cases/`
- Harness: `/mnt/c/Users/sebas/dev/symbolic-language/tests/harness/rust/`
