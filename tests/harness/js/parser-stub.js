/**
 * SYM Parser Stub
 *
 * This is a minimal stub implementation that demonstrates the expected parser interface.
 * A full parser implementation is needed to actually parse .sym files.
 *
 * Expected interface:
 * - parse(input: string) => object | throws ParseError
 *
 * See the SPEC.md for full parsing requirements:
 * - Objects: { :key value, :key2 value2 }
 * - Arrays: [ item1, item2 ]
 * - Strings: unquoted by default
 * - Numbers: integers and floats
 * - Booleans: true, false
 * - Null: null
 * - Symbols: :symbolName
 * - Variables: $varName
 * - Comments: // line comments and /* block comments *\/
 * - Multiline strings: indented content
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
 * Stub parser - REPLACE WITH ACTUAL IMPLEMENTATION
 *
 * This stub will fail all tests and is only here to demonstrate
 * the expected interface.
 */
export function parse(input) {
  throw new ParseError(
    'Parser not implemented. This is a stub. See parser-stub.js for implementation requirements.',
    1,
    0
  );
}

/**
 * Example of what a basic parser might look like:
 *
 * export function parse(input) {
 *   const trimmed = input.trim();
 *
 *   // Handle empty input
 *   if (!trimmed) {
 *     return null;
 *   }
 *
 *   // Handle objects
 *   if (trimmed.startsWith('{') && trimmed.endsWith('}')) {
 *     return parseObject(trimmed);
 *   }
 *
 *   // Handle arrays
 *   if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
 *     return parseArray(trimmed);
 *   }
 *
 *   // Handle primitives
 *   return parsePrimitive(trimmed);
 * }
 *
 * function parseObject(input) {
 *   const result = {};
 *   // ... parse key-value pairs
 *   // Keys start with :
 *   // Values are separated by , at line start
 *   return result;
 * }
 *
 * function parseArray(input) {
 *   const result = [];
 *   // ... parse array items
 *   // Items separated by , at line start
 *   return result;
 * }
 *
 * function parsePrimitive(value) {
 *   // Check for null
 *   if (value === 'null') return null;
 *
 *   // Check for booleans
 *   if (value === 'true') return true;
 *   if (value === 'false') return false;
 *
 *   // Check for numbers
 *   const num = parseFloat(value);
 *   if (!isNaN(num)) return num;
 *
 *   // Default to string
 *   return value;
 * }
 */
