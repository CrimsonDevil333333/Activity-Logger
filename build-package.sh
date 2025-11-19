#!/bin/bash
# Build and Package Script for Linux
# Run this script to create a distributable package

echo -e "\033[0;36mBuilding Activity Logger...\033[0m"

# Build release version
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "\033[0;31mBuild failed!\033[0m"
    exit 1
fi

echo -e "\033[0;32mBuild successful!\033[0m"
echo -e "\033[0;36mCreating distribution package...\033[0m"

# Create distribution directory
DIST_DIR="dist/Activity-Logger-Linux"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copy files
cp target/release/activity_logger "$DIST_DIR/"
cp config.json "$DIST_DIR/"
cp run.sh "$DIST_DIR/"
cp DISTRIBUTION.md "$DIST_DIR/README.md"

# Copy icon if exists
if [ -f "assets/icon.png" ]; then
    cp assets/icon.png "$DIST_DIR/"
fi

# Make scripts executable
chmod +x "$DIST_DIR/activity_logger"
chmod +x "$DIST_DIR/run.sh"

echo -e "\033[0;32mPackage created at: $DIST_DIR\033[0m"

# Create tar.gz archive
TAR_PATH="dist/Activity-Logger-Linux.tar.gz"
rm -f "$TAR_PATH"

tar -czf "$TAR_PATH" -C dist Activity-Logger-Linux

echo -e "\033[0;32mArchive created: $TAR_PATH\033[0m"
echo ""
echo -e "\033[0;36mDistribution package ready!\033[0m"
echo -e "\033[0;36mYou can share the tar.gz file with others.\033[0m"
