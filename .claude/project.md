# SYM Format Project Context

## Project Overview

SYM is a human-friendly data format designed as an alternative to JSON, YAML, and TOML. It emphasizes readability, safety (inline commas don't break data), and developer experience.

## Key Concepts

- **Separator**: `\n,` (newline + comma) separates elements, so inline commas are safe
- **No quotes needed**: String values don't require quotes
- **Leading commas**: Idiomatic style uses commas at the start of lines
- **Variables**: `$var` syntax for reusable values (DRY principle)
- **Symbols**: `:keyword` syntax for enum-like values
- **Types**: Auto-detected (numbers, booleans, null, symbols, strings)

## Repository Structure

```
.
├── README.md              Main introduction and quick start
├── spec/
│   └── SPEC.md           Complete language specification
├── docs/
│   ├── index.html        Live interactive playground
│   └── CLAUDE.md         AI assistant guide (START HERE for Claude)
├── examples/             Real-world configuration examples
│   ├── simple-config.sym
│   ├── app-config.sym
│   ├── kubernetes-deployment.sym
│   └── ...
├── parsers/
│   ├── rust/             Full parser implementation in Rust
│   └── js/               Syntax highlighter for React
└── playground/           Interactive demo component
```

## For Claude: Quick Start

1. Read [docs/CLAUDE.md](docs/CLAUDE.md) for comprehensive guide
2. Check [examples/](examples/) for real-world usage patterns
3. Refer to [spec/SPEC.md](spec/SPEC.md) for detailed language rules

## SYM Syntax Quick Reference

```sym
// Variables (define once, use many times)
{ $domain example.com }

// Main data
{ :name my-app
, :version 1.0.0
, :url https://api.$domain
, :status :active                // Symbol (enum-like)
, :features
  [ :auth
  , :cache
  , :logging
  ]
, :config
  { :debug true
  , :port 8080
  , :timeout 30000
  }
, :description
    This is a multiline string.
    Inline commas, like this, are preserved.
}
```

## Important Rules for Writing SYM

1. **One key-value per line** with leading commas
2. **Don't quote strings** (only escape if value starts with special char)
3. **Use `$var` for repeated values**
4. **Use `:symbol` for enum-like values**
5. **Indent with 2 spaces** for nested structures
6. **Close brackets on their own line**

## Common Patterns

### Configuration with Environment
```sym
{ $env production }
{ :database
  { :host db.$env.example.com
  , :port 5432
  }
}
```

### API Response
```sym
{ :status :success
, :code 200
, :data
  { :items
    [ { :id 1, :name Item 1 }
    , { :id 2, :name Item 2 }
    ]
  }
}
```

### Import and Merge
```sym
@import ./base.sym
{ :logging!              // ! = replace instead of merge
  { :level :info }
}
```

## Parser Usage

```bash
# Rust parser (in parsers/rust/)
cargo build --release
sym-parser config.sym
sym-parser --from-json config.json
sym-parser config.sym --json

# JavaScript highlighter (in parsers/js/)
import SymHighlight from './sym-highlight';
<SymHighlight code={symCode} />
```

## Escaping Rules

Only escape at value start:
- `\$100` → string "$100" (not variable)
- `\:foo` → string ":foo" (not symbol)
- `\42` → string "42" (not number)
- `\true` → string "true" (not boolean)
- `\{` → string starting with "{"
- `\\` → literal backslash

Mid-string, no escaping needed:
- `https://example.com` → colon is literal
- `price is $19.99` → dollar is literal (not at start)

## Design Philosophy

1. **Human-first**: Optimized for humans, not machines
2. **Unambiguous**: Structure is always clear
3. **Safe strings**: Inline commas don't break parsing
4. **DRY**: Variables eliminate repetition
5. **Sensible types**: Auto-detect common types

## Related Files

- **Language spec**: [spec/SPEC.md](spec/SPEC.md)
- **Claude guide**: [docs/CLAUDE.md](docs/CLAUDE.md)
- **Examples**: [examples/](examples/)
- **Live demo**: https://anthropics.github.io/symbolic-language

## License

MIT
