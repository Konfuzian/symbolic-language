# SYM Examples

Real-world configuration examples demonstrating SYM's capabilities.

## Files

| File | Description |
|------|-------------|
| `simple-config.sym` | Basic application configuration |
| `docker-compose.sym` | Docker Compose service stack |
| `kubernetes-deployment.sym` | Kubernetes deployment manifest |
| `package.sym` | Node.js package.json equivalent |
| `github-actions.sym` | GitHub Actions CI/CD pipeline |
| `app-config.sym` | Production application config with database, Redis, auth |
| `api-response.sym` | E-commerce order API response |
| `tailwind-config.sym` | Tailwind CSS configuration |
| `eslint-config.sym` | ESLint + TypeScript configuration |
| `aws-infrastructure.sym` | Terraform-like AWS infrastructure |
| `comprehensive.sym` | All SYM features in one file |

## Style Guide

These examples follow SYM's idiomatic style:

1. **One key-value per line**
2. **Leading commas** for easy reordering and diffs
3. **2-space indentation** for nested structures
4. **Closing brackets on their own line**

```sym
// ✅ Good
{ :name my-app
, :version 1.0.0
, :config
  { :debug true
  , :port 8080
  }
}

// ❌ Avoid
{ :name my-app, :version 1.0.0 }
```

## Features Demonstrated

- **Variables** — DRY configuration with `$var` substitution
- **Symbols** — Enum-like values with `:keyword` syntax
- **Multiline strings** — Natural text without escaping
- **Deep nesting** — Clear structure at any depth
- **Mixed types** — Arrays of objects, nested maps, etc.
