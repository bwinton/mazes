For desktop:
```
cargo run --release
```

For web:
```
cargo build --release --target=wasm32-unknown-unknown && \
mkdir -p target/deploy &&
cp target/wasm32-unknown-unknown/release/mazes.wasm target/deploy && \
cp static/* target/deploy && \
basic-http-server target/deploy
```