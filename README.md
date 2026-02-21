# Contest

Contest is a language-comparison playground where the same CLI tool (`projector`) is implemented in Go, Rust, and TypeScript.

## Core Idea

`projector` stores key-value pairs per directory and supports hierarchical lookup:

- current directory values override parent directory values
- parent values are inherited when not defined locally

This makes it a good small benchmark for API design, error handling, and implementation style across languages.

## Repository Layout

- `cliApp/go` - Go implementation
- `cliApp/rust` - Rust implementation
- `cliApp/ts` - TypeScript implementation
- `basic/` and `adventOfCode/` - additional language exercises

## CLI Operations

All implementations support:

- print all values: `projector`
- print a key: `projector <key>`
- set key/value: `projector add <key> <value>`
- remove key: `projector rm <key>`

## Run Each Implementation

Use the same config file path for easier comparison.

```bash
CONFIG="$(pwd)/projector.json"
```

### Go

```bash
cd cliApp/go
go run cmd/cli/main.go --config "$CONFIG" add foo bar
go run cmd/cli/main.go --config "$CONFIG" foo
go run cmd/cli/main.go --config "$CONFIG"
```

### Rust

```bash
cd cliApp/rust
cargo run --bin projector -- --config "$CONFIG" add foo bar
cargo run --bin projector -- --config "$CONFIG" foo
cargo run --bin projector -- --config "$CONFIG"
```

### TypeScript

```bash
cd cliApp/ts
npm install
npx ts-node src/index.ts --config "$CONFIG" add foo bar
npx ts-node src/index.ts --config "$CONFIG" foo
npx ts-node src/index.ts --config "$CONFIG"
```

## Why This Project

- Compare developer ergonomics across Go/Rust/TS
- Validate behavior consistency with identical operations
- Use as a base for extending into perf and concurrency experiments
