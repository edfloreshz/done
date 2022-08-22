#!/bin/sh

export SOURCE_ROOT="$1"
export DIST="$2"

echo "$SOURCE_ROOT"
echo "$DIST"
cd "$SOURCE_ROOT" || exit
mkdir "$DIST"/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $DIST/.cargo/config
# Move vendor into dist tarball directory
mv vendor "$DIST"

