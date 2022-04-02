# Go for Rust RPC

Simple service to communicate between GOLANG and RUST via RPC

## Usage

- proto build 
``` bash
cargo build --manifest-path=rust/Cargo.toml
protoc --go-grpc_out=. --go-grpc_opt=require_unimplemented_servers=false --go_out=. proto/*.proto
```

- running project

```bash
 cargo run --manifest-path=rust/Cargo.toml --bin server # listeing rpc server
 cd golang && go run cmd/client.go #for client

 # or

 cd golang && go run cmd/server.go #for golang server
 cargo run --manifest-path=rust/Cargo.toml --bin client
```
