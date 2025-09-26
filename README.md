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
rode script.js

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
- **File System API**: Complete file and directory operations
- **HTTP Server**: Built-in web server capabilities
- **Watch Mode**: Auto-restart on file changes
- **TypeScript Support**: Full type definitions included
- **Zero Dependencies**: No external JavaScript dependencies

## TypeScript Support

Rode includes TypeScript definitions. For the best development experience:

1. Include `rode.d.ts` in your project
2. Use an editor with TypeScript support
3. Get full IntelliSense for all Rode APIs

## License

MIT
