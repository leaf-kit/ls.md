#!/bin/bash
set -e

echo "================================"
echo "  lsmd build script"
echo "  Version: $(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')"
echo "================================"
echo ""

# Step 1: Run tests
echo "[1/3] Running tests..."
cargo test 2>&1
echo ""
echo "  -> All tests passed."
echo ""

# Step 2: Build release binary
echo "[2/3] Building release binary..."
cargo build --release 2>&1
echo ""
echo "  -> Build complete: target/release/lsmd"
echo ""

# Step 3: Show binary info
BINARY="target/release/lsmd"
SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
echo "[3/3] Binary info"
echo "  Path: $BINARY"
echo "  Size: $SIZE"
echo ""

echo "================================"
echo "  Build successful!"
echo ""
echo "  Install:"
echo "    cp $BINARY /usr/local/bin/"
echo "================================"
