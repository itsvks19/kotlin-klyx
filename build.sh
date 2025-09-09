#!/bin/bash
set -e

rm -rf output/
mkdir -p output/

cargo build --target wasm32-wasip1 --release

WASM_FILE_NAME=$(grep '^name = ' Cargo.toml | cut -d '"' -f 2 | sed 's/[ -]/_/g')
WASM_FILE="target/wasm32-wasip1/release/${WASM_FILE_NAME}.wasm"

[ -f "$WASM_FILE" ] || { echo "Error: $WASM_FILE not found"; exit 1; }

mkdir -p output/temp/src
cp "$WASM_FILE" output/temp/src/
cp extension.toml output/temp/

mkdir -p lib
cp "$WASM_FILE" lib/

EXTENSION_ID=$(grep '^id = ' extension.toml | cut -d '"' -f 2)
EXTENSION_VERSION=$(grep '^version = ' extension.toml | cut -d '"' -f 2)
ZIP_NAME="${EXTENSION_ID}-${EXTENSION_VERSION}.zip"

(cd output/temp && zip -r "../${ZIP_NAME}" .)
rm -rf output/temp

echo "Build complete: output/${ZIP_NAME}"
