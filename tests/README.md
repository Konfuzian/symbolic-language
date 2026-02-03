# SYM Test Suite

A language-agnostic test suite for validating SYM format parser implementations.

## Purpose

This test suite provides a comprehensive collection of test cases to ensure your SYM parser correctly handles all language features, edge cases, and error conditions. The test cases are structured in a language-agnostic format, allowing parser implementers in any language to validate their implementation against a shared standard.

## Directory Structure

```
tests/
├── cases/              # Test case files organized by category
│   ├── basic/          # Fundamental parsing (strings, numbers, booleans, null, empty structures)
│   ├── strings/        # String-specific tests (multiline, escaping, inline commas)
│   ├── numbers/        # Number formats (int, float, hex, binary, octal, scientific, inf, nan)
│   ├── objects/        # Object structures (nesting, empty keys, complex structures)
│   ├── arrays/         # Array structures (mixed types, nested arrays)
│   ├── variables/      # Variable definitions and substitution ($var syntax)
│   ├── symbols/        # Symbol values (:symbol syntax)
│   ├── comments/       # Comment handling (line and block comments)
│   ├── edge-cases/     # Unusual but valid inputs (whitespace, blank lines, formatting)
│   └── errors/         # Invalid inputs that should produce errors
└── harness/            # Test harness implementations for different languages
    ├── rust/           # Rust test runner
    ├── js/             # JavaScript/Node.js test runner
    └── python/         # Python test runner
```

## Test Case Organization

### By Category

Each test case belongs to one of the following categories:

1. **basic/** - Core functionality tests
   - Simple strings, numbers, booleans, null
   - Empty objects and arrays
   - Multiple key-value pairs

2. **strings/** - String parsing tests
   - Inline commas (safe in SYM)
   - Multiline strings
   - Escaped strings
   - Leading/trailing whitespace handling

3. **numbers/** - Number format tests
   - Integers (decimal, hex `0xff`, binary `0b1010`, octal `0o755`)
   - Floats (decimal point, scientific notation)
   - Special values (`inf`, `-inf`, `nan`)
   - Underscore separators (`1_000_000`)

4. **objects/** - Object structure tests
   - Nested objects
   - Empty keys (default to empty string)
   - Complex nested structures
   - Field separators

5. **arrays/** - Array structure tests
   - Simple arrays
   - Nested arrays
   - Mixed type arrays
   - Arrays of objects

6. **variables/** - Variable tests
   - Variable definitions (`$var`)
   - Variable substitution
   - Multiple defs blocks
   - Variable override (`$var!`)

7. **symbols/** - Symbol tests
   - Symbol values (`:symbol`)
   - Symbols in objects and arrays
   - Distinguishing symbols from strings with colons

8. **comments/** - Comment handling tests
   - Line comments (`//`)
   - Block comments (`/* */`)
   - Inline comments (after whitespace)
   - Comments vs URLs (e.g., `https://` is not a comment)

9. **edge-cases/** - Unusual but valid inputs
   - Extra whitespace
   - Blank lines between fields
   - Mixed indentation
   - Minimal whitespace

10. **errors/** - Invalid inputs that should fail
    - Unclosed braces/brackets
    - Invalid syntax
    - Undefined variables
    - Malformed numbers
    - Invalid escape sequences

## Test Case Format

Each test case is a directory containing up to 4 files:

### File Structure

```
test-case-name/
├── input.sym          # Required: SYM input to parse
├── expected.json      # For valid inputs: expected JSON output
├── error.json         # For error tests: expected error details
└── meta.json          # Optional: test case metadata
```

### File Naming Conventions

#### 1. `input.sym` (Required)

The SYM format input file to be parsed.

**Example:**
```sym
{ :name Alice
, :age 28
, :active true
}
```

#### 2. `expected.json` (For valid inputs)

The expected JSON output after parsing. This file should contain the exact JSON representation of the parsed SYM data.

**Example:**
```json
{
  "name": "Alice",
  "age": 28,
  "active": true
}
```

**Important notes:**
- Strings remain strings
- Numbers become JSON numbers
- Booleans become JSON booleans
- Null becomes JSON null
- Symbols (`:foo`) become JSON strings with the symbol name (e.g., `"foo"` or implementation-specific representation)
- Variables are substituted before output

#### 3. `error.json` (For error tests)

For test cases in the `errors/` directory, this file describes the expected error.

**Format:**
```json
{
  "type": "ParseError",
  "messagePattern": "(regex pattern to match error message)",
  "line": 3
}
```

**Fields:**
- `type`: The error type/category (e.g., `"ParseError"`, `"UndefinedVariable"`, `"SyntaxError"`)
- `messagePattern`: A regex pattern that the error message should match (case-insensitive)
- `line`: (Optional) The line number where the error occurs

**Example:**
```json
{
  "type": "ParseError",
  "messagePattern": "(unclosed|unexpected end|missing closing|expected.*})",
  "line": 3
}
```

**Note:** Implementations may provide additional error details (column number, context, suggestions), but at minimum they should match the specified pattern on the expected line.

#### 4. `meta.json` (Optional)

Metadata describing the test case. This is primarily for documentation and is not used by test harnesses.

**Format:**
```json
{
  "description": "Human-readable description of what this test verifies"
}
```

**Example:**
```json
{
  "description": "Empty object - minimal object with no key-value pairs"
}
```

## How to Write New Test Cases

### 1. Choose the Right Category

Determine which category your test belongs to:
- Is it testing basic functionality? → `basic/`
- Is it a string edge case? → `strings/`
- Is it testing error handling? → `errors/`
- etc.

### 2. Create a Directory

Create a descriptive directory name using kebab-case:

```bash
mkdir -p tests/cases/strings/multiline-with-indentation
```

### 3. Create `input.sym`

Write the SYM input:

```bash
cat > tests/cases/strings/multiline-with-indentation/input.sym << 'EOF'
{ :poem
    Roses are red
    Violets are blue
    No comma-newline here
    So it's all one value
, :author Anonymous
}
EOF
```

### 4. Create Expected Output

For valid inputs, create `expected.json`:

```bash
cat > tests/cases/strings/multiline-with-indentation/expected.json << 'EOF'
{
  "poem": "Roses are red\nViolets are blue\nNo comma-newline here\nSo it's all one value",
  "author": "Anonymous"
}
EOF
```

For error inputs, create `error.json`:

```bash
cat > tests/cases/errors/unclosed-array/error.json << 'EOF'
{
  "type": "ParseError",
  "messagePattern": "(unclosed|unexpected end|missing closing|expected.*\\])",
  "line": 2
}
EOF
```

### 5. Add Metadata (Optional)

Create `meta.json` to document the test:

```bash
cat > tests/cases/strings/multiline-with-indentation/meta.json << 'EOF'
{
  "description": "Multiline strings with leading whitespace - should strip whitespace per line"
}
EOF
```

### 6. Test Your Test Case

Run your test harness to verify the test case works as expected.

## Running Tests

Test harnesses are provided in the `harness/` directory for different languages. Each harness automatically discovers and runs all test cases.

### Rust

```bash
cd tests/harness/rust
cargo build --release
cargo run --release
```

**Output:**
- Green: Passing tests
- Red: Failing tests
- Summary of results

### JavaScript/Node.js

```bash
cd tests/harness/js
npm install
npm test
```

### Python

```bash
cd tests/harness/python
pip install -r requirements.txt
python run_tests.py
```

### Creating Your Own Harness

To implement a test harness in your language:

1. **Discover test cases**: Recursively scan `tests/cases/`
2. **For each test case directory**:
   - Read `input.sym`
   - Parse it with your parser
   - If `expected.json` exists:
     - Compare parser output to expected JSON
     - Report success/failure
   - If `error.json` exists:
     - Verify parser throws an error
     - Check error type and message pattern match
     - Report success/failure
3. **Print results**: Summary of passed/failed tests

**Example structure:**

```
harness/
└── your-language/
    ├── README.md           # How to run the harness
    ├── run_tests.ext       # Main test runner script
    └── ...                 # Any dependencies/config files
```

## Expected Behavior

### Successful Parsing

A test case passes if:
1. The parser successfully parses `input.sym`
2. The output exactly matches `expected.json` (after JSON normalization)

### Error Handling

An error test case passes if:
1. The parser throws/returns an error
2. The error type matches (if specified in `error.json`)
3. The error message matches the regex pattern (case-insensitive)
4. The error occurs on the expected line (if specified)

### Symbol Representation

Parsers may represent symbols in different ways:
- As strings: `":active"` or `"active"`
- As special symbol objects: `{"type": "symbol", "value": "active"}`
- As enum/keyword types in the host language

**Test harnesses should normalize symbol representation when comparing outputs.**

### Variable Substitution

Variables (`$var`) should be substituted before output:
- Input: `{ $name Alice } { :user $name }`
- Output: `{ "user": "Alice" }`

The defs block itself should not appear in the output.

## Common Pitfalls

### 1. Inline Commas Are Literal

```sym
{ :address 123 Main St, Apt 4, New York, NY 10001 }
```

This is ONE value: `"123 Main St, Apt 4, New York, NY 10001"`

Only `\n,` (newline + comma) separates elements.

### 2. Leading Whitespace in Multiline Strings

```sym
{ :code
    def hello():
        print("hi")
}
```

Leading whitespace is stripped per line. To preserve it, use `\`:

```sym
{ :code
    def hello():
\       print("hi")
}
```

### 3. Symbols vs Strings with Colons

```sym
{ :url https://example.com    # string "https://example.com"
, :status :active              # symbol :active
}
```

`:foo` is only a symbol at the **start** of a value.

### 4. Variables Only in Data Block

```sym
{ $env prod }              # defs block (only $-keys, not last block)
{ :deployment $env }       # data block (can use variables)
```

Variables are substituted, and defs blocks are not included in output.

## Contributing Test Cases

When adding new test cases:

1. Ensure the test case is **minimal** - tests one specific feature or edge case
2. Use **descriptive directory names**
3. Add a `meta.json` with a clear description
4. Verify the test case works with at least one existing parser
5. Consider both positive (valid input) and negative (error) test cases

## Test Coverage Goals

The test suite aims to cover:

- All value types (string, int, float, boolean, null, symbol)
- All number formats (decimal, hex, binary, octal, scientific)
- All string escaping rules
- Nested structures (depth > 3)
- Empty structures
- Multiline strings with various whitespace patterns
- Comment placement (line start, inline, block)
- Variable definition and substitution
- Variable overrides (`$var!`)
- Field separators (with blank lines, extra whitespace)
- Symbols vs strings with colons
- All documented error conditions

## License

This test suite is part of the SYM format specification and is released under the MIT license.
