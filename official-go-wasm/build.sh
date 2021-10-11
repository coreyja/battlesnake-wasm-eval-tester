set -e

GOOS=js GOARCH=wasm go build -o ../wasm/official.wasm
