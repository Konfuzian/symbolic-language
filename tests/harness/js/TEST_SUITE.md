# SYM Test Suite Overview

Complete overview of the test suite for SYM parser implementations.

## Test Statistics

**Total Tests:** 54 test cases

### Tests by Category

| Category | Tests | Description |
|----------|-------|-------------|
| `arrays` | 4 | Array parsing and nested arrays |
| `basic` | 7 | Basic primitives and empty structures |
| `comments` | 3 | Line and block comment handling |
| `edge-cases` | 2 | Unicode and whitespace handling |
| `errors` | 22 | Invalid syntax and error detection |
| `numbers` | 3 | Number format parsing |
| `objects` | 2 | Object parsing and nesting |
| `strings` | 5 | String handling including multiline |
| `symbols` | 3 | Symbol type handling |
| `variables` | 3 | Variable definition and substitution |

## Test Case Structure

Each test case is a directory containing:

### Success Tests (32 tests)
- `input.sym` - SYM format input
- `expected.json` - Expected parsed output

### Error Tests (22 tests)
- `input.sym` - Invalid SYM input
- `error.json` - Expected error specification

## Test Categories in Detail

### arrays/ (4 tests)
- `array-of-objects` - Arrays containing object elements
- `mixed-array` - Arrays with different value types
- `nested-arrays` - Deeply nested array structures
- `simple-array` - Basic array with simple values

### basic/ (7 tests)
- `empty-array` - `[]`
- `empty-object` - `{}`
- `multiple-keys` - Objects with multiple key-value pairs
- `simple-boolean` - `true` and `false` values
- `simple-null` - `null` value
- `simple-number` - Integer and float values
- `simple-string` - Basic string values

### comments/ (3 tests)
- `block-comments` - `/* ... */` style comments
- `line-comments` - `// ...` style comments
- `mixed-comments` - Both types together

### edge-cases/ (2 tests)
- `unicode` - Unicode character handling
- `whitespace-handling` - Various whitespace scenarios

### errors/ (22 tests)
Various invalid syntax cases:

**Invalid Syntax (9 tests)**
- `invalid-syntax/defs-not-last-with-data-keys`
- `invalid-syntax/empty-key`
- `invalid-syntax/invalid-identifier`
- `invalid-syntax/invalid-modifier`
- `invalid-syntax/invalid-number-format`
- `invalid-syntax/missing-colon-in-key`
- `invalid-syntax/missing-dollar-in-var-def`
- `invalid-syntax/separator-without-newline`
- `invalid-syntax/unclosed-block-comment`

**Invalid Variables (4 tests)**
- `invalid-variable/duplicate-definition`
- `invalid-variable/undefined-in-array`
- `invalid-variable/undefined-in-nested-object`
- `invalid-variable/undefined-reference`

**Mismatched Braces (9 tests)**
- `mismatched-braces/extra-closing-brace`
- `mismatched-braces/mismatched-close`
- `mismatched-braces/unclosed-array`
- `mismatched-braces/unclosed-block-comment`
- `mismatched-braces/unclosed-object`
- And more...

### numbers/ (3 tests)
- `float-numbers` - Floating point values
- `integer-numbers` - Integer values
- `special-formats` - Hex, scientific notation, underscores

### objects/ (2 tests)
- `nested-objects` - Objects within objects
- `simple-object` - Basic object structure

### strings/ (5 tests)
- `multiline-string` - Indented multiline content
- `string-with-commas` - Strings containing commas
- `string-with-newlines` - Strings with embedded newlines
- `string-with-special-chars` - Special character handling
- More string edge cases...

### symbols/ (3 tests)
- `simple-symbol` - Basic symbol values (`:symbolName`)
- `symbol-vs-string` - Distinguishing symbols from strings
- `symbols-as-enums` - Using symbols like enums

### variables/ (3 tests)
- `multiple-variables` - Multiple variable definitions
- `simple-variable` - Basic variable definition and reference
- `variable-substitution` - Variable value substitution

## Error Test Specifications

Error tests include `error.json` files with:

```json
{
  "type": "ParseError",
  "messagePattern": "(expected|unexpected).*",
  "line": 3
}
```

**Fields:**
- `type` - Expected error class name (ParseError, SyntaxError, VariableError)
- `messagePattern` - Regex pattern the error message must match
- `line` - Expected line number where error occurred (optional)

## Running Specific Test Categories

The test harness runs all tests automatically, but you can filter by modifying the test runner or by examining specific test files:

```bash
# View a specific test case
cat ../../cases/basic/simple-string/input.sym
cat ../../cases/basic/simple-string/expected.json

# View an error test case
cat ../../cases/errors/mismatched-braces/unclosed-object/input.sym
cat ../../cases/errors/mismatched-braces/unclosed-object/error.json
```

## Test Coverage

The test suite covers:

✅ **Basic Types**
- Strings (unquoted)
- Numbers (integers, floats, special formats)
- Booleans (true, false)
- Null

✅ **Structures**
- Objects with single and multiple keys
- Arrays with various element types
- Nested objects and arrays

✅ **Special Features**
- Comments (line and block)
- Symbols (`:symbolName`)
- Variables (`$varName`)
- Multiline strings (indented content)

✅ **Error Cases**
- Syntax errors
- Unclosed braces/brackets
- Invalid identifiers
- Undefined variables
- Duplicate variables
- Invalid number formats

## Expected Behavior

### Success Cases
When parsing succeeds, the parser should return a JavaScript object/array/primitive that exactly matches the structure in `expected.json`.

### Error Cases
When parsing fails, the parser should:
1. Throw an error (or return an error object)
2. Error type should match `error.json` type field
3. Error message should match the regex in `messagePattern`
4. Error should report the correct line number (if specified)

## Test Quality

The test suite provides:
- **Comprehensive coverage** of all SYM features
- **Clear examples** for each feature
- **Edge case testing** for boundary conditions
- **Error validation** for robust error handling
- **Regression testing** to catch breaking changes

## Adding New Tests

To add a new test case:

1. Create a directory under the appropriate category:
   ```bash
   mkdir -p tests/cases/arrays/new-test-case
   ```

2. Add `input.sym`:
   ```sym
   [ item1
   , item2
   ]
   ```

3. Add `expected.json` (for success) or `error.json` (for error):
   ```json
   ["item1", "item2"]
   ```

4. Run the test harness:
   ```bash
   node test-runner.js
   ```

The new test will be automatically discovered and run.

## Continuous Integration

The test harness is designed for CI/CD:

```bash
# In CI script
cd tests/harness/js
npm test

# Exit code 0 = all tests pass
# Exit code 1 = one or more tests failed
```

Example GitHub Actions workflow:

```yaml
- name: Run JavaScript Parser Tests
  run: |
    cd tests/harness/js
    npm test
```

## Performance Notes

Running all 54 tests typically takes:
- **<1 second** with a simple stub (like the current one)
- **1-5 seconds** with a complete parser implementation
- **Depends on** parser complexity and optimization

The test harness itself is optimized for fast execution.

## Future Test Additions

Potential areas for additional test coverage:
- More complex variable scenarios
- Deeper nesting levels
- Larger files (performance testing)
- More Unicode edge cases
- Complex multiline string scenarios
- Mixed comment and code patterns

## Resources

- **Test cases**: `tests/cases/`
- **Spec**: `spec/SPEC.md`
- **Examples**: `examples/`
- **Rust parser**: `parsers/rust/` (reference implementation)
