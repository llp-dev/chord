#!/bin/bash
set -e

SEL4_VERSION="${SEL4_VERSION:-15.0.0}"
BUILD_DIR="$(mktemp -d)"

git clone --depth 1 --branch "$SEL4_VERSION" https://github.com/seL4/seL4.git "$BUILD_DIR/sel4-src"

mkdir "$BUILD_DIR/build"
cd "$BUILD_DIR/build"

cmake -G Ninja "$BUILD_DIR/sel4-src" \
    -DCMAKE_TOOLCHAIN_FILE="$BUILD_DIR/sel4-src/gcc.cmake" \
    -DCMAKE_INSTALL_PREFIX=/sel4 \
    -C /tmp/config.cmake

ninja
ninja install
