# Using Claude with SYM Format

This guide shows how to effectively ask Claude to work with SYM files.

## Quick Start

When working with this repository, Claude has access to:
- [CLAUDE.md](CLAUDE.md) - Comprehensive guide for AI assistants
- [CHEATSHEET.md](CHEATSHEET.md) - Quick syntax reference
- [SPEC.md](../spec/SPEC.md) - Complete language specification
- [examples/](../examples/) - Real-world SYM files

## Example Prompts

### Reading and Understanding SYM

```
"Read the config.sym file and explain what it configures"

"What database settings are defined in app-config.sym?"

"Show me all the variables defined in this SYM file"

"What's the difference between these two SYM configuration files?"
```

### Writing SYM Files

```
"Convert this JSON to SYM format: { ... }"

"Create a SYM config file for a web server with database, Redis, and auth settings"

"Write a Kubernetes deployment in SYM format"

"Convert my package.json to SYM format"
```

### Editing SYM Files

```
"Add a new database connection to the config"

"Update the API endpoint URL to use production domain"

"Add a new feature flag called 'dark-mode' set to true"

"Extract repeated URLs into variables"
```

### Validation and Fixes

```
"Check if this SYM file is valid"

"Fix the syntax errors in my SYM file"

"This isn't parsing correctly, can you help?"

"Validate that all variables are defined"
```

### Converting Formats

```
"Convert this YAML file to SYM"

"Show me this config in both JSON and SYM"

"Rewrite this TOML config as SYM"

"Convert my .env file to a SYM format"
```

### Advanced Operations

```
"Split this large config into base.sym and production.sym using imports"

"Show me how to merge these two SYM configs"

"Add override modifiers where the production config should replace base values"

"Optimize this SYM file by extracting common values into variables"
```

## Best Practices for Prompts

### Be Specific About Style

```
✅ "Write a SYM config using variables for repeated domains and ports"
✅ "Convert to SYM with leading comma style"
✅ "Create a SYM file with one key-value per line"

❌ "Make a config file"
❌ "Convert this"
```

### Provide Context

```
✅ "This is a production config for a Node.js API. Convert it to SYM."
✅ "Create a development environment config in SYM format with local database and debug mode enabled"

❌ "Write a config"
```

### Ask for Explanation

```
✅ "Explain the structure of this SYM file and what each section does"
✅ "Walk me through how the variables are used in this config"
✅ "What would happen if I imported base.sym into this file?"

❌ "What does this do?"
```

## Common Workflows

### 1. Converting Existing Config

```
You: "I have this JSON config file. Can you convert it to SYM format?"

Claude will:
1. Read your JSON
2. Identify repeated values for variables
3. Suggest symbols for enum-like values
4. Convert to idiomatic SYM style
5. Explain the changes
```

### 2. Creating New Config

```
You: "I need a config for a microservice with:
- PostgreSQL database
- Redis cache
- JWT auth
- Different settings for dev/prod environments"

Claude will:
1. Design a structure with variables
2. Use symbols for enums (:dev, :prod)
3. Show import pattern for environment configs
4. Include common settings and best practices
```

### 3. Refactoring Config

```
You: "This config.sym is getting large. Help me split it into base and environment-specific files."

Claude will:
1. Identify common settings for base.sym
2. Create environment-specific overrides
3. Use @import directives
4. Add `!` modifier where replacement is needed
5. Ensure variables are properly scoped
```

### 4. Debugging Syntax

```
You: "My SYM file isn't parsing. Here's the error: ..."

Claude will:
1. Identify the syntax issue
2. Explain what's wrong
3. Show the corrected version
4. Point to relevant spec section
```

## Example Session

```
You: "Create a SYM config for a web app with database and Redis"

Claude: "I'll create a SYM configuration file for a web application
with PostgreSQL database and Redis cache:

{ $domain myapp.com
, $db-host db.production.myapp.com
}

{ :app
  { :name my-web-app
  , :version 1.0.0
  , :domain $domain
  , :port 3000
  }
, :database
  { :host $db-host
  , :port 5432
  , :name myapp_db
  , :pool-size 20
  , :ssl true
  }
, :redis
  { :host redis.production.myapp.com
  , :port 6379
  , :db 0
  , :ttl 3600
  }
, :features
  [ :auth
  , :logging
  , :metrics
  ]
}

This config uses:
- Variables ($domain, $db-host) for repeated values
- Nested objects for organization
- Symbols in :features array for feature flags
- Leading comma style for easy editing
"

You: "Can you add a separate dev environment version?"

Claude: "I'll create a base config and a dev override...
[Shows base.sym and dev.sym with @import]"
```

## Tips for Working with Claude

1. **Reference files by name**: "Read app-config.sym"
2. **Ask for explanations**: "Why did you use a symbol here?"
3. **Request alternatives**: "Show me two ways to structure this"
4. **Validate understanding**: "Does this SYM file match the JSON version?"
5. **Iterate**: "Now add caching configuration"

## Common Questions

**Q: How does Claude know SYM syntax?**
A: Claude can read [CLAUDE.md](CLAUDE.md) and examples to understand the format.

**Q: Can Claude validate SYM files?**
A: Yes, Claude can check syntax, structure, and variable references.

**Q: Will Claude use the Rust parser?**
A: Claude can run `sym-parser` commands if the tool is installed.

**Q: Can Claude help migrate from JSON/YAML?**
A: Yes, Claude can convert and explain the migration process.

**Q: Does Claude understand import merging?**
A: Yes, Claude knows the merge rules and the `!` modifier.

## Getting Help

If Claude makes a mistake or you're unsure:

1. **Ask for clarification**: "Explain why you used `:key!` instead of `:key`"
2. **Request validation**: "Can you verify this against the spec?"
3. **Check examples**: "Show me a similar example from the examples/ folder"
4. **Reference the spec**: "What does the spec say about multiline strings?"

## Advanced Patterns

### Multi-Environment Configs

```
"Create a base config with imports for dev, staging, and production.
Use variables for environment-specific values."
```

### Config Migration

```
"I have 5 JSON config files. Help me convert them to SYM with shared
variables and imports to avoid duplication."
```

### Template Generation

```
"Generate a template SYM config for a typical Express.js app with
comments explaining each section."
```

### Validation Rules

```
"Check if all required variables are defined and all modifiers
are used correctly."
```

## Next Steps

- Read [CLAUDE.md](CLAUDE.md) for comprehensive guide
- See [examples/](../examples/) for real-world patterns
- Check [SPEC.md](../spec/SPEC.md) for detailed rules
- Try [CHEATSHEET.md](CHEATSHEET.md) for quick reference
