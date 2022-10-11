#! /bin/bash

export RUST_BACKTRACE=1
cargo build --release
sudo ../target/release/tcp-rs