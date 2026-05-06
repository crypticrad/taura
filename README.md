# taura

A Unix shell implementation written in Rust.

## Features

- **REPL loop** — interactive prompt that continuously reads and executes commands
- **Built-in commands**
  - `echo <args>` — print arguments to stdout
  - `type <command>` — identify whether a command is a shell builtin or an executable, and show its path
  - `exit` — exit the shell
- **External command execution** — resolves executables from `PATH`, checks execute permissions, and spawns them with correct `argv[0]`

## Build

```sh
cargo build --release
```

## Run

```sh
cargo run
```

## Usage

```
$ echo hello world
hello world
$ type grep
grep is /usr/bin/grep
$ type exit
exit is a shell builtin
$ grep --help
...
$ exit
```
