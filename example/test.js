// console.log('Starting server...')

// Rode.http.serve((request) => {
//   console.log(request)

//   return {
//     status: 200,
//     body: 'Hello World!!!!',
//   }
// }, 3000)

// console.log('Server setup complete!')

const CargoToml = Rode.fs.exists('Cargo.toml')
console.log((CargoToml && 'Cargo.toml exists') || 'Cargo.toml does not exist')
