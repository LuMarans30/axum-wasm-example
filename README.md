# Axum + WASM Example

A minimal full-stack Rust website using **Axum** and **wasm-bindgen**.

The `wasm` workspace member is compiled to WebAssembly and optimized with `wasm-opt`.  
`wasm-bindgen` generates the JavaScript bindings needed to interface with the WASM module.  
The Axum server serves the generated JS and WASM files, which are loaded and executed in the browser.

## Usage

- `./build_wasm.sh && cargo run --release`

- Visit `http://localhost:3000` 