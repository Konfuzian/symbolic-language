# SYM Format Documentation

Welcome to the SYM format documentation. Choose your starting point:

## For AI Assistants (Claude, etc.)

Start here if you're an AI assistant working with SYM files:

- **[CLAUDE.md](CLAUDE.md)** - Comprehensive guide for AI assistants
  - Reading and writing SYM
  - Common patterns and conversions
  - Tips and best practices
  - 10-15 minute read

## Quick References

- **[CHEATSHEET.md](CHEATSHEET.md)** - Syntax quick reference
  - All syntax in one page
  - Quick lookup table
  - Common patterns
  - 2-3 minute read

- **[USAGE.md](USAGE.md)** - Working with Claude
  - Example prompts
  - Common workflows
  - Best practices for AI interaction
  - 5-7 minute read

## Complete Documentation

- **[SPEC.md](../spec/SPEC.md)** - Full language specification
  - Every language feature
  - Grammar definitions
  - Edge cases and rules
  - Reference documentation

## Examples

- **[examples/](../examples/)** - Real-world SYM files
  - Application configs
  - Infrastructure as code
  - API responses
  - Docker, Kubernetes, CI/CD

## Interactive

- **[Live Playground](https://anthropics.github.io/sym-format)** - Try SYM in your browser
  - Syntax highlighting
  - Live parsing
  - Example gallery

## Learning Path

### 1. Quick Start (5 minutes)
1. Read [CHEATSHEET.md](CHEATSHEET.md)
2. Look at [examples/simple-config.sym](../examples/simple-config.sym)
3. Try the [Live Playground](https://anthropics.github.io/sym-format)

### 2. Deep Dive (30 minutes)
1. Read [CLAUDE.md](CLAUDE.md) or similar guide
2. Study [examples/comprehensive.sym](../examples/comprehensive.sym)
3. Review [SPEC.md](../spec/SPEC.md) sections as needed

### 3. Advanced (1 hour)
1. Read full [SPEC.md](../spec/SPEC.md)
2. Study all [examples/](../examples/)
3. Build with [parsers/rust/](../parsers/rust/)

## By Use Case

### I want to...

**Understand SYM basics**
→ [CHEATSHEET.md](CHEATSHEET.md) + [examples/simple-config.sym](../examples/simple-config.sym)

**Convert JSON/YAML to SYM**
→ [CLAUDE.md](CLAUDE.md) "Converting From Other Formats" section

**Write SYM configs**
→ [USAGE.md](USAGE.md) + [examples/](../examples/)

**Parse SYM programmatically**
→ [parsers/rust/](../parsers/rust/) or [parsers/js/](../parsers/js/)

**Learn all language features**
→ [SPEC.md](../spec/SPEC.md) + [examples/comprehensive.sym](../examples/comprehensive.sym)

**Use SYM with AI assistants**
→ [USAGE.md](USAGE.md)

**Debug syntax errors**
→ [CHEATSHEET.md](CHEATSHEET.md) + [CLAUDE.md](CLAUDE.md) "Error Prevention"

## Documentation Files

| File | Purpose | Audience | Length |
|------|---------|----------|--------|
| [CHEATSHEET.md](CHEATSHEET.md) | Quick syntax reference | Everyone | 2-3 min |
| [CLAUDE.md](CLAUDE.md) | AI assistant guide | AI/Claude | 10-15 min |
| [USAGE.md](USAGE.md) | Working with Claude | Humans using AI | 5-7 min |
| [SPEC.md](../spec/SPEC.md) | Complete specification | Implementers | 30+ min |
| [examples/README.md](../examples/README.md) | Example overview | Everyone | 2 min |

## Key Concepts

Before diving in, understand these core ideas:

1. **Separator is `\n,`** - Only newline+comma separates elements, so inline commas are safe
2. **No quotes needed** - Strings don't require quotes unless escaping special values
3. **Leading commas** - Idiomatic style for easy editing and clean diffs
4. **Variables with `$`** - Define once, use many times (DRY principle)
5. **Symbols with `:`** - Enum-like values that aren't substituted

## Example

```sym
// Define variables
{ $env production
, $domain example.com
}

// Use them in config
{ :app my-service
, :database
  { :host db.$env.$domain
  , :port 5432
  , :ssl true
  }
, :cache
  { :driver :redis              // Symbol (enum-like)
  , :url redis://cache.$env.$domain:6379
  }
, :features
  [ :auth                       // Array of symbols
  , :logging
  , :metrics
  ]
}
```

## Contributing

See the main [README.md](../README.md) for contribution guidelines.

## License

MIT - See [LICENSE](../LICENSE) file
