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

## Todo in future

GO [Go-routines, contexts] vs NodeJS [Cluster & workers]