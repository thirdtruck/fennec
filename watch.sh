#!/bin/sh

cargo watch --clear --watch src --watch Cargo.toml -s "cargo check && cargo build --bin fennec && cargo build"
