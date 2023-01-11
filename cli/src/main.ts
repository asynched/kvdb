import readline from 'node:readline'
import { KvClient } from '../../client/src/main'

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
})

const question = (q: string) => {
  return new Promise<string>((resolve) => {
    rl.question(q, (answer) => {
      resolve(answer)
    })
  })
}

const main = async () => {
  const client = new KvClient('127.0.0.1:7890')

  while (true) {
    const command = await question('?> ')

    try {
      const start = Date.now()
      const output = await client.raw(command)
      const end = Date.now()

      console.log(output, `(${end - start}ms)`)
    } catch (err) {
      console.error(err)
    }
  }
}

main()
