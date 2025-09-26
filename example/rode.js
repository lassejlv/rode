// // import { add } from './math.js'

// // const result = add(1, 2)
// // console.log(result)

// const rodeTypescriptTypes = Rode.fs.readFile('example/rode.d.ts')

// const path = Rode.path.relative('text.txt', 'text2.txt')
// console.log(path)

// console.log(Rode.uuid.v7())
// console.log(Rode.fs.readDir('example'))

const resp = 0 == '0'

console.log(resp)

const pass = Rode.password.generate()
const hash = Rode.password.hash(pass)
console.log(hash)
console.log(Rode.password.verify(pass + '1', hash))
