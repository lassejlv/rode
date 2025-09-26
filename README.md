# 🦀 Rode

A fast JavaScript runtime built with Rust and V8, featuring file system operations and HTTP server capabilities.

## Installation

```bash
# Build from source
cargo build --release

# Run directly with Cargo
cargo run -- script.js
```

## Usage

```bash
# Run a JavaScript file once
rode script.js     # JavaScript
rode script.ts     # TypeScript (auto-stripped)

# Run with file watching (auto-restart on changes)
rode --watch script.js
rode -w script.js
```

## API Reference

### File System (`Rode.fs`)

```javascript
// Read file contents
const content = Rode.fs.readFile('config.json')

// Write file contents
Rode.fs.writeFile('output.txt', 'Hello, World!')

// Check if file/directory exists
if (Rode.fs.exists('data.json')) {
  // File exists
}

// Create directories
Rode.fs.mkdir('logs')
Rode.fs.mkdir('nested/deep/path', true) // recursive

// Remove files/directories
Rode.fs.remove('temp.txt')
Rode.fs.remove('temp-dir', true) // recursive

// List directory contents
const entries = Rode.fs.readDir('.')
entries.forEach((entry) => {
  console.log(`${entry.name} - ${entry.isDirectory ? 'DIR' : 'FILE'}`)
})
```

### HTTP Server (`Rode.http`)

```javascript
// Start an HTTP server
Rode.http.serve((request) => {
  console.log(`${request.method} ${request.url}`)

  if (request.url === '/') {
    return {
      status: 200,
      body: 'Hello, World!',
    }
  }

  if (request.url === '/api/users') {
    return {
      status: 200,
      body: JSON.stringify([
        { id: 1, name: 'Alice' },
        { id: 2, name: 'Bob' },
      ]),
    }
  }

  return {
    status: 404,
    body: 'Not Found',
  }
}, 3000) // Port 3000 (default: 8000)
```

### Modules (CommonJS)

```javascript
// math.js - Export functions and constants
export function add(a, b) {
  return a + b
}

export const PI = 3.14159

export default function square(x) {
  return x * x
}

// main.js - Import from other modules
const math = require('./math.js')

console.log('Add:', math.add(5, 3)) // 8
console.log('PI:', math.PI) // 3.14159
console.log('Square:', math.default(4)) // 16
```

### Path Utilities (`Rode.path`)

Cross-platform path manipulation utilities:

```javascript
// Join path segments
const fullPath = Rode.path.join('/home', 'user', 'documents', 'file.txt')
console.log(fullPath) // '/home/user/documents/file.txt'

// Resolve absolute path
const absolutePath = Rode.path.resolve('docs', 'readme.txt')

// Get directory name
const dir = Rode.path.dirname('/home/user/file.txt') // '/home/user'

// Get filename
const filename = Rode.path.basename('/home/user/file.txt') // 'file.txt'
const name = Rode.path.basename('/home/user/file.txt', '.txt') // 'file'

// Get file extension
const ext = Rode.path.extname('archive.tar.gz') // '.gz'

// Check if path is absolute
const isAbs = Rode.path.isAbsolute('/home/user') // true

// Normalize path (resolve . and ..)
const normalized = Rode.path.normalize('./src/../lib/utils.js') // 'lib/utils.js'

// Path constants
console.log(Rode.path.sep) // '/' on Unix, '\' on Windows
console.log(Rode.path.delimiter) // ':' on Unix, ';' on Windows
```

### Console

```javascript
// Standard console output
console.log('Hello', 'World', 42)
```

## Examples

### Simple Script

```javascript
// hello.js
console.log('Hello from Rode!')

const message = 'JavaScript on Rust!'
console.log(message)
```

```bash
rode hello.js
```

### File Operations

```javascript
// file-demo.js
const data = { name: 'Rode', version: '1.0.0' }

// Write JSON file
Rode.fs.writeFile('config.json', JSON.stringify(data, null, 2))

// Read it back
const content = Rode.fs.readFile('config.json')
console.log('File content:', content)

// List current directory
const files = Rode.fs.readDir('.')
console.log(
  'Files:',
  files.map((f) => f.name)
)
```

### HTTP Server

```javascript
// server.js
console.log('Starting server...')

Rode.http.serve((request) => {
  if (request.url === '/') {
    return { status: 200, body: 'Welcome to Rode!' }
  }

  if (request.url === '/time') {
    return {
      status: 200,
      body: JSON.stringify({ time: Date.now() }),
    }
  }

  return { status: 404, body: 'Not Found' }
}, 8080)
```

### Development with Watch Mode

```bash
# Auto-restart server on file changes
rode -w server.js
```

## Features

- **Fast V8 Engine**: Built on Google's V8 JavaScript engine
- **TypeScript Support**: Automatic TypeScript stripping for .ts files
- **File System API**: Complete file and directory operations
- **HTTP Server**: Built-in web server capabilities
- **Module System**: CommonJS-style imports with ES6 export syntax
- **Watch Mode**: Auto-restart on file changes
- **Type Definitions**: Full TypeScript definitions included
- **Zero Dependencies**: No external JavaScript dependencies

## TypeScript Support

Rode automatically strips TypeScript syntax from `.ts` files:

```typescript
// math.ts - TypeScript source
function add(a: number, b: number): number {
  return a + b
}

const result: number = add(5, 3)
console.log('Result:', result)
```

```bash
# Runs automatically with type stripping
rode math.ts
```

**Supported TypeScript Features:**

- Type annotations on variables and functions
- Function parameter and return types
- Interface declarations (removed)
- Type alias declarations (removed)
- Enum declarations (removed)
- **ES6 imports** (converted to CommonJS requires)

**Import Conversion Examples:**

```typescript
// TypeScript ES6 imports
import './module'
import { func } from './utils'
import defaultExport from './lib'

// Automatically becomes CommonJS
require('./module')
const { func } = require('./utils')
const defaultExport = require('./lib')
```

For the best development experience:

1. Include `rode.d.ts` in your project
2. Use an editor with TypeScript support
3. Get full IntelliSense for all Rode APIs

## License

MIT
