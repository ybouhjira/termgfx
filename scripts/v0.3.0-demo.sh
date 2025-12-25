#!/bin/bash
# TermGFX v0.3.0 Demo - New Features Showcase
# Run: ./scripts/v0.3.0-demo.sh

set -e
TERMGFX="${TERMGFX:-./target/release/termgfx}"

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

pause() {
    echo ""
    read -p "Press Enter to continue..." || true
    clear
}

section() {
    echo -e "\n${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${CYAN}  $1${NC}"
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Build if needed
if [ ! -f "$TERMGFX" ]; then
    echo "Building termgfx..."
    cargo build --release
fi

clear
echo -e "${BOLD}${GREEN}"
cat << 'EOF'
████████╗███████╗██████╗ ███╗   ███╗ ██████╗ ███████╗██╗  ██╗
╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝ ██╔════╝╚██╗██╔╝
   ██║   █████╗  ██████╔╝██╔████╔██║██║  ███╗█████╗   ╚███╔╝
   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║   ██║██╔══╝   ██╔██╗
   ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╔╝██║     ██╔╝ ██╗
   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝
EOF
echo -e "${NC}"
echo -e "${BOLD}v0.3.0 New Features Demo${NC}"
echo ""
echo "This script showcases what's new in v0.3.0:"
echo "  ✨ --demo flag on 7 commands"
echo "  🎬 --animate flag on line & pie charts"
echo "  🔧 UX fixes (animate, select, typewriter)"
pause

#=============================================================================
# FEATURE 1: --demo flags
#=============================================================================
section "1. NEW: --demo Flag (7 Commands)"

echo -e "${YELLOW}▸ termgfx box \"x\" --demo${NC}\n"
$TERMGFX box "x" --demo
pause

echo -e "${YELLOW}▸ termgfx banner \"x\" --demo${NC}\n"
$TERMGFX banner "x" --demo
pause

echo -e "${YELLOW}▸ termgfx progress 0 --demo${NC}\n"
$TERMGFX progress 0 --demo
pause

echo -e "${YELLOW}▸ termgfx chart bar --data \"a:1\" --demo${NC}\n"
$TERMGFX chart bar --data "a:1" --demo
pause

echo -e "${YELLOW}▸ termgfx sparkline \"1\" --demo${NC}\n"
$TERMGFX sparkline "1" --demo
pause

echo -e "${YELLOW}▸ termgfx gauge 0 --demo${NC}\n"
$TERMGFX gauge 0 --demo
pause

echo -e "${YELLOW}▸ termgfx table --rows \"a:1\" --demo${NC}\n"
$TERMGFX table --rows "a:1" --demo
pause

#=============================================================================
# FEATURE 2: Chart Animations
#=============================================================================
section "2. NEW: Chart Animations"

echo -e "${YELLOW}▸ termgfx chart line --data \"...\" --animate${NC}\n"
$TERMGFX chart line --data "10,25,18,35,28,42,38,55,48,62,58,70" --title "Monthly Growth" --animate
pause

echo -e "${YELLOW}▸ termgfx chart pie --data \"...\" --animate${NC}\n"
$TERMGFX chart pie --data "Desktop:45,Mobile:35,Tablet:15,Other:5" --animate
pause

#=============================================================================
# FEATURE 3: UX Fixes
#=============================================================================
section "3. UX FIXES"

echo -e "${YELLOW}FIX 1: animate now requires --effect-type${NC}"
echo "  Before: termgfx animate (showed progress by default - confusing)"
echo "  After:  termgfx animate --effect-type progress"
echo ""
$TERMGFX animate --effect-type progress --duration 2
pause

echo -e "${YELLOW}FIX 2: typewriter is now under animate${NC}"
echo "  Before: termgfx typewriter \"text\""
echo "  After:  termgfx animate --effect-type typewriter --text \"text\""
echo ""
$TERMGFX animate --effect-type typewriter --text "Hello World!" --duration 1
pause

echo -e "${YELLOW}FIX 3: choose merged into select --multi${NC}"
echo "  Before: termgfx choose \"Pick:\" opt1 opt2"
echo "  After:  termgfx select \"Pick:\" opt1 opt2 --multi"
echo ""
echo "(Interactive - skipping in demo)"
pause

#=============================================================================
# SUMMARY
#=============================================================================
section "SUMMARY: What's New in v0.3.0"

echo "┌─────────────────────────────────────────────────────────────┐"
echo "│                    TermGFX v0.3.0                           │"
echo "├─────────────────────────────────────────────────────────────┤"
echo "│  ✨ NEW FEATURES                                            │"
echo "│    • --demo flag on 7 commands (instant showcase)           │"
echo "│    • --animate on line charts (point-by-point drawing)      │"
echo "│    • --animate on pie charts (slice-by-slice reveal)        │"
echo "│                                                             │"
echo "│  🔧 UX IMPROVEMENTS                                         │"
echo "│    • animate requires --effect-type (no confusing default)  │"
echo "│    • typewriter moved under animate -t typewriter           │"
echo "│    • choose merged into select --multi                      │"
echo "│                                                             │"
echo "│  📊 DOCUMENTATION                                           │"
echo "│    • Comprehensive UX analysis (docs/UX-ANALYSIS.md)        │"
echo "│    • Competitive comparison (docs/COMPETITORS.md)           │"
echo "│    • Plotext feature demo (scripts/plotext-demo.py)         │"
echo "└─────────────────────────────────────────────────────────────┘"
echo ""
$TERMGFX box "termgfx v0.3.0 - More intuitive, more animated!" --style success --border rounded
echo ""
echo "🎉 Demo complete!"
