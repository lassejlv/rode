// import { add } from './math.js'

// const result = add(1, 2)
// console.log(result)

const rodeTypescriptTypes = Rode.fs.readFile('example/rode.d.ts')

const path = Rode.path.relative('text.txt', 'text2.txt')
console.log(path)
