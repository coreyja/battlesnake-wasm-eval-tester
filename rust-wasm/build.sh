set -e

wasm-pack build --target web

mkdir -p ../wasm/rust
cp -r pkg/* ../wasm/rust/
