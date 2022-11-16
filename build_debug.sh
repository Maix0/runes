#!/bin/sh
cargo xtask build --quiet && rustfmt out/instructions.rs && cargo build --quiet
