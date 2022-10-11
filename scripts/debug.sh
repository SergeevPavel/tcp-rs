#! /bin/bash

export RUST_BACKTRACE=1
cargo build
sudo target/debug/tcp-rs