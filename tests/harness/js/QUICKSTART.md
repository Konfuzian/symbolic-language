# Quick Start Guide

Get up and running with the SYM test harness in 5 minutes.

## Step 1: Verify Setup

```bash
cd tests/harness/js
node test-runner.js
```

You should see all tests failing with "Parser not implemented" - this is expected!

## Step 2: Understand the Test Cases

Look at a simple test case:

```bash
cat ../../cases/basic/simple-string/input.sym
# Output: { :message Hello, world }

cat ../../cases/basic/simple-string/expected.json
# Output: { "message": "Hello, world" }
```

The test harness will:
1. Read `input.sym`
2. Pass it to your parser
3. Compare the output with `expected.json`

## Step 3: Implement Your Parser

### Option A: Start from Scratch

Replace the contents of `parser-stub.js` with your implementation:

```javascript
export class ParseError extends Error {
  constructor(message, line = null, column = null) {
    super(message);
    this.name = 'ParseError';
    this.line = line;
    this.column = column;
  }
}

export function parse(input) {
  // Your parser implementation here
  // Must return parsed JSON-like structure
  // or throw ParseError on invalid input
}
```

### Option B: Use the Example as a Starting Point

```bash
# Copy the example implementation
cp parser-example.js parser.js

# Update parser-stub.js to use it
```

Edit `parser-stub.js`:
```javascript
export { parse, ParseError } from './parser.js';
```

**Note:** The example parser only handles basic cases and will fail most tests. Use it as a reference for parsing structure.

## Step 4: Run Tests

```bash
# Run all tests
npm test

# Run with detailed output
npm run test:verbose
```

## Step 5: Iterate

1. Pick a failing test category (start with `basic/`)
2. Look at the test case files
3. Implement the feature
4. Run tests again
5. Repeat!

### Suggested Implementation Order

1. **Basic values** (primitives: strings, numbers, booleans, null)
   - Tests: `basic/simple-string`, `basic/simple-number`, etc.

2. **Empty structures**
   - Tests: `basic/empty-object`, `basic/empty-array`

3. **Simple objects**
   - Tests: `basic/multiple-keys`

4. **Simple arrays**
   - Tests: `arrays/simple-array`, `arrays/mixed-types`

5. **Nested structures**
   - Tests: `objects/nested-objects`, `arrays/nested-arrays`

6. **Comments**
   - Tests: `comments/line-comments`, `comments/block-comments`

7. **Advanced features**
   - Symbols: `symbols/simple-symbol`
   - Variables: `variables/simple-variable`
   - Multiline strings: `strings/multiline-string`

8. **Error handling**
   - Tests: `errors/mismatched-braces`, etc.

## Understanding Test Output

### Default Output

```
SYM Test Harness

Found 54 test case(s)

Test Results
============================================================

basic
✓   empty-object
✗   simple-string
    Output does not match expected

============================================================
1/54 tests passed (1.9%)
```

### Verbose Output

```bash
node test-runner.js --verbose
```

Shows:
- Expected JSON
- Actual output from your parser
- Full error messages
- Stack traces

## Example: Implementing Basic Objects

Here's a minimal example to get the first test passing:

```javascript
export class ParseError extends Error {
  constructor(message, line = null) {
    super(message);
    this.name = 'ParseError';
    this.line = line;
  }
}

export function parse(input) {
  const trimmed = input.trim();

  // Handle empty object
  if (trimmed === '{}') {
    return {};
  }

  throw new ParseError('Not implemented yet');
}
```

Run tests:
```bash
node test-runner.js
```

Result:
```
basic
✓   empty-object
✗   empty-array
✗   multiple-keys
...
```

Success! One test passing. Now implement the next feature.

## Key SYM Parsing Concepts

### 1. Keys are prefixed with `:`
```sym
{ :name John }
```
→
```json
{ "name": "John" }
```

### 2. Commas go at LINE START
```sym
{ :name John
, :age 30
}
```

### 3. Values are unquoted
```sym
{ :city New York }
```
→
```json
{ "city": "New York" }
```

### 4. Types are detected automatically
```sym
{ :age 30
, :score 95.5
, :active true
, :data null
}
```
→
```json
{
  "age": 30,
  "score": 95.5,
  "active": true,
  "data": null
}
```

## Resources

- **Full Spec**: `spec/SPEC.md` in the repo root
- **Reference Implementation**: `parsers/rust/` directory
- **More Examples**: `examples/` directory
- **Test Cases**: `tests/cases/` directory

## Getting Help

1. Check `README.md` for detailed parser interface requirements
2. Look at `parser-example.js` for a basic implementation structure
3. Review the test cases in `tests/cases/` for concrete examples
4. Read `spec/SPEC.md` for the complete language specification

## Tips

- Start simple - get basic tests passing first
- Use verbose mode to debug: `node test-runner.js --verbose`
- Look at the Rust implementation for reference
- Test frequently as you add features
- Focus on one test category at a time

Good luck! 🚀
