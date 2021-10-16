set -e

wasm-pack build --target web --debug

mkdir -p ../wasm/rust
cp -r pkg/* ../wasm/rust/
