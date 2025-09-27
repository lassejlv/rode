# 🦀 Rode

A fast JavaScript runtime built with Rust and V8, featuring file system operations and HTTP server capabilities.

## Installation

```bash
# Build from source
cargo build --release

# Run directly with Cargo
cargo run -- script.js
```

## Environment Configuration

Rode automatically loads environment variables from `.env` files before starting. This happens before any script execution, making environment variables immediately available.

**File precedence:** `.env.local` → `.env`

### .env File Format

```bash
# Comments start with #
APP_NAME=My Application
NODE_ENV=development

# Variable expansion
BASE_URL=http://localhost:3000
API_URL=${BASE_URL}/api

# Quoted values (support escape sequences)
SECRET="my secret key"
MULTILINE="Line 1\nLine 2\tTabbed"

# Single quotes (no expansion or escaping)
LITERAL='${HOME} stays as literal text'

# System variable expansion
LOG_FILE=/var/log/${APP_NAME}.log
CONFIG_PATH=${HOME}/.config/myapp
```

**Supported features:**

- Comments with `#`
- Variable expansion: `${VAR}` and `$VAR`
- Double quotes with escape sequences (`\n`, `\r`, `\t`, `\\`, `\"`)
- Single quotes (literal, no expansion)
- System environment variable references

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

### Password Security (`Rode.password`)

Secure password hashing, verification, and generation:

```javascript
// Hash a password with bcrypt
const password = 'MySecurePassword123!'
const hash = Rode.password.hash(password)
console.log(hash) // '$2b$12$...'

// Verify password against hash
const isValid = Rode.password.verify(password, hash)
console.log(isValid) // true

// Custom rounds (higher = more secure but slower)
const strongHash = Rode.password.hash(password, 15)

// Analyze password strength
const strength = Rode.password.strength('weak123')
console.log(strength.score) // 45
console.log(strength.level) // 'fair'
console.log(strength.feedback) // ['Add symbols', 'Use more characters']

// Generate secure passwords
const randomPassword = Rode.password.generate()
console.log(randomPassword) // 'rM}09s=qP!FS*I=q'

// Custom generation options
const customPassword = Rode.password.generate(12, {
  lowercase: true,
  uppercase: true,
  numbers: true,
  symbols: false,
  excludeSimilar: true,
})
console.log(customPassword) // 'AbCdEfGh2345'
```

### UUID Generation (`Rode.uuid`)

UUID generation and manipulation utilities:

```javascript
// Generate random UUID v4
const id = Rode.uuid.v4()
console.log(id) // 'f47ac10b-58cc-4372-a567-0e02b2c3d479'

// Generate time-based UUID v1
const timeId = Rode.uuid.v1()
console.log(timeId) // '6c84fb90-12c4-11e1-840d-7b25c5ee775a'

// Generate time-ordered UUID v7 (recommended for new applications)
const v7Id = Rode.uuid.v7()
console.log(v7Id) // '01890a5b-def0-7000-8000-123456789abc'

// Get nil UUID (all zeros)
const nil = Rode.uuid.nil()
console.log(nil) // '00000000-0000-0000-0000-000000000000'

// Validate UUID format
const isValid = Rode.uuid.validate('f47ac10b-58cc-4372-a567-0e02b2c3d479')
console.log(isValid) // true

// Get UUID version
const version = Rode.uuid.version('f47ac10b-58cc-4372-a567-0e02b2c3d479')
console.log(version) // 4

// Parse and normalize UUID
const normalized = Rode.uuid.parse('f47ac10b-58cc-4372-a567-0e02b2c3d479')
console.log(normalized) // 'F47AC10B-58CC-4372-A567-0E02B2C3D479'

// Practical usage
const recordId = Rode.uuid.v4()
const sessionToken = Rode.uuid.v4()
const filename = `log_${Rode.uuid.v4()}.txt`
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

### Interactive Input

```javascript
// Get user input from the console
const name = prompt('What is your name?')
console.log(`Hello, ${name}!`)

// Prompt with default value
const age = prompt('What is your age?', '25')
console.log(`You are ${age} years old`)

// Empty prompt (just shows >)
const input = prompt()

// Interactive menu example
const choice = prompt('Choose option (1-3)', '1')
switch (choice) {
  case '1':
    console.log('Option 1 selected')
    break
  case '2':
    console.log('Option 2 selected')
    break
  case '3':
    console.log('Option 3 selected')
    break
  default:
    console.log('Invalid choice')
}

// Yes/No confirmation
const confirmed = alert('Continue with operation?') // Shows: Continue with operation? (Y/n):
if (confirmed) {
  console.log('User confirmed')
} else {
  console.log('User cancelled')
}
```

### Enhanced Console

```javascript
// Standard logging with colors
console.log('Normal message')
console.info('Info message (blue)')
console.warn('Warning message (yellow)')
console.error('Error message (red)')

// Display data in tables
const users = [
  { name: 'John', age: 30 },
  { name: 'Jane', age: 25 },
]
console.table(users)

// Object inspection
console.dir({ nested: { data: 'value' } })

// Timing operations
console.time('operation')
// ... some work
console.timeEnd('operation') // Shows elapsed time

// Counting
console.count('iterations') // iterations: 1
console.count('iterations') // iterations: 2

// Clear screen
console.clear()
```

### Process & Environment

```javascript
// Access command line arguments
console.log('Script arguments:', Rode.args) // ['arg1', 'arg2']
console.log('All arguments:', Rode.argv) // ['rode', 'script.js', 'arg1', 'arg2']

// Environment variables
console.log('Home directory:', Rode.env.HOME)
console.log('PATH:', Rode.env.PATH)
console.log('All env vars:', Rode.env)

// Exit the process
if (someCondition) {
  Rode.exit(1) // Exit with code 1
}
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
