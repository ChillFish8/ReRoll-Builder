#!/bin/bash
set -ex

export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release

cp -r "./target/release/ReRollBuilder.exe" "./build/"
