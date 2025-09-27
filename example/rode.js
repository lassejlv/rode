console.log(Rode.env.DATABASE_URL)

const contentForFile = prompt('Enter the content for the file')
const filename = prompt('Enter the filename')

const confirmation = alert('Are you sure you want to write to the file?')

if (!confirmation) {
  console.log('File not written')
  Rode.exit(1)
}

Rode.fs.writeFile(filename, contentForFile)

console.log('File written')
