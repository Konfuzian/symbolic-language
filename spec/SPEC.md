# SYM Format Specification v0.1

A data format inspired by JSON — less verbose, easier to read.

---

## Introduction

SYM is a human-friendly data format designed to be:
- **Less verbose** than JSON (no quotes, fewer braces and commas)
- **Easy to read and write** for configuration files
- **Flexible** with multiline strings and inline comments
- **Type-rich** with support for symbols, numbers, booleans, and more

### Quick Example

```
{ :name Alice
, :age 28
, :active true
, :hobbies
  [ reading
  , cycling
  , photography
  ]
}
```

In JSON, this would require quotes around every string and key, making it harder to scan visually. SYM keeps the structure clear while reducing noise.

---

## Basic Values

SYM supports several basic value types:

| Type | Examples | Notes |
|------|----------|-------|
| String | `hello`, `Hello, world`, `New York` | always unquoted |
| Int | `42`, `-42`, `1_000_000` | |
| Float | `3.14`, `42.0`, `1e10` | |
| Boolean | `true`, `false` | |
| Null | `null` | |

**Strings** are always unquoted. If a value doesn't match another type (number, boolean, null, symbol, object, array), it's a string. Use `\` to force values to be interpreted as strings (e.g., `\42` for the string "42").

**Numbers** can use underscores for readability (`1_000_000`) and support scientific notation (`3.14e-5`).

---

## Basic Structures

### Objects

Objects use curly braces with colon-prefixed keys:

```
{ :name Alice
, :age 28
, :active true
}
```

Keys start with `:` and values come after. Commas go at the start of new lines, not the end.

If a key has no value, it defaults to an empty string:
```
{ :name
, :age 28
}
// :name is ""
```

### Arrays

Arrays use square brackets:

```
[ one
, two
, three
]
```

Values are separated the same way as object fields.

---

## Comments

```
// line comment at start

{ :name Alice  // inline comment (after whitespace)
, :url https://example.com   // this works, url:// is not a comment
}

/* block
   comment */
```

- `//` — line comment, must be at start of line or preceded by whitespace
- `/* */` — block comment (can span lines)

---

## Field Separator

The separator is `\n\s*,` — a newline followed by optional whitespace (including blank lines) and a comma.

Blank lines between fields are allowed:

```
{ :first one

, :second two
}
```

Inline commas are literal (safe in string values):

```
{ :greeting Hello, world
, :note This comma is fine, no problem
}
```

---

## Multiline Strings

Just work — lines accumulate until the next separator or closing bracket:

```
{ :poem
    Roses are red
    Violets are blue
    No comma-newline here
    So it's all one value
, :author Anonymous
}
```

**Whitespace handling:**
- Leading whitespace is stripped (per line)
- Trailing whitespace is stripped (per line)
- To preserve leading whitespace, escape it with `\` (e.g., `\ ` for space, `\` + tab for tab)

```
{ :code
    def hello():
\       print("hi")
, :note indentation preserved with backslash
}
```

---

## Symbols

Symbols are `:prefixed` values — a distinct type, like strings or numbers:

```
{ :status :active
, :services
  [ :redis
  , :postgres
  ]
}
```

- Symbols are **not** strings — they're a separate type
- Tools can interpret them as enums, tags, commands, etc.
- `:foo` is only a symbol at the **start** of a value

Mid-string colons are literal:

```
{ :image nginx:alpine        // string "nginx:alpine"
, :url https://example.com   // string "https://example.com"
, :status :running           // symbol :running
}
```

---

## Advanced Number Formats

SYM supports various number formats:

```
{ :hex 0xff                // 255
, :binary 0b1010           // 10
, :octal 0o755             // 493
, :scientific 6.022e23     // Avogadro's number
, :infinity inf
, :neg-infinity -inf
, :not-a-number nan
}
```

**Number formats:**
- Integers: decimal, hex (`0x`), binary (`0b`), octal (`0o`)
- Underscores allowed: `1_000_000`
- Scientific notation: `1e10`, `3.14e-5`, `6.022E+23`
- Special floats: `inf`, `-inf`, `nan`
- Int vs float determined by presence of `.` or special value

---

## Variables

Define variables in a defs block, reference them by name:

```
{ $theme dark
, $accent #ff5500
}
{ :config
  { :mode $theme
  , :highlight $accent
  }
}
```

**Rules:**
- A block is a defs block if it contains only `$`-prefixed keys and is not the last block
- Multiple defs blocks allowed — definitions accumulate
- The last block is always data
- Variables are substituted with their values
- Undefined `$variable` is an error
- `$foo` is only a variable at the **start** of a value

**Multiple defs blocks:**
```
{ $env prod }
{ $region us-east }
{ :deploy $env-$region }
```

---

## Document Structure

With variables and imports, a full SYM document has this structure:

```
[imports]?
[defs block]*
<data>
```

- **Imports** — optional `@import` directives at top of file
- **Defs block** — optional `{ }` containing only `$`-prefixed variable definitions
- **Data** — any value: object, array, string, number, boolean, null, symbol

---

## Imports

```
@import ./base.sym
@import ../shared/config.sym

{ $port 8080 }
{ :server { :port $port } }
```

**Rules:**
- Imports must appear at the top of the file (before any defs or data)
- Multiple imports allowed, processed in order
- Imported `$variables` are added to scope
- Imported data is merged with local data
- Duplicate variables require explicit override with `$var!`

---

### Variable Override

```
// base.sym
{ $port 3000
, $host localhost
}

// main.sym
@import ./base.sym

{ $port! 8080 }              // OK - explicit override
```

Without `!`, duplicate variable definitions are an error.

---

### Data Merging

Imported data is **deep merged** with local data:

```
// base.sym
{ :database
  { :host localhost
  , :port 5432
  }
, :logging
  { :level :debug
  , :format :json
  , :output stdout
  }
, :cache
  { :enabled false
  }
}
```

```
// production.sym
@import ./base.sym

{ :database
  { :host prod.example.com
  , :ssl true
  }
}
```

**Result:**
```
{ :database
  { :host prod.example.com   // overridden
  , :port 5432               // inherited
  , :ssl true                // added
  }
, :logging
  { :level :debug
  , :format :json
  , :output stdout
  }
, :cache
  { :enabled false
  }
}
```

---

### Replace with `!`

Use `:key!` to replace a key entirely instead of merging:

```
// production.sym
@import ./base.sym

{ :logging!
  { :driver datadog
  , :api-key abc123
  }
}
```

**Result:**
```
{ :database
  { :host localhost
  , :port 5432
  }
, :logging
  { :driver datadog          // replaced entirely
  , :api-key abc123          // no inherited fields
  }
, :cache
  { :enabled false
  }
}
```

Without `!`, inherited fields like `:level`, `:format`, `:output` would leak into the merged result.

---

### Append Arrays with `+`

Use `:key+` to append to an array instead of replacing:

```
// base.sym
{ :plugins
  [ :auth
  , :logging
  ]
}
```

```
// extended.sym
@import ./base.sym

{ :plugins+
  [ :cache
  , :ratelimit
  ]
}
```

**Result:**
```
{ :plugins
  [ :auth
  , :logging
  , :cache
  , :ratelimit
  ]
}
```

---

### Modifier Summary

| Syntax | Meaning |
|--------|---------|
| `:key` | Deep merge (default) |
| `:key!` | Replace entirely (no inherited fields) |
| `:key+` | Append to array |
| `$var!` | Override variable |

---

## Escaping

`\` at the start of a value forces interpretation as a string (disables special types):

| Write | Get |
|-------|-----|
| `\\` | `\` |
| `\` + space | preserved leading space |
| `\` + tab | preserved leading tab |
| `\,` | `,` (prevents separator at line start) |
| `\{` | string starting with `{` |
| `\[` | string starting with `[` |
| `\:foo` | string `:foo` |
| `\$100` | string `$100` |
| `\42` | string `42` |
| `\true` | string `true` |
| `\false` | string `false` |
| `\null` | string `null` |
| `\inf` | string `inf` |
| `\nan` | string `nan` |

**Note on `{` and `[`:** These only need escaping at value start (first non-whitespace after key). Mid-string braces and brackets are literal:

```
{ :note
    \{this starts with a brace
    but {these} and [these] are fine
    no escaping needed mid-string
}
```

The first character after `:note` determines the type:
- `{` → object
- `[` → array
- Anything else → string (then `{` and `[` are literal throughout)

---

## Reserved Words

These parse as non-strings at **value start** and must be escaped if you want the literal string:

- `true`, `false` — booleans
- `null` — null
- `inf`, `-inf`, `nan` — floats
- Numeric patterns — `42`, `3.14`, `0xff`, `1e10`, etc.
- `{` — starts an object
- `[` — starts an array

Mid-string, these are all literal (no escaping needed).

---

## Examples

**Simple object:**
```
{ :name Alice
, :age 28
}
```

**With variables:**
```
{ $env production }
{ :database
  { :host db.$env.example.com
  , :pool 10
  }
}
```

**Array as root:**
```
{ $domain example.com }
[ https://$domain
, https://api.$domain
, http://localhost:3000
]
```

---

## Grammar (informal)

```
document     = import* defs* data
import       = '@import' path

defs         = '{' (var-binding separator)* '}'   // only $-keys, not last block
var-binding  = '$' identifier '!'? value

data         = value
value        = object | array | string | number | boolean | null | symbol | variable

object       = '{' (field separator)* '}'
field        = ':' identifier ('!' | '+')? value?  // ! = replace, + = append

array        = '[' (value separator)* ']'

separator    = '\n' whitespace* ','

symbol       = ':' identifier
variable     = '$' identifier
identifier   = [a-zA-Z_][a-zA-Z0-9_-]*

number       = int | float
int          = decimal | hex | binary | octal
float        = decimal '.' digits exponent? | digits exponent | 'inf' | '-inf' | 'nan'
exponent     = ('e' | 'E') ('+' | '-')? digits

boolean      = 'true' | 'false'
null         = 'null'

string       = (anything not matching above, or escaped)

comment      = ('^' | '\s') '//' .* '\n' | '/*' .* '*/'
```

---

## File Extension

`.sym`

---

## Comprehensive Example

```
/*
 * SYM Format - Comprehensive Example
 * Demonstrates all features
 */

// === Variable Definitions ===

{ $app myapp
, $version 2.1.0
, $debug true
, $max-connections 1_000
}

{ $primary-color #3498db
, $timeout-ms 30000
}

// === Data Block ===

{ :meta
  { :name $app                         // variable substitution
  , :version $version
  , :description
      A sample application config
      that spans multiple lines.
      Commas, like this, are fine inline.
  , :keywords
    [ backend
    , api
    , $app                              // variable in array
    ]
  }

, :server
  { :host localhost
  , :port 8080
  , :ssl false
  , :timeout $timeout-ms
  , :status :running                   // symbol value
  }

, :database
  { :primary
    { :host db.example.com
    , :port 5432
    , :pool-size $max-connections
    , :ssl true
    }
  , :replica
    { :host replica.example.com
    , :port 5432
    , :read-only true
    }
  }

, :feature-flags
  { :debug-mode $debug
  , :new-ui true
  , :legacy-api false
  , :experimental null                 // null value
  }

, :services                            // array of symbols
  [ :redis
  , :postgres
  , :nginx
  , :prometheus
  ]

, :numbers-showcase
  { :int 42
  , :negative -17
  , :big 1_000_000
  , :hex 0xff
  , :binary 0b1010
  , :octal 0o755
  , :float 3.14159
  , :explicit-float 42.0
  , :scientific 6.022e23
  , :sci-negative 1.5e-10
  , :infinity inf
  , :neg-infinity -inf
  , :not-a-number nan
  }

, :escaping-showcase
  { :price \$99.99                     // string "$99.99"
  , :symbol-text \:not-a-symbol        // string ":not-a-symbol"
  , :number-text \42                   // string "42"
  , :bool-text \true                   // string "true"
  , :null-text \null                   // string "null"
  , :backslash \\path\\to\\file        // string "\path\to\file"
  , :brace-text \{not an object}       // string "{not an object}"
  , :bracket-text \[not an array]      // string "[not an array]"
  }

, :mid-string-braces
    This contains {braces} and [brackets]
    No escaping needed because value started with T

, :empty-value                         // empty string

, :multiline-with-indent
\   line 1 with preserved indent
\   line 2 with preserved indent
    line 3 no indent (stripped)

, :log-levels
  [ :trace
  , :debug
  , :info
  , :warn
  , :error
  , :fatal
  ]

, :theme
  { :colors
    { :primary $primary-color
    , :secondary #2ecc71
    , :danger #e74c3c
    , :warning #f39c12
    }
  , :fonts
    [ Inter, sans-serif
    , Fira Code, monospace
    ]
  }

, :message Hello, world! Welcome to $app version $version.
}
```

**After variable substitution, the data block is the output.** Variables like `$app` become `myapp`, `$version` becomes `2.1.0`, etc. Symbols like `:running` and `:redis` remain as symbols.

