rm -rf ./target output.json build
cd ./contract && cargo build --release --target wasm32-unknown-unknown
