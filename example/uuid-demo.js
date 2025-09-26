console.log('=== Rode.uuid Module Demo ===')

// Generate UUID v4 (random)
console.log('\nUUID v4 (Random):')
const uuid4_1 = Rode.uuid.v4()
const uuid4_2 = Rode.uuid.v4()
console.log('  uuid1:', uuid4_1)
console.log('  uuid2:', uuid4_2)
console.log('  unique:', uuid4_1 !== uuid4_2)

// Generate UUID v1 (time-based)
console.log('\nUUID v1 (Time-based):')
const uuid1_1 = Rode.uuid.v1()
const uuid1_2 = Rode.uuid.v1()
console.log('  uuid1:', uuid1_1)
console.log('  uuid2:', uuid1_2)

// Nil UUID
console.log('\nNil UUID:')
const nilUuid = Rode.uuid.nil()
console.log('  nil:', nilUuid)

// Validate UUIDs
console.log('\nValidation:')
console.log('  valid v4:', Rode.uuid.validate(uuid4_1))
console.log('  valid v1:', Rode.uuid.validate(uuid1_1))
console.log('  valid nil:', Rode.uuid.validate(nilUuid))
console.log('  invalid:', Rode.uuid.validate('not-a-uuid'))
console.log('  invalid format:', Rode.uuid.validate('12345678-1234-1234-1234-123456789abc'))

// Get UUID versions
console.log('\nVersions:')
console.log('  v4 version:', Rode.uuid.version(uuid4_1))
console.log('  v1 version:', Rode.uuid.version(uuid1_1))
console.log('  nil version:', Rode.uuid.version(nilUuid))

// Parse UUIDs (normalize)
console.log('\nParsing:')
try {
  const lowercase = uuid4_1.toLowerCase()
  const parsed = Rode.uuid.parse(lowercase)
  console.log('  lowercase:', lowercase)
  console.log('  parsed:', parsed)
} catch (e) {
  console.log('  parse error:', e.message)
}

try {
  const invalid = 'invalid-uuid'
  const parsed = Rode.uuid.parse(invalid)
  console.log('  invalid parsed:', parsed)
} catch (e) {
  console.log('  expected error:', e)
}

// Practical usage examples
console.log('\nPractical Examples:')

// Database record ID
const recordId = Rode.uuid.v4()
console.log('  Record ID:', recordId)

// Session token
const sessionToken = Rode.uuid.v4()
console.log('  Session Token:', sessionToken)

// File names
const filename = `log_${Rode.uuid.v4()}.txt`
console.log('  Unique Filename:', filename)

// Request tracking
const requestId = Rode.uuid.v1() // Time-based for chronological ordering
console.log('  Request ID:', requestId)

console.log('\n=== Demo Complete ===')
