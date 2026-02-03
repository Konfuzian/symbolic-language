# SYM

**A data format for humans.** Less syntax, more clarity.

**[Live Demo →](https://konfuzian.github.io/symbolic-language/)**

<pre><code><span style="color:#6a9955">// Application configuration</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:name</span> <span style="color:#ce9178">my-app</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:version</span> <span style="color:#b5cea8">1.0.0</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:database</span>
  <span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:host</span> <span style="color:#ce9178">localhost</span>
  <span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:port</span> <span style="color:#b5cea8">5432</span>
  <span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:credentials</span>
    <span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:user</span> <span style="color:#ce9178">admin</span>
    <span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:password</span> <span style="color:#ce9178">secret123</span>
    <span style="color:#ffd700">}</span>
  <span style="color:#ffd700">}</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:features</span>
  <span style="color:#ffd700">[</span> <span style="color:#4ec9b0">:dark-mode</span>
  <span style="color:#d4d4d4">,</span> <span style="color:#4ec9b0">:notifications</span>
  <span style="color:#d4d4d4">,</span> <span style="color:#4ec9b0">:export</span>
  <span style="color:#ffd700">]</span>
<span style="color:#ffd700">}</span>
</code></pre>

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

<pre><code><span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:address</span> <span style="color:#ce9178">123 Main St, Apt 4, New York, NY 10001</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:message</span> <span style="color:#ce9178">Hello, world!</span>
<span style="color:#ffd700">}</span>
</code></pre>

## Quick Start

### Syntax at a Glance

<pre><code><span style="color:#6a9955">// Line comments</span>
<span style="color:#6a9955">/* Block comments */</span>

<span style="color:#6a9955">// Objects use { }</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:key</span> <span style="color:#ce9178">value</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:another-key</span> <span style="color:#ce9178">another value</span>
<span style="color:#ffd700">}</span>

<span style="color:#6a9955">// Arrays use [ ]</span>
<span style="color:#ffd700">[</span> <span style="color:#ce9178">first item</span>
<span style="color:#d4d4d4">,</span> <span style="color:#ce9178">second item</span>
<span style="color:#d4d4d4">,</span> <span style="color:#ce9178">third item</span>
<span style="color:#ffd700">]</span>

<span style="color:#6a9955">// Variables</span>
<span style="color:#ffd700">{</span> <span style="color:#c586c0">$base-url</span> <span style="color:#ce9178">https://api.example.com</span>
<span style="color:#ffd700">}</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:endpoint</span> <span style="color:#c586c0">$base-url</span><span style="color:#ce9178">/users</span>
<span style="color:#ffd700">}</span>

<span style="color:#6a9955">// Symbols (like enums)</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:status</span> <span style="color:#4ec9b0">:active</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:role</span> <span style="color:#4ec9b0">:admin</span>
<span style="color:#ffd700">}</span>

<span style="color:#6a9955">// Multiline strings (just indent)</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:description</span>
    <span style="color:#ce9178">This is a multiline string.</span>
    <span style="color:#ce9178">It continues on the next line.</span>
    <span style="color:#ce9178">Commas, like this, are preserved.</span>
<span style="color:#ffd700">}</span>
</code></pre>

### Formatting Rules

SYM is designed for **one key-value pair per line** with **leading commas**:

<pre><code><span style="color:#6a9955">// ✅ Idiomatic SYM</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:name</span> <span style="color:#ce9178">John</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:age</span> <span style="color:#b5cea8">30</span>
<span style="color:#d4d4d4">,</span> <span style="color:#9cdcfe">:city</span> <span style="color:#ce9178">New York</span>
<span style="color:#ffd700">}</span>

<span style="color:#6a9955">// ❌ Avoid inline style</span>
<span style="color:#ffd700">{</span> <span style="color:#9cdcfe">:name</span> <span style="color:#ce9178">John, :age 30</span> <span style="color:#ffd700">}</span>  <span style="color:#6a9955">// This parses as :name with value "John, :age 30"</span>
</code></pre>

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

## Test Suite

A comprehensive language-agnostic test suite ensures consistency across all parser implementations.

### Running Tests

**Rust:**
```bash
cd tests/harness/rust
./run.sh
```

**JavaScript:**
```bash
cd tests/harness/js
npm test
```

**Python:**
Ready for implementation - see [tests/harness/python/](tests/harness/python/)

### Test Coverage

54 test cases covering:
- **Basic types** — Objects, arrays, strings, numbers, booleans, null
- **Strings** — Multiline, special characters, comma handling
- **Complex structures** — Nested objects/arrays, mixed types
- **Variables** — Definition, substitution, scoping
- **Symbols** — Enum-like values, symbol syntax
- **Comments** — Line and block comments
- **Edge cases** — Unicode, whitespace handling
- **Error handling** — 22 invalid input scenarios

See [tests/](tests/) for the complete test suite and [tests/README.md](tests/README.md) for documentation.

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
