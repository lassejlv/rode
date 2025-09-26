console.log('=== Rode.path Module Demo ===')

// path.join - combine path segments
console.log('path.join:')
console.log('  join("a", "b", "c"):', Rode.path.join('a', 'b', 'c'))
console.log('  join("/home", "user", "docs"):', Rode.path.join('/home', 'user', 'docs'))
console.log('  join("./src", "../lib", "utils.js"):', Rode.path.join('./src', '../lib', 'utils.js'))

// path.resolve - resolve absolute path
console.log('\npath.resolve:')
console.log('  resolve("docs", "readme.txt"):', Rode.path.resolve('docs', 'readme.txt'))
console.log('  resolve("/tmp", "test.js"):', Rode.path.resolve('/tmp', 'test.js'))

// path.dirname - get directory
console.log('\npath.dirname:')
console.log('  dirname("/home/user/file.txt"):', Rode.path.dirname('/home/user/file.txt'))
console.log('  dirname("./src/main.js"):', Rode.path.dirname('./src/main.js'))

// path.basename - get filename
console.log('\npath.basename:')
console.log('  basename("/home/user/file.txt"):', Rode.path.basename('/home/user/file.txt'))
console.log('  basename("/home/user/file.txt", ".txt"):', Rode.path.basename('/home/user/file.txt', '.txt'))
console.log('  basename("main.js"):', Rode.path.basename('main.js'))

// path.extname - get extension
console.log('\npath.extname:')
console.log('  extname("file.txt"):', Rode.path.extname('file.txt'))
console.log('  extname("archive.tar.gz"):', Rode.path.extname('archive.tar.gz'))
console.log('  extname("README"):', Rode.path.extname('README'))

// path.isAbsolute - check if absolute
console.log('\npath.isAbsolute:')
console.log('  isAbsolute("/home/user"):', Rode.path.isAbsolute('/home/user'))
console.log('  isAbsolute("./relative"):', Rode.path.isAbsolute('./relative'))
console.log('  isAbsolute("file.txt"):', Rode.path.isAbsolute('file.txt'))

// path.normalize - normalize path
console.log('\npath.normalize:')
console.log('  normalize("./src/../lib/utils.js"):', Rode.path.normalize('./src/../lib/utils.js'))
console.log('  normalize("/home/user//docs/./file.txt"):', Rode.path.normalize('/home/user//docs/./file.txt'))

// path constants
console.log('\npath constants:')
console.log('  path.sep:', JSON.stringify(Rode.path.sep))
console.log('  path.delimiter:', JSON.stringify(Rode.path.delimiter))

console.log('\n=== Demo Complete ===')
