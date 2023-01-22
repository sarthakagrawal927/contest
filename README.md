# Contest

Started off from a course. Current contenders:
- Rust
- Golang
- TS

```bash
cd cliApp

#rust
cd rust
cargo run --bin  projector -- --config $(pwd)/conf.json

#ts
cd ts
npx ts-node src/index.ts --config $(pwd)/conf.json

#go
cd go
go run cmd/cli/main.go --config $(pwd)/conf.json add foo bar
```

## My Review

Golang is easy for the kind of performance it provides, probably great for backend services.
Whereas Rust being the fastest but hard to write will apparently be good at system level programming & CLI apps. Not so much use case for backend development. So probably not picking till June'23. Will try to master Golang & Elixir, it seems nice.

## Todo in future

GO [Go-routines, contexts] vs NodeJS [Cluster & workers]