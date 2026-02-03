# Example Test Output

This document shows example output from the test harness in various scenarios.

## Successful Test Run

When all tests pass:

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

**Exit code:** `0`

## Failed Test Run

When some tests fail:

```
SYM Parser Test Harness
============================================================

10 test cases found

[✓ PASS] basic/empty-array
[✓ PASS] basic/empty-object
[✗ FAIL] basic/nested-object
  Output mismatch:
  Expected: {
    "server": {
      "host": "localhost",
      "port": 8080
    }
  }
  Got:      {
    "server": {
      "host": "localhost",
      "port": "8080"
    }
  }
[✓ PASS] basic/simple-boolean
[✓ PASS] basic/simple-null
[✗ FAIL] numbers/hex-values
  Parser error: Parse error at line 1, column 12: Invalid hex digit
[✓ PASS] basic/simple-string
[✓ PASS] arrays/simple-array
[✓ PASS] strings/multiline
[✓ PASS] symbols/simple-symbol

============================================================
Test Summary
============================================================
Total:  10 tests
Passed: 8 tests
Failed: 2 tests
Pass rate: 80.0%

Failed tests:
  • basic/nested-object
    Output mismatch:
    Expected: {...}
    Got:      {...}
  • numbers/hex-values
    Parser error: Parse error at line 1, column 12: Invalid hex digit
```

**Exit code:** `1`

## Error Test Success

When an error test correctly catches an expected error:

```
[✓ PASS] errors/unclosed-object
```

This means:
- The input was invalid (as expected)
- The parser produced an error (as expected)
- The error message matched the expected pattern

## Error Test Failure

When an error test fails (parser should have failed but didn't):

```
[✗ FAIL] errors/unclosed-object
  Expected parser error, but parsing succeeded
```

This means the parser accepted invalid input that should have been rejected.

## Error Test Failure (Wrong Error)

When the error doesn't match the expected pattern:

```
[✗ FAIL] errors/invalid-key
  Error message mismatch:
  Expected pattern: expected key starting with :
  Got: Parse error at line 1, column 3: unexpected token
```

This means:
- The parser correctly rejected the input
- But the error message was different than expected

## Output with Many Tests

With dozens of tests:

```
SYM Parser Test Harness
============================================================

47 test cases found

[✓ PASS] arrays/empty-array
[✓ PASS] arrays/nested-arrays
[✓ PASS] arrays/simple-array
[✓ PASS] arrays/mixed-types
[✓ PASS] basic/empty-object
[✓ PASS] basic/multiple-keys
[✓ PASS] basic/simple-boolean
[✓ PASS] basic/simple-null
[✓ PASS] basic/simple-number
[✓ PASS] basic/simple-string
[✓ PASS] comments/block-comment
[✓ PASS] comments/inline-comment
[✓ PASS] comments/multiple-comments
[✓ PASS] edge-cases/empty-string
[✓ PASS] edge-cases/unicode
[✓ PASS] edge-cases/url-not-comment
[✓ PASS] numbers/binary
[✓ PASS] numbers/float
[✓ PASS] numbers/hex
[✓ PASS] numbers/octal
[✓ PASS] numbers/scientific
[✓ PASS] objects/deep-nesting
[✓ PASS] objects/empty-object
[✓ PASS] objects/nested-object
[✓ PASS] strings/empty-string
[✓ PASS] strings/escaped-strings
[✓ PASS] strings/multiline-string
[✓ PASS] strings/string-with-commas
[✓ PASS] strings/url-in-string
[✓ PASS] symbols/multiple-symbols
[✓ PASS] symbols/simple-symbol
[✓ PASS] symbols/symbol-as-value
[✓ PASS] variables/reference-before-definition
[✓ PASS] variables/simple-variable
[✓ PASS] variables/variable-override

============================================================
Test Summary
============================================================
Total:  35 tests
Passed: 35 tests
Failed: 0 tests
Pass rate: 100.0%

All tests passed! 🎉
```

## Colorized Output

In a terminal with color support, the output is colorized:

- **Test harness title**: Cyan, bold
- **Separators (====)**: Cyan
- **✓ PASS**: Green
- **✗ FAIL**: Red
- **Passed count**: Green
- **Failed count**: Red
- **Error messages**: Dimmed (gray)
- **Success message**: Green, bold

## No Tests Found

When no test cases are discovered:

```
SYM Parser Test Harness
============================================================

No test cases found!
```

This usually means:
- You're running from the wrong directory
- The `tests/cases/` directory is empty
- Test cases are missing required files (`input.sym` + `expected.json`/`error.json`)

## Directory Not Found Error

When the test cases directory doesn't exist:

```
Error: Test cases directory not found at "/path/to/tests/cases"
```

**Exit code:** `1`

This means the harness can't find the test cases directory. Check your current directory and the repository structure.

## Parser Compilation Error

If the parser can't be built:

```
error: failed to compile `sym-parser`
```

This is a compilation error, not a test failure. Fix the parser code or dependencies before running tests.

## Understanding Test Names

Test names follow the pattern: `category/test-name`

Examples:
- `basic/empty-object` → Basic category, empty object test
- `numbers/hex` → Numbers category, hexadecimal test
- `errors/unclosed-object` → Errors category, unclosed object test
- `strings/multiline-string` → Strings category, multiline string test

The category and test name correspond to the directory structure:
```
tests/cases/
└── category/
    └── test-name/
        ├── input.sym
        └── expected.json
```

## Running Individual Test Categories

Currently, the harness runs all tests. To test specific categories, you could filter the test cases directory:

```bash
# Copy specific category to temporary location
mkdir -p /tmp/test-subset/
cp -r tests/cases/basic /tmp/test-subset/

# Modify harness to point to /tmp/test-subset/
# Or use directory filtering (feature to be added)
```

## Performance Information

The harness doesn't currently show timing information, but typical performance:

- **Single test**: < 1ms
- **10 tests**: < 10ms
- **100 tests**: < 100ms
- **Startup overhead**: ~50ms (loading parser, discovering tests)

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed, or harness error

This follows standard Unix conventions and works well with CI/CD pipelines.
