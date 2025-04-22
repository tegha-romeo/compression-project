#!/bin/bash

INPUT="test.txt"
RUST_OUT="out.rle"
JS_OUT="out-js.rle"

# Generate a test file (repeat pattern)
echo -n "ABABABABABABABABABABCDCDCDCDCDCD" > "$INPUT"

echo "▶️ Testing Rust CLI (--rle)..."
time docker run -v "$(pwd)":/data rust-compressor compress /data/$INPUT /data/$RUST_OUT --rle
time docker run -v "$(pwd)":/data rust-compressor decompress /data/$RUST_OUT /data/recovered-rust.txt --rle

echo "✅ Rust Output Size: $(stat -c%s "$RUST_OUT") bytes"
echo

echo "▶️ Testing JS CLI (--rle)..."
time docker run -v "$(pwd)":/data js-compressor compress /data/$INPUT /data/$JS_OUT --rle
time docker run -v "$(pwd)":/data js-compressor decompress /data/$JS_OUT /data/recovered-js.txt --rle

echo "✅ JS Output Size: $(stat -c%s "$JS_OUT") bytes"
echo

# Final verification
echo "🧪 Verifying roundtrip correctness..."
diff test.txt recovered-rust.txt && echo "✅ Rust roundtrip OK"
diff test.txt recovered-js.txt && echo "✅ JS roundtrip OK"
