#!/bin/sh
set -euo pipefail

export WASI_SDK="$(pwd)/../wasi-sdk/build/wasi-sdk-21.0.0ga50a641f4b5a+m" \

RUSTC_LOG=rustc_codegen_ssa::back::link=info \
RUSTFLAGS="-C link-args=--trace-symbol=stderr -C link-args=--trace-symbol=stdout" \
  cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target wasm32-wasi --release -vv

mkdir -p ./build
wasm-opt -Oz -g -o ./build/preopt.wasm ./target/wasm32-wasi/release/quicky.wasm
wasm-tools print ./build/preopt.wasm > ./build/preopt.wat
wizer --allow-wasi --wasm-bulk-memory=true -o ./build/wizened.wasm ./build/preopt.wasm
wasm-tools print ./build/wizened.wasm > ./build/wizened.wat
wasm-opt -Oz -g -o ./build/opt.wasm ./build/wizened.wasm
wasm-tools print ./build/opt.wasm > ./build/opt.wat

ls -lah ./build/*.wasm
