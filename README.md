<p align="center">
  <img src="assets/logo.svg" alt="SYM - Write config, not syntax." width="400">
</p>

**[Try it →](https://konfuzian.github.io/symbolic-language/)**

```sym
{ :address 123 Main St, Apt 4, New York, NY 10001
, :message Hello, world!
}
```

No escaping. No quotes. The comma in "Apt 4, New York" is preserved as-is.

## How it works

Only `newline + comma` separates entries. Commas anywhere else are just text.

```sym
{ :name my-app
, :version 1.0.0
, :features
  [ dark-mode
  , notifications
  ]
}
```

## The basics

```sym
// Objects
{ :key value
, :another another value
}

// Arrays
[ first item
, second item
]

// Variables
{ $api https://api.example.com
, :endpoint $api/users
}

// Symbols (enums)
{ :status :active }

// Multiline (just indent)
{ :bio
    Software engineer.
    Likes coffee, code, and commas.
}

// Comments
// line comment
/* block comment */
```

## Install

**Rust CLI:**
```bash
cd parsers/rust && cargo install --path .
sym-parser config.sym          # parse
sym-parser config.sym --json   # output JSON
sym-parser --from-json in.json # convert from JSON
```

## Docs

- [Cheatsheet](docs/CHEATSHEET.md) — 2 min syntax reference
- [Full Spec](spec/SPEC.md) — Complete language reference
- [Examples](examples/) — Real configs (Docker, K8s, package.json, etc.)

## Why not JSON/YAML/TOML?

**JSON**: No comments, quotes everywhere, trailing comma errors.

**YAML**: Indentation-sensitive in subtle ways. [Norway problem](https://hitchdev.com/strictyaml/why/implicit-typing-removed/). Anchors are confusing.

**TOML**: Great for flat configs, awkward for deep nesting.

**SYM**: Brackets make structure explicit. Commas in values are safe. Variables built-in.

## License

MIT
