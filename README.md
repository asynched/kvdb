# kvdb

KVDB is a key-value store written in Rust using GRPC as the communication layer between applications. The database is similar to any other key-value database (Redis or Memcached), it includes commands such as `GET`, `SET`, `EXISTS` and so on. One caveat is that the database does not implement `TTL` for keys, so they are not evicted.

## About

The database can be used as a middleware between your application and your database, one use-case it to use it to store the result of expensive queries in your database, since kvdb stores values in `RAM` is tends to be faster than typical in-disk databases.

The database can store multiple data-types inside it, such as:

- strings
- integers (64-bit)
- floats (64-bit)
- booleans

## Commands

The database supports the following commands:

- SET
- GET
- DEL
- EXISTS
- INCR
- FLUSHALL

### SET

The `SET` command is used to store a value in the database.

```
?> SET name "Foo bar"
OK
```

### GET

The `GET` command is used to retrieve a value from the database.

```
?> GET name
"Foo bar"
```

### DEL

The `DEL` command is used to delete a key from the database

```
?> DEL name
OK
```

### EXISTS

The `EXISTS` command can be used to check if a key exists in the database.

```
?> EXISTS name
true
```

### INCR

The `INCR` command can be used to increment a value in the database.

```
?> INCR counter
1
```

In this case, if the counter value isn't found it'll create it and set it to `1`. If the datatype of the key isn't an int it'll return an error.

### FLUSHALL

The flushall command is used to flush all the keys in the database.

```
?> FLUSHALL
OK
```

## How to run?

To run the application, you'll need the following tools installed

- Cargo
- rustc
- Node

### Running

You can start the application by using the following command:

```sh
cargo run
```

Or you can start it in release mode using:

```sh
cargo run --release
```

### CLI arguments

The application supports the following arguments:

- Shards - Number of shards to use
- Addr - Network address for the server
- Reserve - Amount of memory to reserve for the database

> Note: Every argument has a default value and you can get help by adding the `--help` flag.

## Client

The client application is located inside the `client` folder in the root directory and it is the middle-man between the server and any other application that interacts with the server.

### Installation

To install the client application you'll need to enter it's directory and install the dependencies with a node package manager (I'm using PNPM in this case).

To install the dependencies you can run:

```sh
pnpm i # Or npm i if you're using NPM
```

## CLI

The database comes with a command-line interface for interacting with the server. The CLI is located inside the `cli` folder in the root directory. To run it you'll need to do the server and client setup (with the instructions mentioned above).

### Installation

To install the CLI you'll need to enter it's directory and install the dependencies as well, you can do that by running:

```sh
pnpm i # Or npm i if you're using NPM
```

And then you can run the client with:

```sh
pnpm cli # Or npm run cli if you're using NPM
```

## Technologies

- [Rust](https://www.rust-lang.org/)

  - [Tokio](https://tokio.rs/)
  - [Hyper](https://hyper.rs/)
  - [Axum](https://github.com/tokio-rs/axum)
  - [Tonic](https://github.com/hyperium/tonic)
  - [GRPC](https://grpc.io/)
  - [Clap](https://github.com/clap-rs/clap)

- [Node](https://https://nodejs.org/)
  - [Typescript](https://typescriptlang.org/)
  - [GRPC](https://grpc.io/)

## Author

| ![Eder Lima](https://github.com/asynched.png?size=100) |
| ------------------------------------------------------ |
| [Eder Lima](https://github.com/asynched)               |
