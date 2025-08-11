cargo install wasm-bindgen-cli wasm-opt
rm -r ./pkg
echo "Compiling Rust to WASM..."
cargo build --release --target wasm32-unknown-unknown --package wasm
echo "Generating WASM bindings..."
wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm.wasm --out-dir pkg --target web
echo "WASM has been generated!"
wasm-opt pkg/wasm_bg.wasm -o pkg/wasm_bg.wasm-opt.wasm -O
echo "WASM has been optimized!"
mv pkg/wasm_bg.wasm-opt.wasm pkg/wasm_bg.wasm
echo "Optimized WASM file available at ./pkg/wasm_bg.wasm"