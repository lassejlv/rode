const command = Rode.args[1].replace('--', '')

async function main() {
  try {
    switch (command) {
      case 'users':
        const users = fetch('https://jsonplaceholder.typicode.com/users')

        const usersJson = users.json()

        Rode.fs.writeFile('users.json', JSON.stringify(usersJson, null, 2))

        break
      default:
        throw new Error(`Unknown command: ${command}`)
    }
  } catch (error) {
    console.error(error)
    Rode.exit(1)
  }
}

main()
