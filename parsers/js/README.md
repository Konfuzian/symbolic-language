# SYM JavaScript Syntax Highlighter

A React component for syntax highlighting SYM format code.

## Installation

Copy `sym-highlight.jsx` into your React project.

## Usage

```jsx
import { SymHighlight } from './sym-highlight';

function App() {
  const code = `{ :name Alice
, :age 28
}`;

  return <SymHighlight code={code} />;
}
```

## Props

| Prop | Type | Description |
|------|------|-------------|
| `code` | string | The SYM code to highlight |
| `style` | object | Optional custom styles for the `<pre>` element |

## Color Scheme

| Token Type | Color | Example |
|------------|-------|---------|
| Comments | Green | `// comment` |
| Variables | Purple | `$name` |
| Keys | Light Blue | `:key` |
| Symbols | Teal | `:active` |
| Numbers | Light Green | `42`, `0xff` |
| Booleans | Blue | `true`, `false` |
| Null | Blue | `null` |
| Brackets | Gold | `{`, `}`, `[`, `]` |
| Escapes | Tan | `\$`, `\:` |
| Strings | Orange | `hello world` |

## Demo

Import and render the demo component:

```jsx
import { SymHighlightDemo } from './sym-highlight';

function App() {
  return <SymHighlightDemo />;
}
```
