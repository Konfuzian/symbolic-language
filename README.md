# SYM

**A data format for humans.** Less syntax, more clarity.

**[Live Demo →](https://konfuzian.github.io/symbolic-language/)**

```sym
// Application configuration
{ :name my-app
, :version 1.0.0
, :database
  { :host localhost
  , :port 5432
  , :credentials
    { :user admin
    , :password secret123
    }
  }
, :features
  [ :dark-mode
  , :notifications
  , :export
  ]
}
```

## Why SYM?

| Feature | JSON | YAML | TOML | SYM |
|---------|------|------|------|-----|
| No quotes needed | ❌ | ✅ | Partial | ✅ |
| Inline commas safe | ❌ | ❌ | ❌ | ✅ |
| Clear nesting | ❌ | ❌ | ❌ | ✅ |
| Comments | ❌ | ✅ | ✅ | ✅ |
| Variables | ❌ | Anchors | ❌ | ✅ |
| Symbols/Enums | ❌ | ❌ | ❌ | ✅ |

**Key insight:** Commas inside your values are fine — only `\n,` (newline + comma) separates elements.

```sym
{ :address 123 Main St, Apt 4, New York, NY 10001
, :message Hello, world!
}
```

## Quick Start

### Syntax at a Glance

```sym
// Line comments
/* Block comments */

// Objects use { }
{ :key value
, :another-key another value
}

// Arrays use [ ]
[ first item
, second item
, third item
]

// Variables
{ $base-url https://api.example.com
}
{ :endpoint $base-url/users
}

// Symbols (like enums)
{ :status :active
, :role :admin
}

// Multiline strings (just indent)
{ :description
    This is a multiline string.
    It continues on the next line.
    Commas, like this, are preserved.
}
```

### Formatting Rules

SYM is designed for **one key-value pair per line** with **leading commas**:

```sym
// ✅ Idiomatic SYM
{ :name John
, :age 30
, :city New York
}

// ❌ Avoid inline style
{ :name John, :age 30 }  // This parses as :name with value "John, :age 30"
```

## Documentation

### Getting Started
- **[Live Playground](https://konfuzian.github.io/symbolic-language/)** — Interactive demo with syntax highlighting
- **[Cheatsheet](docs/CHEATSHEET.md)** — Quick syntax reference (2 min read)
- **[Examples](examples/)** — Real-world configuration files

### Complete Reference
- **[Full Specification](spec/SPEC.md)** — Complete language reference
- **[Documentation Hub](docs/)** — All guides and references

### For AI Assistants
- **[Claude Guide](docs/CLAUDE.md)** — Comprehensive guide for AI assistants
- **[Usage Guide](docs/USAGE.md)** — Working with Claude and SYM files

## Implementations

### Rust Parser

A complete parser with CLI and library API.

```bash
cd parsers/rust
cargo build --release

# Parse a file
sym-parser config.sym

# Convert from JSON
sym-parser --from-json config.json

# Output as JSON
sym-parser config.sym --json
```

See [parsers/rust/](parsers/rust/) for details.

### JavaScript Syntax Highlighter

A React component for syntax highlighting SYM code.

```jsx
import SymHighlight from './sym-highlight';

<SymHighlight code={symCode} />
```

See [parsers/js/](parsers/js/) for details.

## Examples

The [examples/](examples/) directory contains real-world configurations:

- `simple-config.sym` — Basic application config
- `docker-compose.sym` — Docker Compose equivalent
- `kubernetes-deployment.sym` — Kubernetes manifest
- `package.sym` — Node.js package.json equivalent
- `github-actions.sym` — CI/CD pipeline
- `app-config.sym` — Production application config
- `api-response.sym` — REST API response
- `tailwind-config.sym` — Tailwind CSS config
- `eslint-config.sym` — ESLint configuration
- `aws-infrastructure.sym` — Terraform-like AWS config

## Design Principles

1. **Human-first** — Optimized for reading and writing by humans
2. **Unambiguous** — Clear structure through brackets and indentation
3. **Safe strings** — Inline commas don't break your data
4. **DRY** — Variables prevent repetition
5. **Typed where useful** — Numbers, booleans, symbols, null detected automatically

## License

MIT
