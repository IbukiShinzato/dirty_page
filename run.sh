#!/bin/bash

# Create the file if it doesn't exist
FILE="test_file"
SIZE_GB=2

if [ ! -e "$FILE" ]; then
    echo "Creating $FILE ($SIZE_GB GB)..."
    dd if=/dev/zero of="$FILE" bs=1M count=$((SIZE_GB * 1024))
else
    echo "$FILE already exists."
fi

# Sync and clear the OS page cache to ensure a clean environment
echo "Cleaning environment..."
sync
echo 3 | sudo tee /proc/sys/vm/drop_caches

# Start the program
echo "Starting Program..."
if [ -f "./target/release/dirty_page" ]; then
    ./target/release/dirty_page "$FILE"
else
    # Fallback to cargo run if release binary is not found
    cargo run --release -- "$FILE"
fi

# Final sync to flush remaining dirty pages to disk
echo "Final syncing..."
sync
echo "Done."
