#!/bin/bash

DIR="$(dirname "$0")"

if cargo "$@"; then
    [ -d "$DIR/target/debug" ] && rm -rf "$DIR/target/debug/templates" && cp -r "$DIR/templates" "$DIR/target/debug/templates"
    [ -d "$DIR/target/release" ] && rm -rf "$DIR/target/release/templates" && cp -r "$DIR/templates" "$DIR/target/release/templates"
fi
