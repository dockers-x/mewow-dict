#!/bin/bash
# Simple test script to verify MDD parsing works

echo "Building the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo "Build successful!"

echo ""
echo "To test MDD resource loading:"
echo "1. Start the server: cargo run --bin mewow-dict"
echo "2. The server will load all .mdd files from your dictionary directories"
echo "3. Access MDD resources via: http://localhost:8181/mdd/{resource_path}"
echo ""
echo "Example resources from loaded MDD files:"
echo "  - Images: /mdd/example.png"
echo "  - Audio: /mdd/pronunciation.mp3"
echo "  - CSS: /mdd/styles.css"
echo ""
echo "Check the console output when starting the server to see:"
echo "  - How many MDD files were found"
echo "  - Which MDD files were loaded successfully"
