/**
 * SYM Parser Example Implementation
 *
 * This is a simplified example showing how to start implementing a SYM parser.
 * This is NOT a complete implementation - it only handles very basic cases
 * to demonstrate the parsing approach.
 *
 * To use this example:
 * 1. Copy this file to parser.js
 * 2. Update parser-stub.js to import from parser.js instead
 * 3. Expand the implementation to handle all SYM features
 */

export class ParseError extends Error {
  constructor(message, line = null, column = null) {
    super(message);
    this.name = 'ParseError';
    this.line = line;
    this.column = column;
  }
}

/**
 * Simple tokenizer - breaks input into meaningful chunks
 */
function tokenize(input) {
  const tokens = [];
  let i = 0;
  let line = 1;
  let col = 1;

  while (i < input.length) {
    const char = input[i];

    // Track line numbers
    if (char === '\n') {
      line++;
      col = 1;
      i++;
      continue;
    }

    // Skip whitespace
    if (/\s/.test(char)) {
      col++;
      i++;
      continue;
    }

    // Braces and brackets
    if (char === '{' || char === '}' || char === '[' || char === ']') {
      tokens.push({ type: char, value: char, line, col });
      i++;
      col++;
      continue;
    }

    // Comma separator (important: only at line start in SYM)
    if (char === ',') {
      tokens.push({ type: 'COMMA', value: ',', line, col });
      i++;
      col++;
      continue;
    }

    // Keys (start with :)
    if (char === ':') {
      let value = ':';
      i++;
      col++;
      while (i < input.length && /[a-zA-Z0-9_-]/.test(input[i])) {
        value += input[i];
        i++;
        col++;
      }
      tokens.push({ type: 'KEY', value: value.slice(1), line, col });
      continue;
    }

    // Values (everything else until whitespace or structural char)
    let value = '';
    const startCol = col;
    while (
      i < input.length &&
      !(/\s/.test(input[i])) &&
      input[i] !== '{' &&
      input[i] !== '}' &&
      input[i] !== '[' &&
      input[i] !== ']' &&
      input[i] !== ','
    ) {
      value += input[i];
      i++;
      col++;
    }
    if (value) {
      tokens.push({ type: 'VALUE', value, line, col: startCol });
    }
  }

  return tokens;
}

/**
 * Parse a value - convert to appropriate type
 */
function parseValue(value) {
  // Null
  if (value === 'null') return null;

  // Boolean
  if (value === 'true') return true;
  if (value === 'false') return false;

  // Number
  const num = Number(value);
  if (!isNaN(num) && value.match(/^-?\d+(\.\d+)?$/)) {
    return num;
  }

  // Default to string
  return value;
}

/**
 * Simplified parser - handles only basic cases
 */
export function parse(input) {
  // Remove comments (simplified - doesn't handle all cases)
  input = input.replace(/\/\/[^\n]*/g, '');
  input = input.replace(/\/\*[\s\S]*?\*\//g, '');

  const tokens = tokenize(input);
  let pos = 0;

  function current() {
    return tokens[pos];
  }

  function advance() {
    pos++;
  }

  function expect(type) {
    const token = current();
    if (!token || token.type !== type) {
      throw new ParseError(
        `Expected ${type} but got ${token ? token.type : 'EOF'}`,
        token?.line,
        token?.col
      );
    }
    advance();
    return token;
  }

  function parseObject() {
    expect('{');
    const obj = {};

    while (current() && current().type !== '}') {
      // Skip commas
      if (current().type === 'COMMA') {
        advance();
        continue;
      }

      // Parse key
      if (current().type !== 'KEY') {
        throw new ParseError(
          `Expected key in object, got ${current().type}`,
          current().line,
          current().col
        );
      }
      const key = current().value;
      advance();

      // Parse value
      let value = '';
      if (current() && current().type === 'VALUE') {
        value = parseValue(current().value);
        advance();
      } else if (current() && (current().type === '{' || current().type === '[')) {
        value = parseStructure();
      }

      obj[key] = value;
    }

    expect('}');
    return obj;
  }

  function parseArray() {
    expect('[');
    const arr = [];

    while (current() && current().type !== ']') {
      // Skip commas
      if (current().type === 'COMMA') {
        advance();
        continue;
      }

      // Parse value
      if (current().type === 'VALUE') {
        arr.push(parseValue(current().value));
        advance();
      } else if (current().type === '{' || current().type === '[') {
        arr.push(parseStructure());
      }
    }

    expect(']');
    return arr;
  }

  function parseStructure() {
    const token = current();
    if (!token) {
      throw new ParseError('Unexpected end of input');
    }

    if (token.type === '{') {
      return parseObject();
    } else if (token.type === '[') {
      return parseArray();
    } else if (token.type === 'VALUE') {
      const val = parseValue(token.value);
      advance();
      return val;
    } else {
      throw new ParseError(`Unexpected token: ${token.type}`, token.line, token.col);
    }
  }

  // Handle empty input
  if (tokens.length === 0) {
    return null;
  }

  // Parse the structure
  const result = parseStructure();

  // Ensure we consumed all tokens
  if (current()) {
    throw new ParseError(
      `Unexpected token after parsing: ${current().type}`,
      current().line,
      current().col
    );
  }

  return result;
}

/**
 * LIMITATIONS OF THIS EXAMPLE:
 *
 * This simplified parser does NOT handle:
 * - Multiline strings (indented content)
 * - Symbols (should preserve : prefix, e.g., :active as a symbol type)
 * - Variables ($var definition and references)
 * - Proper comment handling (edge cases)
 * - Escaped characters (\$, \:, etc.)
 * - Complex number formats (hex, scientific notation, underscores)
 * - Strings with commas (need proper tokenization)
 * - Proper error messages with context
 * - Many edge cases
 *
 * For a complete implementation, see:
 * - spec/SPEC.md for full language specification
 * - parsers/rust/ for reference implementation
 */
