import type { ChannelCredentials } from '@grpc/grpc-js'

declare global {
  declare interface CommandRequest {
    command: string
  }

  declare interface CommandResponse {
    output: string
  }

  declare interface CommandClient {
    new (url: string, credentials: ChannelCredentials): CommandClient
    execute(
      command: CommandRequest,
      callback: (err: Error, response: CommandResponse) => unknown
    )
  }

  declare type Value = string | number | null | boolean
}
