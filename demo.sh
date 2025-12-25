#!/bin/bash
# termgfx Feature Demo - Showcases all 23 commands
# Run with: ./demo.sh

TERMGFX="./target/release/termgfx"
PAUSE=1.5

clear
echo ""

# Header
$TERMGFX banner "TERMGFX DEMO" --gradient cyan-purple
sleep $PAUSE

# ============================================================================
# STYLED BOXES
# ============================================================================
$TERMGFX box "Styled Boxes - Multiple styles and borders" --style info --border double
sleep 0.5

$TERMGFX box "Info Style" --style info --border rounded
sleep 0.3
$TERMGFX box "Success Style" --style success --border rounded --emoji "✅"
sleep 0.3
$TERMGFX box "Warning Style" --style warning --border rounded --emoji "⚠️"
sleep 0.3
$TERMGFX box "Danger Style" --style danger --border double --emoji "🚨"
sleep 0.3
$TERMGFX box "Gradient Style" --style gradient --border single
sleep $PAUSE

# ============================================================================
# BANNERS
# ============================================================================
$TERMGFX box "ASCII Art Banners" --style info --border double
sleep 0.5

$TERMGFX banner "HELLO"
sleep 0.5
$TERMGFX banner "WORLD" --gradient green-cyan
sleep $PAUSE

# ============================================================================
# PROGRESS BARS
# ============================================================================
$TERMGFX box "Progress Bars - Multiple styles" --style info --border double
sleep 0.5

echo "  Gradient style:"
$TERMGFX progress 75 --style gradient
echo ""
echo "  Blocks style:"
$TERMGFX progress 60 --style blocks
echo ""
echo "  Modern style:"
$TERMGFX progress 85 --style modern
echo ""
echo "  Classic style:"
$TERMGFX progress 50 --style classic
echo ""
echo "  Thin style:"
$TERMGFX progress 90 --style thin
echo ""
echo "  Custom colors (red to green):"
$TERMGFX progress 70 --from red --to green
echo ""
sleep 0.5

echo "  Animated progress:"
$TERMGFX progress 100 --animate --duration 1000
sleep $PAUSE

# ============================================================================
# SPINNERS
# ============================================================================
$TERMGFX box "Spinners - 8 styles with auto-stop" --style info --border double
sleep 0.5

echo "  Dots spinner:"
$TERMGFX spinner "Loading..." --style dots --duration 1
echo "  Moon spinner:"
$TERMGFX spinner "Processing..." --style moon --duration 1
echo "  Line spinner:"
$TERMGFX spinner "Working..." --style line --duration 1
sleep $PAUSE

# ============================================================================
# TYPEWRITER
# ============================================================================
$TERMGFX box "Typewriter Effect" --style info --border double
sleep 0.5

$TERMGFX typewriter "This text appears character by character..." --speed 30
echo ""
sleep $PAUSE

# ============================================================================
# SPARKLINES
# ============================================================================
$TERMGFX box "Sparklines - Inline charts" --style info --border double
sleep 0.5

echo "  CPU Usage: "
$TERMGFX sparkline "10,25,50,75,60,40,80,95,70,45,30,55"
echo ""
echo "  Memory:    "
$TERMGFX sparkline "20,20,25,30,35,40,45,50,55,60,65,70"
echo ""
sleep $PAUSE

# ============================================================================
# CHARTS
# ============================================================================
$TERMGFX box "Charts - Line, Bar, Pie" --style info --border double
sleep 0.5

echo ""
echo "  Line Chart:"
$TERMGFX chart line --data "10,25,15,40,30,55,45,70" --title "Weekly Sales"
echo ""
sleep 0.5

echo "  Bar Chart:"
$TERMGFX chart bar --data "Mon:20,Tue:35,Wed:25,Thu:45,Fri:30"
echo ""
sleep 0.5

echo "  Pie Chart:"
$TERMGFX chart pie --data "Chrome:65,Firefox:20,Safari:10,Other:5"
echo ""
sleep $PAUSE

# ============================================================================
# TABLES
# ============================================================================
$TERMGFX box "Tables - Formatted data display" --style info --border double
sleep 0.5

$TERMGFX table --headers "Name,Role,Status" --rows "Alice,Developer,Active|Bob,Designer,Active|Carol,Manager,Away" --border rounded
sleep $PAUSE

# ============================================================================
# TREE
# ============================================================================
$TERMGFX box "Tree Structure" --style info --border double
sleep 0.5

$TERMGFX tree "project>src>main.rs,lib.rs|tests>unit,integration|docs>README"
sleep $PAUSE

# ============================================================================
# DIFF
# ============================================================================
$TERMGFX box "File Diff - Side by side comparison" --style info --border double
sleep 0.5

# Create temp files for diff demo
echo -e "line 1\nline 2\nline 3" > /tmp/file1.txt
echo -e "line 1\nmodified\nline 3" > /tmp/file2.txt
$TERMGFX diff /tmp/file1.txt /tmp/file2.txt
rm /tmp/file1.txt /tmp/file2.txt
sleep $PAUSE

# ============================================================================
# ANIMATIONS
# ============================================================================
$TERMGFX box "Animation Effects" --style info --border double
sleep 0.5

echo "  Counter animation:"
$TERMGFX animate -t counter --from 0 --to 1000 --prefix "$" -D 1.5
echo ""
sleep 0.5

echo "  Progress animation:"
$TERMGFX animate -t progress -D 1.5
echo ""
sleep $PAUSE

# ============================================================================
# SCRIPT
# ============================================================================
$TERMGFX box "Script Runner - Chain commands" --style info --border double
sleep 0.5

$TERMGFX script --inline 'box "From script!" style:success
progress 50
sparkline 1,2,3,4,5,4,3,2,1'
sleep $PAUSE

# ============================================================================
# FOOTER
# ============================================================================
echo ""
$TERMGFX banner "FIN" --gradient purple-cyan
$TERMGFX box "23 commands | 200 tests | TTY-aware animations" --style gradient --border double
echo ""
echo "  GitHub: https://github.com/ybouhjira/termgfx"
echo ""
