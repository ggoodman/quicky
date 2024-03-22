#!/bin/sh
set -euo pipefail

TRACE_SYMBOLS="stderr stdout __stdio_close __stdio_write __stdio_seek"
RUSTFLAGS="${RUSTFLAGS:-}"

for TRACE_SYMBOL in $TRACE_SYMBOLS; do
  RUSTFLAGS="$RUSTFLAGS -C link-args=--trace-symbol=$TRACE_SYMBOL"
done

export WASI_SDK="$(pwd)/wasi-sdk/build/install/opt/wasi-sdk"

RUSTC_LOG=rustc_codegen_ssa::back::link=info \
RUSTFLAGS=$RUSTFLAGS \
  cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target wasm32-wasi --release -vv

mkdir -p ./build
cp ./target/wasm32-wasi/release/quicky.wasm ./build/0_quicky.wasm
wasm-tools print ./build/0_quicky.wasm > ./build/0_quicky.wat
wasm-opt -Oz -g -o ./build/1_preopt.wasm ./build/0_quicky.wasm
wasm-tools print ./build/1_preopt.wasm > ./build/1_preopt.wat
wizer --allow-wasi --wasm-bulk-memory=true -o ./build/2_wizened.wasm ./build/1_preopt.wasm
wasm-tools print ./build/2_wizened.wasm > ./build/2_wizened.wat
wasm-opt -Oz -g -o ./build/3_opt.wasm ./build/2_wizened.wasm
wasm-tools print ./build/3_opt.wasm > ./build/3_opt.wat

echo
echo "Generated wasm files:"
ls -lah ./build/*.wasm

echo
echo "WASI Imports"
cat ./build/0_quicky.wat | grep '(import "wasi_snapshot_preview1"'
