#!/bin/bash
#needs: rustup target install x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin