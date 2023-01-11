import * as path from 'node:path'
import * as protoLoader from '@grpc/proto-loader'
import { credentials, loadPackageDefinition } from '@grpc/grpc-js'

const proto = protoLoader.loadSync(
  path.resolve(__dirname, '..', '..', 'proto', 'commands.proto')
)

const descriptor = loadPackageDefinition(proto)

const rpc = descriptor.rpc
const Command = (rpc as any).Command as CommandClient

export class KvClient {
  private client: CommandClient

  constructor(addr: string) {
    this.client = new Command(addr, credentials.createInsecure())
  }

  private intoValue(str: string): Value {
    if (str.startsWith('n:')) {
      return null
    }

    if (str.startsWith('s:')) {
      return str.slice(2)
    }

    if (str.startsWith('i:')) {
      return parseInt(str.slice(2))
    }

    if (str.startsWith('f:')) {
      return parseFloat(str.slice(2))
    }

    if (str.startsWith('b:')) {
      return str.slice(2) === 'true'
    }

    if (str === 'OK') {
      return null
    }

    throw new Error(`Could not parse value ${str}`)
  }

  get(key: string): Promise<Value> {
    return new Promise<Value>((resolve, reject) => {
      const cmd = {
        command: `GET ${key}`,
      }

      this.client.execute(cmd, (err, response) => {
        if (err) {
          return reject(err)
        }

        return resolve(this.intoValue(response.output))
      })
    })
  }

  set(key: string, value: Value): Promise<void> {
    if (typeof value === 'string') {
      value = `"${value}"`
    }

    return new Promise<void>((resolve, reject) => {
      const cmd = {
        command: `SET ${key} ${value}`,
      }

      this.client.execute(cmd, (err) => {
        if (err) {
          return reject(err)
        }

        resolve()
      })
    })
  }

  del(key: string): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      const cmd = {
        command: `DEL ${key}`,
      }

      this.client.execute(cmd, (err) => {
        if (err) {
          return reject(err)
        }

        resolve()
      })
    })
  }

  incr(key: string): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      const cmd = {
        command: `INCR ${key}`,
      }

      this.client.execute(cmd, (err) => {
        if (err) {
          return reject(err)
        }

        resolve()
      })
    })
  }

  exists(key: string): Promise<boolean> {
    return new Promise<boolean>((resolve, reject) => {
      const cmd = {
        command: `EXISTS ${key}`,
      }

      this.client.execute(cmd, (err, response) => {
        if (err) {
          return reject(err)
        }

        resolve(this.intoValue(response.output) as boolean)
      })
    })
  }

  flushall(): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      const cmd = {
        command: 'FLUSHALL',
      }

      this.client.execute(cmd, (err) => {
        if (err) {
          return reject(err)
        }

        resolve()
      })
    })
  }

  raw(command: string): Promise<string> {
    return new Promise<string>((resolve, reject) => {
      const cmd = {
        command,
      }

      this.client.execute(cmd, (err, response) => {
        if (err) {
          return reject(err)
        }

        return resolve(response.output)
      })
    })
  }
}
