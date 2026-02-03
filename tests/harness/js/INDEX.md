# JavaScript Test Harness - File Index

Quick navigation guide for the SYM JavaScript test harness.

## Quick Links

- **Want to get started fast?** → Read [QUICKSTART.md](QUICKSTART.md)
- **Need detailed documentation?** → Read [README.md](README.md)
- **Implementing a parser?** → Read [IMPLEMENTATION_NOTES.md](IMPLEMENTATION_NOTES.md)
- **Want to understand the tests?** → Read [TEST_SUITE.md](TEST_SUITE.md)

## File Overview

### Documentation

| File | Purpose | When to Read |
|------|---------|--------------|
| [QUICKSTART.md](QUICKSTART.md) | Get running in 5 minutes | Start here if you want to dive in quickly |
| [README.md](README.md) | Complete documentation | When you need full details about the harness |
| [IMPLEMENTATION_NOTES.md](IMPLEMENTATION_NOTES.md) | Parser implementation guide | When implementing a SYM parser |
| [TEST_SUITE.md](TEST_SUITE.md) | Test suite overview | When you want to understand test coverage |
| [INDEX.md](INDEX.md) | This file | Navigation help |

### Code

| File | Purpose | Status |
|------|---------|--------|
| [test-runner.js](test-runner.js) | Main test harness | ✅ Complete and working |
| [parser-stub.js](parser-stub.js) | Parser interface stub | ⚠️ **Replace with your implementation** |
| [parser-example.js](parser-example.js) | Example parser implementation | 📝 Reference only (incomplete) |
| [package.json](package.json) | npm package configuration | ✅ Ready to use |

### Configuration

| File | Purpose |
|------|---------|
| [.gitignore](.gitignore) | Git ignore rules |

## Getting Started Flow

```
1. Read QUICKSTART.md (5 min)
   ↓
2. Run: npm test (see all tests fail)
   ↓
3. Read IMPLEMENTATION_NOTES.md (15 min)
   ↓
4. Implement your parser (replace parser-stub.js)
   ↓
5. Run: npm test (watch tests pass!)
   ↓
6. Read TEST_SUITE.md to understand remaining failures
   ↓
7. Iterate until all tests pass
```

## Common Tasks

### Run Tests
```bash
npm test                 # Run all tests
npm run test:verbose     # Run with detailed output
node test-runner.js      # Direct execution
```

### View Test Cases
```bash
# See test case structure
ls -R ../../cases/

# View a specific test
cat ../../cases/basic/simple-string/input.sym
cat ../../cases/basic/simple-string/expected.json
```

### Implement Parser

1. Open `parser-stub.js`
2. Replace the stub `parse()` function
3. Keep the same interface:
   ```javascript
   export function parse(input) { /* your code */ }
   export class ParseError extends Error { /* ... */ }
   ```
4. Run tests

### Debug Failures
```bash
# Verbose mode shows full details
node test-runner.js --verbose

# Or add console.log to your parser
```

## Documentation Summary

### QUICKSTART.md (5 min read)
- Step-by-step setup
- Simple examples
- Quick implementation guide
- **Start here!**

### README.md (10 min read)
- Complete test harness documentation
- Parser interface requirements
- Test case structure
- Output format
- All usage details

### IMPLEMENTATION_NOTES.md (15 min read)
- Technical implementation details
- Parsing strategies
- Common pitfalls
- Debugging tips
- Reference to Rust implementation

### TEST_SUITE.md (5 min read)
- 54 test cases across 10 categories
- Test statistics and breakdown
- What each test validates
- Adding new tests

## Parser Implementation Status

| Feature | Implemented | Tests Passing |
|---------|-------------|---------------|
| Empty structures | ❌ | 0/2 |
| Basic values | ❌ | 0/5 |
| Objects | ❌ | 0/9 |
| Arrays | ❌ | 0/4 |
| Comments | ❌ | 0/3 |
| Numbers | ❌ | 0/3 |
| Strings | ❌ | 0/5 |
| Symbols | ❌ | 0/3 |
| Variables | ❌ | 0/3 |
| Error handling | ❌ | 0/22 |
| **Total** | **0%** | **0/54** |

> Update this table as you implement features!

## Test Categories Quick Reference

```
tests/cases/
├── arrays/ (4 tests)
│   ├── array-of-objects/
│   ├── mixed-array/
│   ├── nested-arrays/
│   └── simple-array/
├── basic/ (7 tests)
│   ├── empty-array/
│   ├── empty-object/
│   ├── multiple-keys/
│   ├── simple-boolean/
│   ├── simple-null/
│   ├── simple-number/
│   └── simple-string/
├── comments/ (3 tests)
├── edge-cases/ (2 tests)
├── errors/ (22 tests)
├── numbers/ (3 tests)
├── objects/ (2 tests)
├── strings/ (5 tests)
├── symbols/ (3 tests)
└── variables/ (3 tests)
```

## External Resources

- **SYM Specification**: `../../../spec/SPEC.md`
- **Reference Implementation**: `../../../parsers/rust/`
- **Examples**: `../../../examples/`
- **Syntax Highlighter**: `../../../parsers/js/sym-highlight.jsx`

## Need Help?

1. **Parser interface unclear?** → See [README.md](README.md#parser-implementation)
2. **Tests failing?** → Run with `--verbose` flag
3. **Don't know where to start?** → See [QUICKSTART.md](QUICKSTART.md#example-implementing-basic-objects)
4. **Stuck on a feature?** → Check [IMPLEMENTATION_NOTES.md](IMPLEMENTATION_NOTES.md)
5. **Want to understand a test?** → Look at the test files in `tests/cases/`

## Project Structure

```
symbolic-language/
├── spec/
│   └── SPEC.md                    # Language specification
├── parsers/
│   ├── rust/                      # Reference parser
│   └── js/
│       └── sym-highlight.jsx      # Syntax highlighter
├── tests/
│   ├── cases/                     # Test cases (54 total)
│   └── harness/
│       ├── rust/
│       ├── python/
│       └── js/                    # ← You are here
│           ├── README.md
│           ├── QUICKSTART.md
│           ├── IMPLEMENTATION_NOTES.md
│           ├── TEST_SUITE.md
│           ├── INDEX.md           # ← This file
│           ├── test-runner.js
│           ├── parser-stub.js     # ← Replace this
│           └── parser-example.js
└── examples/                      # Real-world examples
```

## Contributing

When you've implemented a parser:
1. Ensure all tests pass: `npm test`
2. Consider contributing your parser back to the project
3. Share your implementation approach!

## License

MIT - See project root for details
