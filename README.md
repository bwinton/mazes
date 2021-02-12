For desktop:
```
cargo run --release
```

For web:
```
cargo build --release --target=wasm32-unknown-unknown && \
cp target/wasm32-unknown-unknown/release/mazes.wasm static && \
basic-http-server static
```