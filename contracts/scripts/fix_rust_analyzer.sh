#!/bin/bash

# We use a copy of stable Rust 1.61 in this example, which we place on a folder we
# create on .cargo; this is closest to the failure seen in the wild.
# (The first two commands are "translations" of steps I actually did using Windows File Explorer,
# since I'm more familiar with Linux commands)
mkdir ~/.cargo/my_custom_toolchains
cp -r ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu ~/.cargo/my_custom_toolchains/totally-custom-toolchain

rustup toolchain link totally-custom-toolchain ~/.cargo/my_custom_toolchains/totally-custom-toolchain

# No need to switch to newly-created "totally-custom-toolchain"; we will instead set `RUSTUP_TOOLCHAIN`.

# for .vscode/settings.json

# {
#     "rust-analyzer.checkOnSave.enable": true,
#     "rust-analyzer.checkOnSave.command": "clippy",
#     "rust-analyzer.server.extraEnv": {
#         "RUSTUP_TOOLCHAIN": "totally-custom-toolchain"
#     }
#     // All other settings at their default values
# }