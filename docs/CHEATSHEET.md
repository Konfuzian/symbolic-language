# SYM Format Cheatsheet

## Basic Syntax

| Feature | Syntax | Example |
|---------|--------|---------|
| Object | `{ :key value }` | `{ :name Alice, :age 30 }` |
| Array | `[ item ]` | `[ red, green, blue ]` |
| Comment | `//` or `/* */` | `// This is a comment` |
| Variable def | `{ $var value }` | `{ $port 8080 }` |
| Variable use | `$var` | `:port $port` |
| Symbol | `:symbol` | `:status :active` |
| Multiline | Just indent | See below |

## Formatting Style

```sym
// ✅ DO: One per line, leading commas
{ :name John
, :age 30
, :city New York
}

// ❌ DON'T: Inline commas separate
{ :name John, :age 30 }  // Parses as one value!
```

## Values

| Type | Examples |
|------|----------|
| String | `hello`, `New York`, `Version 2.0` |
| Integer | `42`, `-17`, `1_000_000`, `0xff`, `0b1010` |
| Float | `3.14`, `1e10`, `inf`, `-inf`, `nan` |
| Boolean | `true`, `false` |
| Null | `null` |
| Symbol | `:active`, `:redis`, `:info` |
| Variable | `$port`, `$domain` |

## Separator: `\n,`

```sym
// Inline commas are SAFE (literal)
{ :address 123 Main St, Apt 4, New York
, :message Hello, world!
}

// Only newline+comma separates
{ :first one
, :second two
}
```

## Multiline Strings

```sym
{ :description
    This is line one.
    This is line two.
    Inline commas, like this, work fine.
, :next-key value
}
```

## Variables

```sym
// Define in defs block
{ $domain example.com
, $port 8080
}

// Use in data block
{ :api-url https://api.$domain
, :server-port $port
}
```

## Symbols (Enums)

```sym
{ :status :active          // Not a string, not a var
, :log-level :debug
, :cache :redis
, :services
  [ :postgres
  , :nginx
  , :redis
  ]
}
```

## Imports

```sym
@import ./base.sym
@import ../shared/config.sym

{ :custom-key value }
```

## Modifiers

| Syntax | Meaning | Example |
|--------|---------|---------|
| `:key` | Merge (default) | `:database { :host localhost }` |
| `:key!` | Replace entirely | `:logging! { :driver datadog }` |
| `:key+` | Append to array | `:plugins+ [ :cache ]` |
| `$var!` | Override variable | `$port! 9000` |

## Escaping (Only at Value Start)

| Input | Output | Notes |
|-------|--------|-------|
| `\$100` | `$100` | String, not variable |
| `\:foo` | `:foo` | String, not symbol |
| `\42` | `42` | String, not number |
| `\true` | `true` | String, not boolean |
| `\{text}` | `{text}` | String, not object |
| `\\` | `\` | Literal backslash |

**Mid-string = no escape needed:**
```sym
{ :url https://example.com    // : is literal
, :price costs $19.99         // $ is literal
}
```

## Common Patterns

### Config with Variables
```sym
{ $env prod }
{ :db-host db.$env.example.com }
```

### Nested Structures
```sym
{ :server
  { :http
    { :host localhost
    , :port 8080
    }
  , :https
    { :host localhost
    , :port 8443
    , :ssl true
    }
  }
}
```

### Arrays of Objects
```sym
{ :users
  [ { :name Alice, :role :admin }
  , { :name Bob, :role :user }
  ]
}
```

### Import and Override
```sym
@import ./base.sym
{ :database
  { :host prod.example.com  // Merges with base
  }
}
```

## Reserved Words (Escape if Needed at Start)

- `true`, `false`
- `null`
- `inf`, `-inf`, `nan`
- Numbers: `42`, `3.14`, `0xff`, etc.
- `{` (starts object)
- `[` (starts array)

## Quick Rules

1. One key-value per line
2. Leading commas
3. No quotes needed
4. Indent with 2 spaces
5. Close `}` and `]` on own line
6. Only `\n,` separates elements
7. Use `$var` for DRY
8. Use `:symbol` for enums

## Example: Complete Config

```sym
{ $env production
, $region us-east-1
}

{ :app
  { :name my-service
  , :version 2.1.0
  , :env $env
  }
, :database
  { :host db.$env.example.com
  , :port 5432
  , :pool-size 100
  , :ssl true
  , :region $region
  }
, :cache
  { :driver :redis
  , :url redis://$env-cache.example.com:6379
  , :ttl 3600
  }
, :features
  [ :auth
  , :logging
  , :metrics
  ]
, :logging
  { :level :info
  , :format :json
  , :outputs
    [ :stdout
    , :file
    ]
  }
}
```

## Parser Commands

```bash
# Parse and validate
sym-parser config.sym

# From JSON
sym-parser --from-json config.json

# To JSON
sym-parser config.sym --json
```

## Learn More

- Full spec: [SPEC.md](../spec/SPEC.md)
- Claude guide: [CLAUDE.md](CLAUDE.md)
- Examples: [examples/](../examples/)
- Live demo: https://anthropics.github.io/symbolic-language
