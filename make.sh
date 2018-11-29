#!/bin/bash

# Find our current CLI directory and the directory of the implementation library
DIR="$(dirname "$0")"
TEMPLATE_DIR="$(dirname "$(pwd)")/sliderule-rs"

# Make sure that the templates directory is accessible in whatever build directory it's needed
[ -d "$DIR/target/debug" ] && rm -rf "$DIR/target/debug/templates" && cp -r "$TEMPLATE_DIR/templates" "$DIR/target/debug/templates"
[ -d "$DIR/target/release" ] && rm -rf "$DIR/target/release/templates" && cp -r "$TEMPLATE_DIR/templates" "$DIR/target/release/templates"

# Run cargo with the command that was passed in by the user
cargo "$@"

# Rust seems to have a hard time killing this daemon
pkill -f "git-daemon"
