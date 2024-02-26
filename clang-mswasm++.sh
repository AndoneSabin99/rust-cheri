#!/bin/bash
# Quick and dirty wrapper to call the right clang with the right target and system library path.
# I'm not sure why the build system doesn't do this itself.
# See config.toml.sarah for more information.
source $(dirname "$0")/mswasm-config.sh

#$CHERI_HOME/output/morello-sdk/bin/clang --sysroot $CHERI_HOME/output/rootfs-morello-purecap/ -target wasm32-wasi "$@"

$MSWASM_HOME/mswasm-llvm/llvm/build/bin/clang++ -O3 --target=wasm32-wasi --sysroot=$CHERI_HOME/mswasm-wasi-libc/sysroot/ "$@"

