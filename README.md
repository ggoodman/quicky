# Quicky

Expirementing with QuickJS in WASM.

## Setup

```sh
git clone --recursive https://github.com/ggoodman/quicky
cd quicky
```

```sh
# Coffee break time while we build wasi-libc and half who knows what else.
pushd wasi-sdk
NINJA_FLAGS=-v make build/wasi-libc.BUILT LLVM_CMAKE_FLAGS=-DLLVM_CCACHE_BUILD=ON
popd
```

```sh
# Optionally, clean out cargo cache. Our build script isn't 100% tuned for
# detecting changes.
cargo clean
```

```sh
# Build out quicky's WASM binaries. They'll end up in `./build/*.wasm`.
./scripts/build.sh
```

## Issue

I'm trying to figure out why the rust wasm toolchain is including the `fd_close`, `fd_write` and `fd_seek` imports in the resulting binary. These are being included even through wizening and two passes of `wasm-opt -Oz` (before and after wizening 🤷🏼‍♂️).

Here is the end of the output from the `./scripts/build.sh` script. It shows the `--trace-symbol` output from several libc symbols that stood out. I may very well be missing the critical ones.

You'll also see the file sizes and then the WAT instructions showing that LTO isn't stripping out the `fd_*` symbols.

```
INFO rustc_codegen_ssa::back::link linker stdout:
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(__stdio_close.o): lazy definition of __stdio_close
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(__stdio_seek.o): lazy definition of __stdio_seek
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(__stdio_write.o): lazy definition of __stdio_write
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(stderr.o): lazy definition of stderr
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(stdout.o): reference to __stdio_close
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(__stdio_close.o): definition of __stdio_close
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(stdout.o): reference to __stdio_write
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(__stdio_write.o): definition of __stdio_write
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(stdout.o): reference to __stdio_seek
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(__stdio_seek.o): definition of __stdio_seek
$(pwd)/quicky/wasi-sdk/build/install/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi/libc.a(stdout.o): definition of stdout

    Finished `release` profile [optimized] target(s) in 17.70s

Generated wasm files:
-rwxr-xr-x  1 ggoodman  staff   452K 21 Mar 21:11 ./build/0_quicky.wasm
-rw-r--r--  1 ggoodman  staff   392K 21 Mar 21:11 ./build/1_preopt.wasm
-rw-r--r--  1 ggoodman  staff   438K 21 Mar 21:11 ./build/2_wizened.wasm
-rw-r--r--  1 ggoodman  staff   441K 21 Mar 21:11 ./build/3_opt.wasm

WASI Imports:
  (import "wasi_snapshot_preview1" "clock_time_get" (func $__imported_wasi_snapshot_preview1_clock_time_get (;0;) (type 22)))
  (import "wasi_snapshot_preview1" "fd_close" (func $__imported_wasi_snapshot_preview1_fd_close (;1;) (type 5)))
  (import "wasi_snapshot_preview1" "fd_seek" (func $__imported_wasi_snapshot_preview1_fd_seek (;2;) (type 32)))
  (import "wasi_snapshot_preview1" "fd_write" (func $__imported_wasi_snapshot_preview1_fd_write (;3;) (type 27)))
```
