/**
 * SYM Syntax Highlighter
 * 
 * A React component for syntax highlighting SYM format code.
 */

// Color scheme
const colors = {
  comment: '#6a9955',      // green
  variable: '#c586c0',     // purple  
  key: '#9cdcfe',          // light blue
  symbol: '#4ec9b0',       // teal
  number: '#b5cea8',       // light green
  boolean: '#569cd6',      // blue
  null: '#569cd6',         // blue
  bracket: '#ffd700',      // gold
  escape: '#d7ba7d',       // tan
  string: '#ce9178',       // orange
  default: '#d4d4d4',      // light gray
};

/**
 * Tokenize SYM code
 */
function tokenize(code) {
  const tokens = [];
  let i = 0;
  let atValueStart = false;
  
  while (i < code.length) {
    // Block comments
    if (code.slice(i, i + 2) === '/*') {
      const end = code.indexOf('*/', i + 2);
      const endIdx = end === -1 ? code.length : end + 2;
      tokens.push({ type: 'comment', value: code.slice(i, endIdx) });
      i = endIdx;
      continue;
    }
    
    // Line comments (must be at start of line or after whitespace)
    if (code.slice(i, i + 2) === '//') {
      const prevChar = i > 0 ? code[i - 1] : '\n';
      if (prevChar === '\n' || prevChar === ' ' || prevChar === '\t' || i === 0) {
        const end = code.indexOf('\n', i);
        const endIdx = end === -1 ? code.length : end;
        tokens.push({ type: 'comment', value: code.slice(i, endIdx) });
        i = endIdx;
        continue;
      }
    }
    
    // Newlines
    if (code[i] === '\n') {
      tokens.push({ type: 'newline', value: '\n' });
      i++;
      atValueStart = false;
      continue;
    }
    
    // Brackets
    if ('{['.includes(code[i])) {
      tokens.push({ type: 'bracket', value: code[i] });
      i++;
      atValueStart = true;
      continue;
    }
    
    if ('}]'.includes(code[i])) {
      tokens.push({ type: 'bracket', value: code[i] });
      i++;
      atValueStart = false;
      continue;
    }
    
    // Comma (separator)
    if (code[i] === ',') {
      tokens.push({ type: 'default', value: ',' });
      i++;
      atValueStart = true;
      continue;
    }
    
    // Variable definition or reference: $name
    if (code[i] === '$') {
      let j = i + 1;
      while (j < code.length && /[a-zA-Z0-9_-]/.test(code[j])) j++;
      // Check for ! modifier
      if (code[j] === '!') j++;
      tokens.push({ type: 'variable', value: code.slice(i, j) });
      i = j;
      atValueStart = false;
      continue;
    }
    
    // Key or symbol: :name
    if (code[i] === ':') {
      let j = i + 1;
      while (j < code.length && /[a-zA-Z0-9_-]/.test(code[j])) j++;
      // Check for ! or + modifier
      if (code[j] === '!' || code[j] === '+') j++;
      const value = code.slice(i, j);
      
      // Determine if key or symbol based on context
      // Keys come after { or , at start of line; symbols are values
      const lastNonWs = tokens.filter(t => t.type !== 'newline' && t.value.trim()).pop();
      const isKey = !lastNonWs || 
                    lastNonWs.value === '{' || 
                    lastNonWs.value === ',';
      
      tokens.push({ type: isKey ? 'key' : 'symbol', value });
      i = j;
      atValueStart = !isKey;
      continue;
    }
    
    // Escape sequences
    if (code[i] === '\\') {
      let j = i + 1;
      if (j < code.length) {
        // Include the escaped character
        j++;
        // If it's escaping a word, include the whole word
        while (j < code.length && /[a-zA-Z0-9_-]/.test(code[j])) j++;
      }
      tokens.push({ type: 'escape', value: code.slice(i, j) });
      i = j;
      atValueStart = false;
      continue;
    }
    
    // Whitespace
    if (' \t'.includes(code[i])) {
      let j = i;
      while (j < code.length && ' \t'.includes(code[j])) j++;
      tokens.push({ type: 'default', value: code.slice(i, j) });
      i = j;
      continue;
    }
    
    // Numbers, booleans, null, or strings
    let j = i;
    while (j < code.length && !' \t\n{}[],:'.includes(code[j])) j++;
    const word = code.slice(i, j);
    
    if (word === 'true' || word === 'false') {
      tokens.push({ type: 'boolean', value: word });
    } else if (word === 'null') {
      tokens.push({ type: 'null', value: word });
    } else if (word === 'inf' || word === '-inf' || word === 'nan') {
      tokens.push({ type: 'number', value: word });
    } else if (/^-?(\d[\d_]*\.?[\d_]*([eE][+-]?\d+)?|0x[\da-fA-F_]+|0b[01_]+|0o[0-7_]+)$/.test(word)) {
      tokens.push({ type: 'number', value: word });
    } else {
      tokens.push({ type: 'string', value: word });
    }
    
    i = j;
    atValueStart = false;
  }
  
  return tokens;
}

/**
 * SYM Syntax Highlighter Component
 */
export function SymHighlight({ code, style = {} }) {
  const tokens = tokenize(code);
  
  return (
    <pre style={{
      backgroundColor: '#1e1e1e',
      padding: '16px',
      borderRadius: '8px',
      overflow: 'auto',
      fontFamily: 'Fira Code, Monaco, Consolas, monospace',
      fontSize: '14px',
      lineHeight: '1.5',
      ...style
    }}>
      <code>
        {tokens.map((token, i) => (
          <span key={i} style={{ color: colors[token.type] || colors.default }}>
            {token.value}
          </span>
        ))}
      </code>
    </pre>
  );
}

/**
 * Example usage / demo component
 */
export function SymHighlightDemo() {
  const exampleCode = `/*
 * SYM Format Example
 */

// Variable definitions
{ $app myapp
, $version 1.0.0
, $debug true
}

// Data block
{ :meta
  { :name $app
  , :version $version
  , :description
      A sample application
      with multiline strings.
  }

, :server
  { :host localhost
  , :port 8080
  , :ssl false
  , :status :running
  }

, :tags
  [ :api
  , :backend
  , :production
  ]

, :numbers
  { :int 42
  , :hex 0xff
  , :float 3.14
  , :scientific 1e10
  }

, :escaping
  { :price \\$99.99
  , :literal-colon \\:not-a-symbol
  }
}`;

  return (
    <div style={{ maxWidth: '800px', margin: '0 auto' }}>
      <h1 style={{ color: '#fff' }}>SYM Syntax Highlighter</h1>
      <SymHighlight code={exampleCode} />
    </div>
  );
}

export default SymHighlight;
