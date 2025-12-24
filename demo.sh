#!/bin/bash
# $TERMGFX Full Feature Demo
# Showcases ALL implemented commands

# Use cargo run or installed binary
TERMGFX="cargo run --quiet --"

clear

# Colors for section headers
CYAN='\033[0;36m'
NC='\033[0m'

section() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    sleep 0.5
}

# ============================================================================
# BANNER
# ============================================================================
$TERMGFX banner "$TERMGFX"
sleep 1

$TERMGFX typewriter "Welcome to the $TERMGFX feature showcase!" --speed 20
sleep 1

# ============================================================================
# STYLED BOXES
# ============================================================================
section "STYLED BOXES"

$TERMGFX box "Default info box with rounded border" --style info --border rounded
sleep 0.3

$TERMGFX box "Success! Operation completed" --style success --border single --emoji "✓"
sleep 0.3

$TERMGFX box "Warning: Check your configuration" --style warning --border double
sleep 0.3

$TERMGFX box "Error: Something went wrong" --style danger --border thick
sleep 0.3

$TERMGFX box "Gradient style box" --style gradient --border rounded
sleep 1

# ============================================================================
# PROGRESS BARS
# ============================================================================
section "PROGRESS BARS"

echo "Gradient style:"
$TERMGFX progress 75 --style gradient
sleep 0.5

echo "Blocks style:"
$TERMGFX progress 60 --style blocks
sleep 0.5

echo "Animated style:"
$TERMGFX progress 90 --style animated
sleep 1

# ============================================================================
# SPARKLINES
# ============================================================================
section "SPARKLINES"

echo "Stock price trend:"
$TERMGFX sparkline "10,15,12,18,25,22,30,28,35,40,38,45,50,48,55"
sleep 0.5

echo "Server load:"
$TERMGFX sparkline "5,8,12,25,45,80,95,70,40,20,15,10,8,5,3"
sleep 1

# ============================================================================
# TABLES
# ============================================================================
section "TABLES"

echo "Project Status Table:"
$TERMGFX table --headers "Feature,Status,Issue" --rows "Diff,Done,#32|Table,Done,#30|Tree,Done,#31|Record,Done,#38|Script,Done,#39" --border rounded
sleep 1

echo ""
echo "Team Members (double border):"
$TERMGFX table --headers "Name,Role,Status" --rows "Alice,Developer,Active|Bob,Designer,Active|Carol,PM,Away" --border double --alignment center
sleep 1

# ============================================================================
# TREE
# ============================================================================
section "TREE VISUALIZATION"

echo "Project structure:"
echo '{"$TERMGFX":{"src":{"main.rs":"entry","output":{"banner.rs":"module","box.rs":"module","table.rs":"module"},"charts":{"line.rs":"module","bar.rs":"module"}},"Cargo.toml":"config"}}' | $TERMGFX tree
sleep 1

# ============================================================================
# DIFF
# ============================================================================
section "DIFF COMPARISON"

# Create temp files for diff demo
echo -e "name = \"$TERMGFX\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nclap = \"4.0\"" > /tmp/old_cargo.toml
echo -e "name = \"$TERMGFX\"\nversion = \"0.2.0\"\nedition = \"2021\"\n\n[dependencies]\nclap = \"4.5\"\nserde = \"1.0\"" > /tmp/new_cargo.toml

echo "Side-by-side diff (Cargo.toml changes):"
$TERMGFX diff /tmp/old_cargo.toml /tmp/new_cargo.toml
sleep 1

# Cleanup
rm /tmp/old_cargo.toml /tmp/new_cargo.toml

# ============================================================================
# TYPEWRITER
# ============================================================================
section "TYPEWRITER EFFECT"

$TERMGFX typewriter "This text appears one character at a time..." --speed 30
sleep 0.5
$TERMGFX typewriter "Fast mode!" --speed 10
sleep 0.5
$TERMGFX typewriter "S l o w   m o d e . . ." --speed 80
sleep 1

# ============================================================================
# SCRIPT (Animation Sequences)
# ============================================================================
section "SCRIPT - ANIMATION SEQUENCES"

$TERMGFX script --inline "box \"Scripts can chain multiple commands\" style:info
wait 300ms
box \"They run in sequence\" style:success
wait 300ms
box \"With timing control\" style:warning"
sleep 1

# ============================================================================
# CHARTS
# ============================================================================
section "CHARTS"

echo "Line Chart - Monthly Revenue:"
$TERMGFX chart line --data "10,25,18,35,42,38,55,62,58,70,85,90" --title "Revenue 2024"
sleep 1

echo ""
echo "Bar Chart - Sales by Region:"
$TERMGFX chart bar --data "North:45,South:32,East:58,West:41"
sleep 1

echo ""
echo "Pie Chart - Market Share:"
$TERMGFX chart pie --data "Chrome:65,Firefox:15,Safari:12,Edge:8"
sleep 1

# ============================================================================
# FINALE
# ============================================================================
section "RECORD COMMAND"

echo "The record command captures terminal sessions:"
echo ""
$TERMGFX box "$TERMGFX record start session.rec   # Start recording" --style info --border single
$TERMGFX box "$TERMGFX record play session.rec    # Playback" --style info --border single
$TERMGFX box "$TERMGFX record export session.rec --format gif out.gif" --style info --border single
sleep 1

# ============================================================================
# FINALE
# ============================================================================
echo ""
echo ""
$TERMGFX banner "Done"
sleep 0.5

$TERMGFX box "All 17 commands demonstrated!" style:success border:double emoji:rocket

echo ""
echo "Commands: box, banner, spinner, progress, sparkline, chart (line/bar/pie),"
echo "          image, input, select, choose, confirm, diff, table, tree,"
echo "          record, typewriter, script"
echo ""
$TERMGFX typewriter "Thanks for watching! Star us on GitHub: github.com/ybouhjira/$TERMGFX" --speed 15
echo ""
