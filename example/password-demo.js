console.log('=== Rode.password Module Demo ===')

// Test password hashing and verification
console.log('\nPassword Hashing & Verification:')
const password = 'MySecurePassword123!'

console.log('Original password:', password)

// Hash the password
const hash = Rode.password.hash(password)
console.log('Hash:', hash)

// Verify correct password
const isValid = Rode.password.verify(password, hash)
console.log('Verification (correct):', isValid)

// Verify incorrect password
const isInvalid = Rode.password.verify('WrongPassword', hash)
console.log('Verification (incorrect):', isInvalid)

// Test with different rounds
console.log('\nCustom rounds:')
const hashRounds8 = Rode.password.hash(password, 8)
console.log('Hash (8 rounds):', hashRounds8)
const verifyRounds8 = Rode.password.verify(password, hashRounds8)
console.log('Verification (8 rounds):', verifyRounds8)

// Test password strength
console.log('\nPassword Strength Analysis:')

const passwords = ['password', 'Password123', 'MySecurePassword123!', 'a', 'AbC123!@#$%^&*()', '12345678']

passwords.forEach((pwd) => {
  const strength = Rode.password.strength(pwd)
  console.log(`Password: "${pwd}"`)
  console.log(`  Score: ${strength.score}/100`)
  console.log(`  Level: ${strength.level}`)
  console.log(`  Feedback: ${strength.feedback.join(', ')}`)
  console.log()
})

// Test password generation
console.log('Password Generation:')

// Default password (16 chars, all types)
const generated1 = Rode.password.generate()
console.log('Default:', generated1)

// Custom length
const generated2 = Rode.password.generate(12)
console.log('12 chars:', generated2)

// Custom options
const generated3 = Rode.password.generate(16, {
  lowercase: true,
  uppercase: true,
  numbers: true,
  symbols: false,
})
console.log('No symbols:', generated3)

const generated4 = Rode.password.generate(20, {
  lowercase: true,
  uppercase: true,
  numbers: true,
  symbols: true,
  excludeSimilar: true,
})
console.log('Exclude similar:', generated4)

// Numbers only
const generated5 = Rode.password.generate(8, {
  lowercase: false,
  uppercase: false,
  numbers: true,
  symbols: false,
})
console.log('Numbers only:', generated5)

// Analyze generated passwords
console.log('\nGenerated Password Strength:')
const genPassword = Rode.password.generate(16)
const genStrength = Rode.password.strength(genPassword)
console.log(`Generated: "${genPassword}"`)
console.log(`Strength: ${genStrength.score}/100 (${genStrength.level})`)

console.log('\n=== Demo Complete ===')
