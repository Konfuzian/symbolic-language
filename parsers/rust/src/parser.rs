//! Parser implementation for SYM format

use crate::{Result, SymError, Value};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

/// Key modifier for merge behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyModifier {
    /// Default: deep merge
    Merge,
    /// Replace entirely (!)
    Replace,
    /// Append to array (+)
    Append,
}

/// Parser for SYM format
pub struct Parser<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
    pos: usize,
    variables: HashMap<String, Value>,
    imports: Vec<String>,
}

impl<'a> Parser<'a> {
    /// Create a new parser
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            line: 1,
            col: 1,
            pos: 0,
            variables: HashMap::new(),
            imports: Vec::new(),
        }
    }

    /// Create a parser with pre-defined variables
    pub fn with_vars(input: &'a str, vars: HashMap<String, Value>) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            line: 1,
            col: 1,
            pos: 0,
            variables: vars,
            imports: Vec::new(),
        }
    }

    /// Parse the input and return the final value
    pub fn parse(&mut self) -> Result<Value> {
        // Parse imports
        self.skip_whitespace_and_comments();
        while self.check_import() {
            let path = self.parse_import()?;
            self.imports.push(path);
            self.skip_whitespace_and_comments();
        }

        // Parse defs blocks and data
        let mut blocks: Vec<(bool, Value)> = Vec::new(); // (is_defs, value)

        loop {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }

            let value = self.parse_value()?;

            // Check if this block is a defs block (all keys start with $)
            let is_defs = if let Value::Object(ref obj) = value {
                !obj.is_empty() && obj.keys().all(|k| k.starts_with('$'))
            } else {
                false
            };

            blocks.push((is_defs, value));
        }

        if blocks.is_empty() {
            return Err(self.error("Empty document"));
        }

        // Process blocks: all but last can be defs, last is always data
        let (data_block, defs_blocks) = if blocks.len() == 1 {
            (blocks.pop().unwrap().1, vec![])
        } else {
            let data = blocks.pop().unwrap().1;
            (data, blocks)
        };

        // Process defs blocks - add variables to scope
        for (is_defs, block) in defs_blocks {
            if is_defs {
                if let Value::Object(obj) = block {
                    for (key, value) in obj {
                        // Remove $ prefix from key
                        let var_name = key.trim_start_matches('$');
                        let (var_name, is_override) = if var_name.ends_with('!') {
                            (var_name.trim_end_matches('!').to_string(), true)
                        } else {
                            (var_name.to_string(), false)
                        };

                        if self.variables.contains_key(&var_name) && !is_override {
                            return Err(SymError::DuplicateVariable(var_name));
                        }

                        self.variables.insert(var_name, value);
                    }
                }
            }
        }

        // Substitute variables in data block
        let result = self.substitute_variables(data_block)?;

        Ok(result)
    }

    /// Check if we're at an @import directive
    fn check_import(&mut self) -> bool {
        let remaining = &self.input[self.pos..];
        remaining.starts_with("@import")
    }

    /// Parse an @import directive
    fn parse_import(&mut self) -> Result<String> {
        // Consume "@import"
        for _ in 0..7 {
            self.advance();
        }

        self.skip_horizontal_whitespace();

        // Parse path until end of line
        let mut path = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            path.push(self.advance().unwrap());
        }

        Ok(path.trim().to_string())
    }

    /// Parse a value
    fn parse_value(&mut self) -> Result<Value> {
        self.skip_whitespace_and_comments();

        let ch = match self.chars.peek() {
            Some(&c) => c,
            None => return Err(self.error("Unexpected end of input")),
        };

        match ch {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            ':' => self.parse_symbol_or_key(),
            '$' => self.parse_variable_ref(),
            '\\' => self.parse_escaped_value(),
            _ => self.parse_literal_or_string(),
        }
    }

    /// Parse an object { :key value, ... }
    fn parse_object(&mut self) -> Result<Value> {
        self.expect('{')?;
        self.skip_whitespace_and_comments();

        let mut map = HashMap::new();

        // Empty object
        if self.check('}') {
            self.advance();
            return Ok(Value::Object(map));
        }

        // First field (no leading comma)
        let (key, modifier, value) = self.parse_field()?;
        self.apply_field(&mut map, key, modifier, value)?;

        // Remaining fields (with separator)
        loop {
            self.skip_whitespace_and_comments();

            if self.check('}') {
                self.advance();
                break;
            }

            // Expect separator: newline then comma
            if !self.check_separator() {
                if self.is_at_end() {
                    return Err(self.error("Unclosed object"));
                }
                // Maybe it's the next field on a new line without comma (error)
                return Err(self.error("Expected ',' separator"));
            }

            self.consume_separator();
            self.skip_whitespace_and_comments();

            if self.check('}') {
                self.advance();
                break;
            }

            let (key, modifier, value) = self.parse_field()?;
            self.apply_field(&mut map, key, modifier, value)?;
        }

        Ok(Value::Object(map))
    }

    /// Apply a field to the object, handling modifiers
    fn apply_field(
        &self,
        map: &mut HashMap<String, Value>,
        key: String,
        modifier: KeyModifier,
        value: Value,
    ) -> Result<()> {
        match modifier {
            KeyModifier::Merge => {
                if let Some(existing) = map.get_mut(&key) {
                    existing.deep_merge(value);
                } else {
                    map.insert(key, value);
                }
            }
            KeyModifier::Replace => {
                map.insert(key, value);
            }
            KeyModifier::Append => {
                if let Some(existing) = map.get_mut(&key) {
                    match (existing, value) {
                        (Value::Array(arr), Value::Array(new_items)) => {
                            arr.extend(new_items);
                        }
                        _ => return Err(SymError::AppendToNonArray(key)),
                    }
                } else {
                    map.insert(key, value);
                }
            }
        }
        Ok(())
    }

    /// Parse a single field: :key value or $key value
    fn parse_field(&mut self) -> Result<(String, KeyModifier, Value)> {
        self.skip_whitespace_and_comments();

        let ch = self.chars.peek().copied();

        match ch {
            Some(':') => {
                // Object key
                self.advance(); // consume ':'
                let key = self.parse_identifier()?;

                // Check for modifier
                let modifier = self.parse_key_modifier();

                self.skip_horizontal_whitespace();

                // Parse value (may be empty = empty string)
                let value = if self.check_value_start() {
                    self.parse_value()?
                } else {
                    Value::String(String::new())
                };

                Ok((key, modifier, value))
            }
            Some('$') => {
                // Variable definition
                self.advance(); // consume '$'
                let key = self.parse_identifier()?;

                // Check for override modifier
                let modifier = if self.check('!') {
                    self.advance();
                    KeyModifier::Replace
                } else {
                    KeyModifier::Merge
                };

                self.skip_horizontal_whitespace();

                let value = if self.check_value_start() {
                    self.parse_value()?
                } else {
                    Value::String(String::new())
                };

                // Store with $ prefix so we know it's a var def
                Ok((format!("${}", key), modifier, value))
            }
            _ => Err(self.error("Expected ':' or '$' at start of field")),
        }
    }

    /// Parse key modifier (! or +)
    fn parse_key_modifier(&mut self) -> KeyModifier {
        match self.chars.peek() {
            Some('!') => {
                self.advance();
                KeyModifier::Replace
            }
            Some('+') => {
                self.advance();
                KeyModifier::Append
            }
            _ => KeyModifier::Merge,
        }
    }

    /// Parse an array [ value, ... ]
    fn parse_array(&mut self) -> Result<Value> {
        self.expect('[')?;
        self.skip_whitespace_and_comments();

        let mut arr = Vec::new();

        // Empty array
        if self.check(']') {
            self.advance();
            return Ok(Value::Array(arr));
        }

        // First element (no leading comma)
        let value = self.parse_value()?;
        arr.push(value);

        // Remaining elements (with separator)
        loop {
            self.skip_whitespace_and_comments();

            if self.check(']') {
                self.advance();
                break;
            }

            if !self.check_separator() {
                if self.is_at_end() {
                    return Err(self.error("Unclosed array"));
                }
                return Err(self.error("Expected ',' separator"));
            }

            self.consume_separator();
            self.skip_whitespace_and_comments();

            if self.check(']') {
                self.advance();
                break;
            }

            let value = self.parse_value()?;
            arr.push(value);
        }

        Ok(Value::Array(arr))
    }

    /// Parse a symbol :name (in value position)
    fn parse_symbol_or_key(&mut self) -> Result<Value> {
        self.advance(); // consume ':'
        let name = self.parse_identifier()?;
        Ok(Value::Symbol(name))
    }

    /// Parse a variable reference $name
    fn parse_variable_ref(&mut self) -> Result<Value> {
        self.advance(); // consume '$'
        let name = self.parse_identifier()?;
        // Return as a placeholder - will be substituted later
        Ok(Value::String(format!("${}", name)))
    }

    /// Parse an escaped value \something
    fn parse_escaped_value(&mut self) -> Result<Value> {
        self.advance(); // consume '\'

        // The rest is a literal string
        let content = self.parse_string_content()?;
        Ok(Value::String(content))
    }

    /// Parse a literal (number, bool, null) or string
    fn parse_literal_or_string(&mut self) -> Result<Value> {
        let start_pos = self.pos;

        // Peek at what we have
        let first_line = self.peek_to_end_of_line();

        // Try to parse as number, bool, or null
        if let Some(value) = self.try_parse_number(&first_line) {
            // Advance past the number
            let num_str = self.extract_number_str(&first_line);
            for _ in 0..num_str.len() {
                self.advance();
            }
            return Ok(value);
        }

        // Check for boolean/null keywords (only at value start)
        if first_line.starts_with("true") && !self.is_identifier_char_at(&first_line, 4) {
            for _ in 0..4 {
                self.advance();
            }
            return Ok(Value::Bool(true));
        }
        if first_line.starts_with("false") && !self.is_identifier_char_at(&first_line, 5) {
            for _ in 0..5 {
                self.advance();
            }
            return Ok(Value::Bool(false));
        }
        if first_line.starts_with("null") && !self.is_identifier_char_at(&first_line, 4) {
            for _ in 0..4 {
                self.advance();
            }
            return Ok(Value::Null);
        }
        if first_line.starts_with("inf") && !self.is_identifier_char_at(&first_line, 3) {
            for _ in 0..3 {
                self.advance();
            }
            return Ok(Value::Float(f64::INFINITY));
        }
        if first_line.starts_with("-inf") && !self.is_identifier_char_at(&first_line, 4) {
            for _ in 0..4 {
                self.advance();
            }
            return Ok(Value::Float(f64::NEG_INFINITY));
        }
        if first_line.starts_with("nan") && !self.is_identifier_char_at(&first_line, 3) {
            for _ in 0..3 {
                self.advance();
            }
            return Ok(Value::Float(f64::NAN));
        }

        // It's a string - parse multiline string content
        let content = self.parse_string_content()?;
        Ok(Value::String(content))
    }

    /// Check if character at position is an identifier char
    fn is_identifier_char_at(&self, s: &str, pos: usize) -> bool {
        s.chars()
            .nth(pos)
            .map(|c| c.is_alphanumeric() || c == '_' || c == '-')
            .unwrap_or(false)
    }

    /// Parse string content (potentially multiline)
    fn parse_string_content(&mut self) -> Result<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
        let mut preserve_whitespace = false;

        loop {
            match self.chars.peek().copied() {
                None => break,
                Some('}') | Some(']') => break,
                Some('\n') => {
                    // End of line - save it
                    let trimmed = if preserve_whitespace {
                        current_line.trim_end().to_string()
                    } else {
                        current_line.trim().to_string()
                    };

                    if !trimmed.is_empty() || !lines.is_empty() {
                        lines.push(trimmed);
                    }

                    self.advance(); // consume newline
                    current_line = String::new();
                    preserve_whitespace = false;

                    // Check for separator (newline + whitespace + comma)
                    self.skip_horizontal_whitespace();
                    if self.check(',') {
                        // This is a separator - don't consume it, break out
                        break;
                    }

                    // Check for start of line comment
                    if self.check_line_comment_start() {
                        self.skip_line_comment();
                        continue;
                    }

                    // Check for block comment
                    if self.check_block_comment_start() {
                        self.skip_block_comment();
                        continue;
                    }
                }
                Some('\r') => {
                    self.advance();
                    // Handle \r\n
                }
                Some('\\') => {
                    self.advance();
                    // Check what follows
                    match self.chars.peek().copied() {
                        Some(' ') | Some('\t') => {
                            // Preserve whitespace for this line
                            preserve_whitespace = true;
                            current_line.push(self.advance().unwrap());
                        }
                        Some('\\') => {
                            current_line.push(self.advance().unwrap());
                        }
                        Some(',') => {
                            // Escaped comma - literal comma
                            current_line.push(self.advance().unwrap());
                        }
                        Some(c) => {
                            // Other escape - include the character
                            current_line.push(self.advance().unwrap());
                        }
                        None => {
                            current_line.push('\\');
                        }
                    }
                }
                Some(ch) => {
                    // Check for inline comment (whitespace + //)
                    if ch == ' ' || ch == '\t' {
                        let remaining = &self.input[self.pos..];
                        if let Some(comment_pos) = remaining.find("//") {
                            // Check if there's only whitespace before //
                            let before = &remaining[..comment_pos];
                            if before.chars().all(|c| c == ' ' || c == '\t') {
                                // This is an inline comment - skip to end of line
                                current_line.push_str(before);
                                for _ in 0..comment_pos {
                                    self.advance();
                                }
                                self.skip_line_comment();
                                continue;
                            }
                        }
                    }
                    current_line.push(self.advance().unwrap());
                }
            }
        }

        // Handle last line
        let trimmed = if preserve_whitespace {
            current_line.trim_end().to_string()
        } else {
            current_line.trim().to_string()
        };
        if !trimmed.is_empty() {
            lines.push(trimmed);
        }

        Ok(lines.join("\n"))
    }

    /// Try to parse a number from the start of a string
    fn try_parse_number(&self, s: &str) -> Option<Value> {
        let s = s.trim_start();

        // Hex
        if s.starts_with("0x") || s.starts_with("0X") {
            let num_part: String = s[2..]
                .chars()
                .take_while(|c| c.is_ascii_hexdigit() || *c == '_')
                .collect();
            if num_part.is_empty() {
                return None;
            }
            let clean: String = num_part.chars().filter(|c| *c != '_').collect();
            return i64::from_str_radix(&clean, 16).ok().map(Value::Int);
        }

        // Binary
        if s.starts_with("0b") || s.starts_with("0B") {
            let num_part: String = s[2..]
                .chars()
                .take_while(|c| *c == '0' || *c == '1' || *c == '_')
                .collect();
            if num_part.is_empty() {
                return None;
            }
            let clean: String = num_part.chars().filter(|c| *c != '_').collect();
            return i64::from_str_radix(&clean, 2).ok().map(Value::Int);
        }

        // Octal
        if s.starts_with("0o") || s.starts_with("0O") {
            let num_part: String = s[2..]
                .chars()
                .take_while(|c| c.is_digit(8) || *c == '_')
                .collect();
            if num_part.is_empty() {
                return None;
            }
            let clean: String = num_part.chars().filter(|c| *c != '_').collect();
            return i64::from_str_radix(&clean, 8).ok().map(Value::Int);
        }

        // Decimal (int or float)
        let mut chars = s.chars().peekable();
        let mut num_str = String::new();
        let mut is_float = false;

        // Optional negative sign
        if chars.peek() == Some(&'-') {
            num_str.push(chars.next().unwrap());
        }

        // Integer part
        let mut has_digits = false;
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                has_digits = true;
                num_str.push(chars.next().unwrap());
            } else if c == '_' && has_digits {
                chars.next();
            } else {
                break;
            }
        }

        if !has_digits {
            return None;
        }

        // Decimal part
        if chars.peek() == Some(&'.') {
            // Check it's followed by a digit
            let mut chars_clone = chars.clone();
            chars_clone.next();
            if chars_clone.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                num_str.push(chars.next().unwrap()); // consume '.'
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num_str.push(chars.next().unwrap());
                    } else if c == '_' {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
        }

        // Exponent part
        if chars.peek() == Some(&'e') || chars.peek() == Some(&'E') {
            is_float = true;
            num_str.push(chars.next().unwrap());
            if chars.peek() == Some(&'+') || chars.peek() == Some(&'-') {
                num_str.push(chars.next().unwrap());
            }
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
        }

        // Make sure the next char isn't an identifier char
        if let Some(&c) = chars.peek() {
            if c.is_alphabetic() || c == '_' {
                return None;
            }
        }

        // Parse
        let clean: String = num_str.chars().filter(|c| *c != '_').collect();
        if is_float {
            clean.parse::<f64>().ok().map(Value::Float)
        } else {
            clean.parse::<i64>().ok().map(Value::Int)
        }
    }

    /// Extract the number string from input
    fn extract_number_str(&self, s: &str) -> String {
        let s = s.trim_start();
        let mut result = String::new();

        // Handle prefix
        if s.starts_with("0x")
            || s.starts_with("0X")
            || s.starts_with("0b")
            || s.starts_with("0B")
            || s.starts_with("0o")
            || s.starts_with("0O")
        {
            result.push_str(&s[..2]);
            let rest = &s[2..];
            for c in rest.chars() {
                if c.is_alphanumeric() || c == '_' {
                    result.push(c);
                } else {
                    break;
                }
            }
            return result;
        }

        // Decimal number
        let mut chars = s.chars().peekable();

        // Sign
        if chars.peek() == Some(&'-') {
            result.push(chars.next().unwrap());
        }

        // Digits, dot, exponent
        let mut in_exponent = false;
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() || c == '_' {
                result.push(chars.next().unwrap());
            } else if c == '.' && !in_exponent {
                result.push(chars.next().unwrap());
            } else if (c == 'e' || c == 'E') && !in_exponent {
                in_exponent = true;
                result.push(chars.next().unwrap());
                if chars.peek() == Some(&'+') || chars.peek() == Some(&'-') {
                    result.push(chars.next().unwrap());
                }
            } else {
                break;
            }
        }

        result
    }

    /// Parse an identifier
    fn parse_identifier(&mut self) -> Result<String> {
        let mut name = String::new();

        // First char must be letter or underscore
        match self.chars.peek() {
            Some(&c) if c.is_alphabetic() || c == '_' => {
                name.push(self.advance().unwrap());
            }
            _ => return Err(self.error("Expected identifier")),
        }

        // Rest can include digits and hyphens
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                name.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        Ok(name)
    }

    /// Substitute variables in a value
    fn substitute_variables(&self, value: Value) -> Result<Value> {
        match value {
            Value::String(s) if s.starts_with('$') => {
                let var_name = &s[1..];
                self.variables
                    .get(var_name)
                    .cloned()
                    .ok_or_else(|| SymError::UndefinedVariable(var_name.to_string()))
            }
            Value::Array(arr) => {
                let new_arr: Result<Vec<Value>> =
                    arr.into_iter().map(|v| self.substitute_variables(v)).collect();
                Ok(Value::Array(new_arr?))
            }
            Value::Object(obj) => {
                let new_obj: Result<HashMap<String, Value>> = obj
                    .into_iter()
                    .map(|(k, v)| Ok((k, self.substitute_variables(v)?)))
                    .collect();
                Ok(Value::Object(new_obj?))
            }
            other => Ok(other),
        }
    }

    // Helper methods

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn check(&mut self, expected: char) -> bool {
        self.chars.peek() == Some(&expected)
    }

    fn expect(&mut self, expected: char) -> Result<()> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected '{}'", expected)))
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn peek_to_end_of_line(&self) -> String {
        self.input[self.pos..]
            .lines()
            .next()
            .unwrap_or("")
            .to_string()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();

            if self.check_line_comment_start() {
                self.skip_line_comment();
            } else if self.check_block_comment_start() {
                self.skip_block_comment();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_horizontal_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c == ' ' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn check_line_comment_start(&mut self) -> bool {
        let remaining = &self.input[self.pos..];
        remaining.starts_with("//")
    }

    fn skip_line_comment(&mut self) {
        // Consume //
        self.advance();
        self.advance();

        // Skip to end of line
        while let Some(&c) = self.chars.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn check_block_comment_start(&mut self) -> bool {
        let remaining = &self.input[self.pos..];
        remaining.starts_with("/*")
    }

    fn skip_block_comment(&mut self) {
        // Consume /*
        self.advance();
        self.advance();

        // Find */
        loop {
            match self.advance() {
                Some('*') => {
                    if self.check('/') {
                        self.advance();
                        break;
                    }
                }
                None => break,
                _ => {}
            }
        }
    }

    fn check_separator(&mut self) -> bool {
        // Look back to see if we've had a newline, then look for comma
        let remaining = &self.input[self.pos..];

        // We should be positioned at or before a comma after newline(s)
        for (i, c) in remaining.char_indices() {
            if c == ',' {
                return true;
            } else if !c.is_whitespace() {
                return false;
            }
        }
        false
    }

    fn consume_separator(&mut self) {
        // Skip whitespace until we hit comma
        while let Some(&c) = self.chars.peek() {
            if c == ',' {
                self.advance();
                break;
            } else if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn check_value_start(&mut self) -> bool {
        match self.chars.peek() {
            Some(&c) => !matches!(c, '\n' | '\r' | ',' | '}' | ']'),
            None => false,
        }
    }

    fn error(&self, message: &str) -> SymError {
        SymError::ParseError {
            line: self.line,
            col: self.col,
            message: message.to_string(),
        }
    }
}
