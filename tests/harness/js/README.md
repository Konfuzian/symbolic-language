# SYM JavaScript Test Harness

A comprehensive test harness for validating SYM format parsers written in JavaScript.

## Overview

This test harness automatically discovers and runs all test cases in `tests/cases/`, comparing parser output against expected results or validating error conditions.

## Requirements

- Node.js 14+ (uses ES modules)
- No external dependencies required

## Installation

```bash
cd tests/harness/js
npm install
```

## Usage

### Run all tests

```bash
npm test
```

or

```bash
node test-runner.js
```

### Run with verbose output

```bash
npm run test:verbose
```

or

```bash
node test-runner.js --verbose
```

Verbose mode shows:
- Detailed output for passing tests
- Expected vs actual JSON comparisons
- Full error messages and stack traces

## Test Case Structure

The harness reads test cases from `tests/cases/`. Each test case is a directory containing:

### Success Cases

For tests that should parse successfully:

- `input.sym` - The SYM format input file
- `expected.json` - The expected JSON output after parsing

Example:
```
tests/cases/basic/simple-string/
  ├── input.sym       # { :message Hello, world }
  └── expected.json   # { "message": "Hello, world" }
```

### Error Cases

For tests that should fail with an error:

- `input.sym` - The invalid SYM format input
- `error.json` - Error validation specification

Example `error.json`:
```json
{
  "type": "ParseError",
  "messagePattern": "(unclosed|unexpected end|missing closing)",
  "line": 3
}
```

Fields in `error.json`:
- `type` (optional) - Expected error type (e.g., "ParseError")
- `messagePattern` (optional) - Regex pattern the error message must match
- `line` (optional) - Expected line number where error occurred

## Parser Implementation

### Current Status

**⚠️ PARSER NOT IMPLEMENTED**

The current `parser-stub.js` is a minimal stub that demonstrates the expected interface but does not actually parse SYM files. You need to implement a full parser to run the tests successfully.

### Required Parser Interface

Your parser must export a `parse` function with this signature:

```javascript
/**
 * Parse SYM format input
 * @param {string} input - The SYM format string
 * @returns {any} - Parsed output (object, array, or primitive)
 * @throws {ParseError} - When parsing fails
 */
export function parse(input) {
  // Your implementation here
}

export class ParseError extends Error {
  constructor(message, line = null, column = null) {
    super(message);
    this.name = 'ParseError';
    this.line = line;
    this.column = column;
  }
}
```

### Implementation Guide

To implement a full parser, you need to handle:

#### 1. Basic Values

- **Strings**: Unquoted by default (e.g., `Hello, world`)
- **Numbers**: Integers (`42`, `-42`) and floats (`3.14`, `1e10`)
- **Booleans**: `true`, `false`
- **Null**: `null`
- **Symbols**: Prefixed with `:` (e.g., `:active`, `:admin`)

#### 2. Structures

- **Objects**: `{ :key value, :key2 value2 }`
  - Keys prefixed with `:`
  - Values follow the key on the same line
  - Commas at the start of new lines
  - Nested objects and arrays supported

- **Arrays**: `[ item1, item2, item3 ]`
  - Items separated by `,` at line start
  - Can contain any value type

#### 3. Special Features

- **Comments**:
  - Line comments: `// comment`
  - Block comments: `/* comment */`
  - Must be removed before parsing

- **Variables**:
  - Definition: `{ $var value }`
  - Reference: `$var`

- **Multiline Strings**:
  - Indented content after a key
  - Preserves whitespace and commas

- **Escaping**:
  - `\$` - Literal dollar sign
  - `\:` - Literal colon
  - `\42` - Force string interpretation

#### 4. Parsing Strategy

A basic implementation approach:

```javascript
export function parse(input) {
  // 1. Remove comments
  const withoutComments = removeComments(input);

  // 2. Tokenize
  const tokens = tokenize(withoutComments);

  // 3. Parse tokens into AST
  const ast = parseTokens(tokens);

  // 4. Evaluate (handle variables, symbols, etc.)
  return evaluate(ast);
}
```

### Reference Implementation

For a complete reference implementation, see:
- `parsers/rust/` - Full Rust parser implementation
- `spec/SPEC.md` - Complete language specification

### Testing Your Parser

1. Replace `parser-stub.js` with your implementation
2. Ensure it exports `parse` and `ParseError`
3. Run the test harness: `npm test`

The harness will:
- ✓ Parse all `input.sym` files
- ✓ Compare output with `expected.json`
- ✓ Validate errors match `error.json` patterns
- ✓ Report detailed results

## Output Format

### Summary View (Default)

```
SYM Test Harness

Found 21 test case(s)

Test Results
============================================================

basic
✓   empty-object
✓   simple-string
✓   multiple-keys

errors
✗   unclosed-object
    Expected error pattern not matched

============================================================
18/21 tests passed (85.7%)
3 test(s) failed
```

### Verbose View

Shows full details including:
- Expected vs actual JSON output
- Error messages and patterns
- Stack traces for failures

## Test Categories

Current test cases are organized by category:

- `basic/` - Simple objects, arrays, primitives
- `strings/` - String handling, multiline strings
- `numbers/` - Integer and float parsing
- `objects/` - Nested objects, complex structures
- `arrays/` - Array handling
- `symbols/` - Symbol type (`:symbolName`)
- `variables/` - Variable definition and references
- `comments/` - Comment handling
- `edge-cases/` - Boundary conditions
- `errors/` - Invalid input that should fail

## Development

### Adding New Test Cases

1. Create a directory under `tests/cases/category/test-name/`
2. Add `input.sym` with the test input
3. Add `expected.json` (for success) or `error.json` (for errors)
4. Run the harness

### Debugging

Use verbose mode to see detailed output:

```bash
node test-runner.js --verbose
```

This shows:
- Full expected and actual values
- Error messages and patterns
- Line numbers for errors

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

Use in CI/CD:

```bash
npm test || exit 1
```

## Contributing

When implementing the parser:

1. Start with basic values (strings, numbers, booleans, null)
2. Add object and array parsing
3. Implement comments
4. Add variables and symbols
5. Handle multiline strings
6. Add proper error reporting with line numbers

## License

MIT
