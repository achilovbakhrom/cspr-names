#!/bin/bash
mkdir ~/.cargo/my_custom_toolchains
cp -r ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu ~/.cargo/my_custom_toolchains/totally-custom-toolchain

rustup toolchain link totally-custom-toolchain ~/.cargo/my_custom_toolchains/totally-custom-toolchain
