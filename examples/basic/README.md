# Basic example

## Running the example

```bash
$ cargo run -p basic_example
```

## Compile and run in browser

Run these commands in the current directory:

```bash
cargo install wasm-bindgen-cli https

cargo build --target wasm32-unknown-unknown

wasm-bindgen ../../target/wasm32-unknown-unknown/debug/basic_example.wasm --out-dir . --target web --no-typescript

http
```