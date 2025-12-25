#!/bin/bash
# TermGFX v0.4.0 Demo Script
# Showcases all features including new interactive commands

set -e

TERMGFX="./target/release/termgfx"

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

section() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}  $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    sleep 1
}

pause() {
    echo ""
    echo -e "${GREEN}Press Enter to continue...${NC}"
    read -r
}

# Build if needed
if [ ! -f "$TERMGFX" ]; then
    echo "Building termgfx..."
    cargo build --release
fi

clear
$TERMGFX banner "TERMGFX" --gradient cyan-purple --animate
echo ""
echo "          Beautiful Terminal Graphics for CLI Applications"
echo "                         Version 0.4.0"
echo ""
pause

# ═══════════════════════════════════════════════════════════════════════════
section "📦 STYLED BOXES"
# ═══════════════════════════════════════════════════════════════════════════

echo "Various box styles with animations:"
echo ""

$TERMGFX box "Build completed successfully!" --style success --animate
sleep 0.5
$TERMGFX box "Warning: disk space low" --style warning --animate
sleep 0.5
$TERMGFX box "Error: connection failed" --style danger --animate
sleep 0.5
$TERMGFX box "Deployment started..." --style info --border rounded --animate

pause

# ═══════════════════════════════════════════════════════════════════════════
section "🎨 GRADIENT BANNERS"
# ═══════════════════════════════════════════════════════════════════════════

echo "ASCII art banners with gradient colors:"
echo ""

$TERMGFX banner "DEPLOY" --gradient red-orange --animate
sleep 0.5
$TERMGFX banner "SUCCESS" --gradient green-cyan --animate

pause

# ═══════════════════════════════════════════════════════════════════════════
section "📊 CHARTS"
# ═══════════════════════════════════════════════════════════════════════════

echo "Bar Chart:"
$TERMGFX chart bar --data "Rust:95,Go:85,Python:75,Java:65,C++:70" --animate
echo ""
sleep 1

echo "Line Chart:"
$TERMGFX chart line --data "10,25,18,35,28,42,38,55,50" --title "Weekly Sales" --animate
echo ""
sleep 1

echo "Pie Chart:"
$TERMGFX chart pie --data "Desktop:45,Mobile:35,Tablet:20" --animate
echo ""

pause

# ═══════════════════════════════════════════════════════════════════════════
section "📈 GAUGES & SPARKLINES"
# ═══════════════════════════════════════════════════════════════════════════

echo "Radial Gauges:"
echo ""
$TERMGFX gauge 75 --label "CPU Usage" --style semicircle --animate
echo ""
$TERMGFX gauge 45 --label "Memory" --style minimal --animate
echo ""

echo "Sparklines:"
$TERMGFX sparkline "1,4,2,8,5,7,3,9,6,5,8,3,7,4" --animate

pause

# ═══════════════════════════════════════════════════════════════════════════
section "🗓️ HEATMAP & TIMELINE"
# ═══════════════════════════════════════════════════════════════════════════

echo "Heatmap visualization:"
$TERMGFX heatmap --data "1,3,5,7;2,4,6,8;3,5,7,9;4,6,8,10" --colors viridis --x-labels "Mon,Tue,Wed,Thu" --y-labels "W1,W2,W3,W4" --animate
echo ""

echo "Timeline:"
$TERMGFX timeline --events "Q1:Planning,Q2:Development,Q3:Testing,Q4:Launch" --style arrow --animate

pause

# ═══════════════════════════════════════════════════════════════════════════
section "📋 TABLES & TREES"
# ═══════════════════════════════════════════════════════════════════════════

echo "Table:"
$TERMGFX table --headers "Name,Role,Status" --rows "Alice,Developer,Active|Bob,Designer,Active|Charlie,PM,Away" --border rounded --animate
echo ""

echo "Tree:"
$TERMGFX tree "project>src,tests,docs>main.rs,lib.rs,README.md" --animate

pause

# ═══════════════════════════════════════════════════════════════════════════
section "📝 DIFF COMPARISON"
# ═══════════════════════════════════════════════════════════════════════════

# Create temp files for diff demo
echo -e "Hello World\nThis is line 2\nOriginal line 3" > /tmp/demo_old.txt
echo -e "Hello TermGFX\nThis is line 2\nModified line 3\nNew line 4" > /tmp/demo_new.txt

echo "Side-by-side file comparison:"
$TERMGFX diff /tmp/demo_old.txt /tmp/demo_new.txt

# Cleanup
rm -f /tmp/demo_old.txt /tmp/demo_new.txt

pause

# ═══════════════════════════════════════════════════════════════════════════
section "⏳ PROGRESS & SPINNERS"
# ═══════════════════════════════════════════════════════════════════════════

echo "Progress bar styles:"
echo ""
echo "Gradient:"
$TERMGFX progress 85 --style gradient --animate
echo ""
echo "Blocks:"
$TERMGFX progress 70 --style blocks --animate
echo ""
echo "Classic:"
$TERMGFX progress 60 --style classic --animate
echo ""

echo "Spinner styles:"
$TERMGFX spinner "Loading with dots..." --style dots --duration 2
$TERMGFX spinner "Processing..." --style arc --duration 2
$TERMGFX spinner "Please wait..." --style bounce --duration 2

pause

# ═══════════════════════════════════════════════════════════════════════════
section "✨ ANIMATIONS"
# ═══════════════════════════════════════════════════════════════════════════

echo "Typewriter effect:"
$TERMGFX animate --effect-type typewriter --text "Hello from TermGFX! Beautiful terminal graphics made easy." --duration 2
echo ""

echo "Counter animation:"
$TERMGFX animate --effect-type counter --from 0 --to 100 --prefix "Progress: " --suffix "%" --duration 2
echo ""

pause

# ═══════════════════════════════════════════════════════════════════════════
section "💬 INTERACTIVE PROMPTS"
# ═══════════════════════════════════════════════════════════════════════════

echo "TermGFX includes powerful interactive prompts:"
echo ""
echo "  • input    - Text input with placeholder"
echo "  • select   - Single/multi select menus"
echo "  • confirm  - Yes/No confirmation"
echo "  • file     - File/directory picker"
echo "  • filter   - Fuzzy filter (like fzf)"
echo "  • pager    - Scrollable content viewer"
echo ""
echo "Examples:"
echo "  termgfx input 'Your name:' --placeholder 'John Doe'"
echo "  termgfx select 'Language:' Rust Go Python"
echo "  termgfx file --path /var --ext log"
echo "  ls | termgfx filter --prompt 'Select file:'"
echo "  cat README.md | termgfx pager --line-numbers"

pause

# ═══════════════════════════════════════════════════════════════════════════
section "🎛️ DASHBOARD"
# ═══════════════════════════════════════════════════════════════════════════

echo "Multi-panel dashboard layout:"
$TERMGFX dashboard --layout 2x2 --title "System Monitor" --panels "gauge:75:CPU,sparkline:1,4,2,8,5,7:Memory,progress:60:Disk,box:All systems operational:Status"

pause

# ═══════════════════════════════════════════════════════════════════════════
section "🔔 NOTIFICATIONS"
# ═══════════════════════════════════════════════════════════════════════════

echo "Terminal + Desktop notifications:"
$TERMGFX notification "Deployment completed!" --title "TermGFX" --style success --terminal-only
sleep 0.5
$TERMGFX notification "Build started..." --title "CI/CD" --style info --terminal-only

pause

# ═══════════════════════════════════════════════════════════════════════════
section "🖼️ IMAGE DISPLAY"
# ═══════════════════════════════════════════════════════════════════════════

echo "TermGFX supports multiple image protocols:"
echo ""
echo "  • Kitty    - Best quality (Kitty, WezTerm)"
echo "  • Sixel    - Wide support (xterm, mlterm)"
echo "  • Halfblock - Universal fallback (any terminal)"
echo ""
echo "Usage:"
echo "  termgfx image ./photo.png"
echo "  termgfx image ./logo.jpg --protocol kitty"
echo "  termgfx image https://example.com/image.png"
echo ""

# Try to display an image if one exists
if [ -f "docs/gifs/box.gif" ]; then
    echo "Displaying box.gif with halfblock protocol:"
    $TERMGFX image docs/gifs/box.gif --protocol halfblock 2>/dev/null || echo "(Image display depends on terminal support)"
fi

pause

# ═══════════════════════════════════════════════════════════════════════════
clear
$TERMGFX banner "DONE" --gradient green-cyan --animate
echo ""
echo "          Thanks for watching the TermGFX demo!"
echo ""
echo "  📦 Install: cargo install termgfx"
echo "  📖 Docs:    https://github.com/ybouhjira/termgfx"
echo "  ⭐ Star:    https://github.com/ybouhjira/termgfx"
echo ""
$TERMGFX box "Happy terminal graphics!" --style success --border rounded
