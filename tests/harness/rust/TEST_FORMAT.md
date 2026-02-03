# Test Case Format Guide

This document describes the format for SYM parser test cases.

## Directory Structure

Each test case is a directory containing specific files:

```
tests/cases/
└── category/
    └── test-name/
        ├── input.sym       # Required: Input to parse
        ├── expected.json   # For success tests
        ├── error.json      # For error tests (mutually exclusive with expected.json)
        └── meta.json       # Optional: Test metadata
```

## File Descriptions

### `input.sym` (Required)

The SYM format input that will be parsed by the test harness.

**Example:**
```sym
{ :name Alice
, :age 28
, :active true
}
```

### `expected.json` (For Success Tests)

The expected JSON output after parsing `input.sym`. The test passes if the parser output matches this JSON exactly.

**Example:**
```json
{
  "name": "Alice",
  "age": 28,
  "active": true
}
```

### `error.json` (For Error Tests)

Specifies the expected error when parsing `input.sym`. The test passes if parsing fails with an error matching the specified pattern.

**Format:**
```json
{
  "pattern": "expected }"
}
```

Or:
```json
{
  "message": "Parse error at line 1"
}
```

The test will check if the actual error message contains the specified `pattern` or `message`.

### `meta.json` (Optional)

Metadata about the test case for documentation purposes. Not used by the test harness but helpful for developers.

**Example:**
```json
{
  "description": "Tests parsing of an empty object",
  "category": "basic",
  "difficulty": "trivial"
}
```

## Test Types

### Success Tests

Tests that should parse successfully.

**Requirements:**
- `input.sym` - Input to parse
- `expected.json` - Expected output

**Example:** `tests/cases/basic/simple-string/`

```
simple-string/
├── input.sym         -> { :message Hello, world }
├── expected.json     -> { "message": "Hello, world" }
└── meta.json         -> { "description": "Simple string value" }
```

### Error Tests

Tests that should fail to parse.

**Requirements:**
- `input.sym` - Invalid input
- `error.json` - Expected error pattern

**Example:** `tests/cases/errors/unclosed-object/`

```
unclosed-object/
├── input.sym         -> { :key value
├── error.json        -> { "pattern": "expected }" }
└── meta.json         -> { "description": "Object missing closing brace" }
```

## JSON Representation Rules

The test harness converts SYM values to JSON for comparison:

### Basic Types

| SYM Type | JSON Type | Example |
|----------|-----------|---------|
| `null` | `null` | `null` |
| `true` / `false` | `boolean` | `true` |
| `42` | `number` | `42` |
| `3.14` | `number` | `3.14` |
| `Hello` | `string` | `"Hello"` |
| `:symbol` | `string` | `":symbol"` |

### Symbols

Symbols in SYM (`:symbol`) are represented as strings with the `:` prefix in JSON:

**SYM:**
```sym
{ :status :active }
```

**JSON:**
```json
{
  "status": ":active"
}
```

### Arrays

**SYM:**
```sym
[ one, two, three ]
```

**JSON:**
```json
["one", "two", "three"]
```

### Objects

**SYM:**
```sym
{ :name Alice
, :age 28
}
```

**JSON:**
```json
{
  "name": "Alice",
  "age": 28
}
```

### Nested Structures

**SYM:**
```sym
{ :server
  { :host localhost
  , :port 8080
  }
}
```

**JSON:**
```json
{
  "server": {
    "host": "localhost",
    "port": 8080
  }
}
```

## Special Cases

### Empty Values

**Empty Object:**
```sym
{}
```
```json
{}
```

**Empty Array:**
```sym
[]
```
```json
[]
```

### Multiline Strings

SYM preserves multiline strings:

**SYM:**
```sym
{ :poem
    Roses are red
    Violets are blue
}
```

**JSON:**
```json
{
  "poem": "Roses are red\nViolets are blue"
}
```

### Numbers

**SYM:**
```sym
{ :decimal 42
, :hex 0xFF
, :binary 0b1010
, :octal 0o755
, :float 3.14
, :scientific 1e10
}
```

**JSON:**
```json
{
  "decimal": 42,
  "hex": 255,
  "binary": 10,
  "octal": 493,
  "float": 3.14,
  "scientific": 10000000000.0
}
```

### Special Float Values

**SYM:**
```sym
{ :infinity inf
, :negative -inf
, :not_a_number nan
}
```

**JSON:**
```json
{
  "infinity": "inf",
  "negative": "-inf",
  "not_a_number": "nan"
}
```

Note: Special float values are represented as strings in JSON since JSON doesn't support `Infinity` or `NaN`.

## Creating New Test Cases

### Step 1: Choose a Category

Place your test in an appropriate category:
- `basic/` - Core functionality
- `arrays/` - Array features
- `objects/` - Object features
- `strings/` - String handling
- `numbers/` - Number formats
- `symbols/` - Symbol syntax
- `variables/` - Variable features
- `comments/` - Comment handling
- `edge-cases/` - Corner cases
- `errors/` - Error handling

### Step 2: Create Directory

```bash
mkdir -p tests/cases/category/test-name
```

### Step 3: Add Input File

Create `input.sym` with your test input:

```bash
cat > tests/cases/category/test-name/input.sym << 'EOF'
{ :example test
}
EOF
```

### Step 4: Add Expected Output

For success tests, create `expected.json`:

```bash
cat > tests/cases/category/test-name/expected.json << 'EOF'
{
  "example": "test"
}
EOF
```

For error tests, create `error.json`:

```bash
cat > tests/cases/category/test-name/error.json << 'EOF'
{
  "pattern": "expected closing brace"
}
EOF
```

### Step 5: Add Metadata (Optional)

```bash
cat > tests/cases/category/test-name/meta.json << 'EOF'
{
  "description": "Brief description of what this tests"
}
EOF
```

### Step 6: Run Tests

```bash
cd tests/harness/rust
cargo run
```

Your test will be automatically discovered and executed!

## Common Patterns

### Testing Edge Cases

```
edge-cases/empty-string/
├── input.sym         -> { :value }
├── expected.json     -> { "value": "" }
└── meta.json         -> { "description": "Empty string value" }
```

### Testing Error Messages

```
errors/invalid-key/
├── input.sym         -> { 123 value }
├── error.json        -> { "pattern": "expected key" }
└── meta.json         -> { "description": "Keys must start with :" }
```

### Testing Complex Nesting

```
objects/deep-nesting/
├── input.sym         -> { :a { :b { :c { :d value } } } }
├── expected.json     -> { "a": { "b": { "c": { "d": "value" } } } }
└── meta.json         -> { "description": "Four levels of nesting" }
```

## Debugging Failed Tests

When a test fails, the harness shows:

```
[✗ FAIL] category/test-name
  Output mismatch:
  Expected: {...}
  Got:      {...}
```

**Common Issues:**

1. **Type Mismatch**: Check if numbers/strings are confused
2. **Symbol Format**: Symbols should be `":symbol"` in JSON
3. **Whitespace**: Multiline strings preserve newlines
4. **Ordering**: Object keys may appear in any order (comparison handles this)
5. **Float Precision**: Small floating point differences are tolerated

## Best Practices

1. **One Feature Per Test**: Each test should focus on one feature or edge case
2. **Clear Naming**: Use descriptive directory names like `multiline-string-with-indentation`
3. **Add Metadata**: Include descriptions to help other developers understand the test
4. **Test Boundaries**: Test edge cases, not just happy paths
5. **Minimal Examples**: Keep tests as simple as possible while still testing the feature
6. **Document Expectations**: Use meta.json to explain what behavior is being tested

## Examples Repository

See `tests/cases/` for many examples of well-structured test cases covering various features of the SYM format.
