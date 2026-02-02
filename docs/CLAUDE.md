# SYM Format Guide for Claude

This guide helps Claude (and other AI assistants) work effectively with the SYM data format.

## Quick Reference

SYM is a human-friendly data format that's less verbose than JSON/YAML. Key points:

- **Objects**: `{ :key value, :another key }`
- **Arrays**: `[ item one, item two ]`
- **Variables**: `{ $var value }` then reference with `$var`
- **Symbols**: `:active`, `:redis` (like enums)
- **Separator**: newline + comma (`\n,`)
- **Inline commas are safe**: `Hello, world` works without quotes
- **Comments**: `//` and `/* */`

## Core Principles

### 1. Strings Don't Need Quotes
```sym
{ :name John Doe
, :address 123 Main St, Apt 4, New York, NY
, :message Hello, world! How are you?
}
```

Only newline-comma (`\n,`) separates fields. Inline commas are literal.

### 2. Leading Commas Style
```sym
// ✅ Idiomatic SYM
{ :first value
, :second another value
, :third yet another
}

// ❌ Avoid inline style
{ :first value, :second another value }  // Parses :first as "value, :second another value"
```

### 3. Types Are Detected
- Numbers: `42`, `3.14`, `0xff`, `1e10`, `inf`, `nan`
- Booleans: `true`, `false`
- Null: `null`
- Symbols: `:active`, `:redis` (starts with `:`)
- Strings: everything else

### 4. Multiline Strings Just Work
```sym
{ :description
    This is a multiline string.
    It continues on the next line.
    Commas, like this, are preserved.
, :next-key value
}
```

## Reading SYM Files

When Claude reads a SYM file, here's what to look for:

### Structure
```sym
// 1. Optional imports (at top)
@import ./base.sym

// 2. Optional variable definitions (defs blocks)
{ $env production
, $port 8080
}

// 3. The actual data (last block)
{ :server
  { :host localhost
  , :port $port
  }
}
```

### Common Patterns

**Configuration with variables:**
```sym
{ $domain example.com }
{ :api-url https://api.$domain
, :cdn-url https://cdn.$domain
}
```

**Symbols for enums:**
```sym
{ :status :active
, :role :admin
, :log-level :debug
}
```

**Arrays of objects:**
```sym
{ :users
  [ { :name Alice, :age 30 }
  , { :name Bob, :age 25 }
  ]
}
```

## Writing SYM Files

### Step-by-Step

1. Start with the structure (object or array)
2. Add variables if values repeat
3. Use symbols for enum-like values
4. One key-value per line
5. Leading commas

**Example:**
```sym
// Define reusable values
{ $company Acme Corp
, $year 2024
}

// Main data
{ :company $company
, :fiscal-year $year
, :departments
  [ :engineering
  , :sales
  , :marketing
  ]
, :config
  { :debug true
  , :timeout 30000
  , :endpoints
    { :api https://api.example.com
    , :auth https://auth.example.com
    }
  }
}
```

### When to Use What

| Feature | Use When | Example |
|---------|----------|---------|
| Variables (`$var`) | Values repeat or need DRY | `$env`, `$domain` |
| Symbols (`:symbol`) | Enum-like values, tags | `:active`, `:redis` |
| Strings | Everything else | `Hello`, `New York` |
| Multiline | Long text, code blocks | Descriptions, SQL queries |

## Converting From Other Formats

### From JSON
```json
{
  "name": "my-app",
  "version": "1.0.0",
  "features": ["auth", "cache"]
}
```
↓
```sym
{ :name my-app
, :version 1.0.0
, :features
  [ auth
  , cache
  ]
}
```

### From YAML
```yaml
server:
  host: localhost
  port: 8080
  ssl: true
```
↓
```sym
{ :server
  { :host localhost
  , :port 8080
  , :ssl true
  }
}
```

## Advanced Features

### Imports and Merging
```sym
// base.sym
{ :database
  { :host localhost
  , :port 5432
  }
}

// production.sym
@import ./base.sym
{ :database
  { :host prod.example.com  // Merges with base, overrides :host
  , :ssl true               // Adds new field
  }
}
```

### Replace vs Merge
```sym
@import ./base.sym

{ :logging!              // ! = replace entirely (don't merge)
  { :driver datadog
  }
}
```

### Append Arrays
```sym
@import ./base.sym

{ :plugins+              // + = append to array
  [ :cache
  , :ratelimit
  ]
}
```

## Common Tasks

### Reading Configuration
```sym
{ $env production }
{ :database
  { :host db.$env.example.com
  , :port 5432
  , :pool-size 100
  }
, :redis
  { :url redis://$env-cache.example.com:6379
  , :ttl 3600
  }
}
```

**Variables to substitute:**
- `$env` → `production`
- Result: `db.production.example.com`, `redis://production-cache.example.com:6379`

### Writing API Response
```sym
{ :status :success
, :code 200
, :data
  { :user
    { :id 12345
    , :name Alice Johnson
    , :email alice@example.com
    , :roles
      [ :user
      , :premium
      ]
    }
  , :metadata
    { :timestamp 2024-01-15T10:30:00Z
    , :request-id abc-123-def
    }
  }
}
```

### Infrastructure as Code
```sym
{ $region us-east-1 }
{ :resources
  [ { :type :aws-ec2-instance
    , :name web-server
    , :properties
      { :instance-type t3.medium
      , :ami ami-12345
      , :region $region
      }
    }
  , { :type :aws-rds-instance
    , :name database
    , :properties
      { :engine postgres
      , :version 15.2
      , :region $region
      }
    }
  ]
}
```

## Escaping Rules

Only escape at **value start**:

| Write | Get | Notes |
|-------|-----|-------|
| `\$100` | `$100` | Prevent variable substitution |
| `\:foo` | `:foo` | String, not symbol |
| `\42` | `42` | String, not number |
| `\true` | `true` | String, not boolean |
| `\{text}` | `{text}` | String, not object |
| `\\` | `\` | Literal backslash |

**Mid-string, no escaping needed:**
```sym
{ :url https://example.com    // : and / are literal
, :price $19.99               // $ is literal (not at value start)
, :note See {file} and [doc]  // Braces/brackets literal
}
```

## Error Prevention

### Common Mistakes

**❌ Inline commas separate fields:**
```sym
{ :name John, :age 30 }  // Wrong! Parses as :name = "John, :age 30"
```

**✅ Use newline-comma:**
```sym
{ :name John
, :age 30
}
```

**❌ Forgetting variable definition:**
```sym
{ :port $port }  // Error: $port undefined
```

**✅ Define first:**
```sym
{ $port 8080 }
{ :port $port }
```

**❌ Symbol mid-string confusion:**
```sym
{ :image :nginx:alpine }  // Wrong! Symbol is ":nginx:alpine"
```

**✅ At value start only:**
```sym
{ :image nginx:alpine }  // Correct! String "nginx:alpine"
```

## Integration with Tools

### Rust Parser
```bash
# Parse and validate
sym-parser config.sym

# Convert from JSON
sym-parser --from-json config.json

# Output as JSON
sym-parser config.sym --json
```

### In Code
```rust
use sym_parser::parse;

let source = r#"
{ :name Alice
, :age 30
}
"#;

let result = parse(source)?;
```

## Tips for Claude

1. **Always use leading comma style** when writing SYM
2. **Don't quote strings** unless escaping needed
3. **Use symbols for enums** (`:active`, `:pending`, etc.)
4. **Extract repeated values** into variables
5. **One key-value per line** for clarity
6. **Indent nested structures** with 2 spaces
7. **Close brackets on their own line**

## Examples to Study

See [examples/](../examples/) directory:
- [simple-config.sym](../examples/simple-config.sym) - Start here
- [app-config.sym](../examples/app-config.sym) - Production config
- [comprehensive.sym](../examples/comprehensive.sym) - All features
- [kubernetes-deployment.sym](../examples/kubernetes-deployment.sym) - Complex nesting
- [aws-infrastructure.sym](../examples/aws-infrastructure.sym) - IaC example

## Full Specification

For complete details, see [SPEC.md](../spec/SPEC.md).
