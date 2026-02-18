#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh v0.1.0"
    exit 1
fi

echo "Building releases for $VERSION..."
cargo build-release

# Create GitHub release
gh release create $VERSION \
  target/x86_64-unknown-linux-gnu/release/jig \
  target/x86_64-pc-windows-gnu/release/jig.exe \
  --title "Release $VERSION" \
  --generate-notes

echo "Done! https://github.com/yourname/jig/releases/tag/$VERSION"
