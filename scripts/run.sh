#! /bin/bash

set -e

export RUST_BACKTRACE=1
cargo build --release

sudo target/release/tcp-rs tap0