# Implementation Notes

Technical notes for implementing a SYM parser to pass the test harness.

## Parser Requirements

### Interface

Your parser must export:

```javascript
export class ParseError extends Error {
  constructor(message, line = null, column = null) {
    super(message);
    this.name = 'ParseError';
    this.line = line;
    this.column = column;
  }
}

export function parse(input: string): any
```

The `parse` function should:
- Accept a string containing SYM format data
- Return a JavaScript object/array/primitive representing the parsed data
- Throw `ParseError` (or subclass) on invalid input with line/column info

### Error Types

Based on the test cases, you may want to implement specialized error types:

```javascript
export class SyntaxError extends ParseError {
  constructor(message, line, column) {
    super(message, line, column);
    this.name = 'SyntaxError';
  }
}

export class VariableError extends ParseError {
  constructor(message, line, column) {
    super(message, line, column);
    this.name = 'VariableError';
  }
}
```

Error types expected by tests:
- `ParseError` - General parsing errors
- `SyntaxError` - Invalid syntax errors
- `VariableError` - Undefined or duplicate variables

## Key Parsing Challenges

### 1. Tokenization

SYM requires context-aware tokenization:

```sym
{ :url https://example.com  // This is a comment
, :message Hello, world      // Commas in values are OK
}
```

Challenges:
- `://` should NOT be treated as a comment
- Inline commas in values are preserved
- Only `\n,` (newline + comma) separates elements
- Comments can appear inline after whitespace

### 2. Comma Handling

The comma separator is **positional**:

```sym
{ :a value1
, :b value2
}
```

- Commas must appear at the start of a line (after leading whitespace)
- Commas within values are not separators
- Example: `:address 123 Main St, Apt 4` - the comma is part of the value

### 3. String Tokenization

Strings are unquoted and extend to:
- End of line
- Start of nested structure (`{` or `[`)
- Start of comment (`//` after whitespace)

Example:
```sym
{ :name John Doe
, :city New York, NY
}
```

Both `John Doe` and `New York, NY` are complete strings.

### 4. Multiline Strings

Indented content creates multiline strings:

```sym
{ :description
    This is line 1.
    This is line 2.
, :other value
}
```

The value of `description` is:
```
This is line 1.
This is line 2.
```

Rules:
- Content after the key on same line is trimmed
- Subsequent indented lines are collected
- Relative indentation is preserved
- Ends when a less-indented line appears

### 5. Type Detection

Values are automatically typed:

```javascript
function detectType(value) {
  // Null
  if (value === 'null') return null;

  // Boolean
  if (value === 'true') return true;
  if (value === 'false') return false;

  // Symbol (starts with :)
  if (value.startsWith(':')) {
    return { type: 'symbol', value: value.slice(1) };
  }

  // Number (integer or float)
  // Support formats: 42, -42, 3.14, 1e10, 0xff, 1_000_000
  if (isNumber(value)) {
    return parseNumber(value);
  }

  // Default to string
  return value;
}
```

### 6. Symbols

Symbols should be represented distinctly from strings:

```sym
{ :status :active
, :role :admin
}
```

Output (one approach):
```json
{
  "status": ":active",
  "role": ":admin"
}
```

Or as a special type:
```json
{
  "status": { "__sym_type": "symbol", "value": "active" },
  "role": { "__sym_type": "symbol", "value": "admin" }
}
```

Check test expectations to determine the correct representation.

### 7. Variables

Variables require a two-pass approach:

**Pass 1: Extract definitions**
```sym
{ $base-url https://api.example.com
}
```

**Pass 2: Substitute references**
```sym
{ :endpoint $base-url/users
}
```

Result:
```json
{
  "endpoint": "https://api.example.com/users"
}
```

Variable rules:
- Variables start with `$`
- Definitions typically in a separate object
- References are replaced with their values
- Undefined references should throw `VariableError`
- Duplicate definitions should throw `VariableError`

### 8. Comments

Two types:
```sym
// Line comment (to end of line)

/* Block comment
   can span multiple lines */
```

Edge cases:
- `https://example.com` - NOT a comment
- URLs and other patterns with `//` after non-whitespace
- Comments can appear inline: `{ :key value  // comment`
- Block comments can be nested (optional, check spec)

### 9. Escape Sequences

Force literal interpretation:

```sym
{ :number \42      // String "42", not number
, :price \$100     // String "$100", not variable
, :key \:value     // String ":value", not symbol
}
```

## Parsing Strategy

### Recommended Approach

```javascript
export function parse(input) {
  // 1. Preprocess: Remove comments, preserve line numbers
  const preprocessed = removeComments(input);

  // 2. Tokenize: Break into meaningful tokens
  const tokens = tokenize(preprocessed);

  // 3. Parse: Build AST
  const ast = parseTokens(tokens);

  // 4. Evaluate: Handle variables, type detection
  const result = evaluate(ast);

  return result;
}
```

### Alternative: Single-Pass Parser

```javascript
export function parse(input) {
  let pos = 0;
  let line = 1;
  let col = 1;

  function peek() { /* ... */ }
  function advance() { /* ... */ }
  function skipWhitespace() { /* ... */ }
  function parseValue() { /* ... */ }
  function parseObject() { /* ... */ }
  function parseArray() { /* ... */ }

  return parseValue();
}
```

## Testing Strategy

### Test Categories by Difficulty

1. **Easy** - Basic primitives
   - `basic/empty-object`
   - `basic/empty-array`
   - `basic/simple-string`
   - `basic/simple-number`
   - `basic/simple-boolean`
   - `basic/simple-null`

2. **Medium** - Structures
   - `basic/multiple-keys`
   - `objects/nested-objects`
   - `arrays/simple-array`
   - `arrays/mixed-array`

3. **Hard** - Advanced features
   - `comments/*`
   - `strings/multiline-string`
   - `symbols/*`
   - `variables/*`

4. **Expert** - Edge cases & errors
   - `edge-cases/*`
   - `errors/*`

### Debugging Tips

1. **Use verbose mode**
   ```bash
   node test-runner.js --verbose
   ```

2. **Test individual categories**
   - Modify test-runner.js to filter by category
   - Or create a simple test script:
   ```javascript
   import { parse } from './parser-stub.js';
   const result = parse('{ :name Alice }');
   console.log(result);
   ```

3. **Compare with reference implementation**
   ```bash
   cd ../../../parsers/rust
   cargo build --release
   ./target/release/sym-parser ../../tests/cases/basic/simple-string/input.sym
   ```

4. **Add debug logging**
   ```javascript
   function tokenize(input) {
     console.log('Tokenizing:', input);
     // ...
   }
   ```

## Common Pitfalls

### 1. Comment Detection
❌ Wrong: `if (char === '/' && next === '/')`
✅ Right: Check that `//` appears after whitespace or at line start

### 2. Comma Separation
❌ Wrong: Split on any comma
✅ Right: Only commas at line start are separators

### 3. String Boundaries
❌ Wrong: Read until whitespace
✅ Right: Read until newline, `{`, `[`, or inline comment

### 4. Type Coercion
❌ Wrong: `parseInt(value)` (fails on `0xff`, `1e10`)
✅ Right: Proper number parsing with all formats

### 5. Line Tracking
❌ Wrong: Lose line numbers during preprocessing
✅ Right: Maintain line/column through entire pipeline for error reporting

## Performance Considerations

For the test harness, performance is not critical. However:

- **Avoid regex in loops** - Tokenize once, iterate efficiently
- **Use string builders** - For multiline strings, use arrays and join
- **Cache parsed results** - If re-parsing during variable resolution

## Reference Implementation

The Rust parser is a complete reference:

```bash
cd parsers/rust/src
cat parser.rs  # Main parsing logic
cat lexer.rs   # Tokenization
cat ast.rs     # AST structure
```

Key files to study:
- `parser.rs` - Overall structure
- `lexer.rs` - Tokenization with line tracking
- `tests/` - Unit tests for specific features

## Additional Resources

- **SYM Spec**: `spec/SPEC.md`
- **Test Cases**: `tests/cases/`
- **Examples**: `examples/`
- **Rust Parser**: `parsers/rust/`

## Next Steps

1. Review `parser-example.js` for basic structure
2. Start with empty structures (`{}`, `[]`)
3. Add basic values (strings, numbers, booleans, null)
4. Implement objects with multiple keys
5. Add arrays
6. Implement comments
7. Add advanced features (symbols, variables, multiline strings)
8. Handle all error cases

Good luck! 🚀
