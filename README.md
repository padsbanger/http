# http

Simple HTTP server written in Rust.

## How to run

1. Build

```bash
cargo build
```

1. Run

```
./http --host 127.0.0.1 --port 8080 --directory /public
```

## To-do

- ~~Headers in response~~
- Spawn each listener in separate thread
- Implement http2
