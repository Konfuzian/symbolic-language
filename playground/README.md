# SYM Playground

An interactive gallery showcasing SYM syntax with live syntax highlighting.

## Usage

The playground is a React component that displays various SYM examples with proper syntax highlighting.

### Running Locally

```bash
# Using Vite
npm create vite@latest sym-playground -- --template react
cd sym-playground
cp ../sym-gallery.jsx src/App.jsx
npm install
npm run dev
```

### Embedding

```jsx
import SymGallery from './sym-gallery';

function App() {
  return <SymGallery />;
}
```

## Features

- **8 real-world examples** covering common use cases
- **Full syntax highlighting** for all SYM token types
- **Dark theme** optimized for code readability
- **Responsive design** works on all screen sizes

## Color Scheme

| Token Type | Color | Example |
|------------|-------|---------|
| Comments | Green | `// comment` |
| Variables | Purple | `$name` |
| Keys | Blue | `:key` |
| Symbols | Teal | `:active` |
| Numbers | Light green | `42` |
| Booleans | Blue | `true` |
| Strings | Orange | `hello` |
| Brackets | Gold | `{ }` |

## Customization

Edit the `colors` object at the top of `sym-gallery.jsx` to customize the theme:

```javascript
const colors = {
  comment: '#6a9955',
  variable: '#c586c0',
  key: '#9cdcfe',
  // ...
};
```
