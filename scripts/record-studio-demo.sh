#!/bin/bash
# Record a GIF demo of TermGFX Studio using asciinema + agg
# Requires: asciinema, agg (https://github.com/asciinema/agg)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_DIR/assets"
CAST_FILE="$OUTPUT_DIR/studio-demo.cast"
GIF_FILE="$OUTPUT_DIR/studio-demo.gif"

mkdir -p "$OUTPUT_DIR"

echo "Recording TermGFX Studio demo..."
echo ""
echo "Instructions:"
echo "  1. Navigate through components (j/k)"
echo "  2. Edit some parameters (Enter, sliders)"
echo "  3. Copy a command (c)"
echo "  4. Show help overlay (?)"
echo "  5. Save a favorite (s)"
echo "  6. Quit (q)"
echo ""
echo "Press Enter to start recording..."
read -r

# Record with asciinema
asciinema rec "$CAST_FILE" --cols 120 --rows 35 -c "$PROJECT_DIR/target/release/termgfx studio"

echo ""
echo "Recording saved to: $CAST_FILE"

# Convert to GIF if agg is available
if command -v agg &> /dev/null; then
    echo "Converting to GIF..."
    agg "$CAST_FILE" "$GIF_FILE" --cols 120 --rows 35 --speed 1.5
    echo "GIF saved to: $GIF_FILE"
else
    echo ""
    echo "Install 'agg' to convert to GIF:"
    echo "  cargo install agg"
    echo ""
    echo "Then run:"
    echo "  agg $CAST_FILE $GIF_FILE --cols 120 --rows 35"
fi
