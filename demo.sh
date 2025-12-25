#!/bin/bash
# termgfx Feature Demo - Showcases all 25 commands with animations
# Run with: ./demo.sh

TERMGFX="./target/release/termgfx"
PAUSE=1.5

# Build release version if needed
if [ ! -f "$TERMGFX" ]; then
    echo "Building release version..."
    cargo build --release
fi

clear
echo ""

# ============================================================================
# INTRO
# ============================================================================
$TERMGFX banner "TERMGFX DEMO" --gradient cyan-purple --animate
sleep $PAUSE

$TERMGFX box "25 Commands | 293 Tests | Full Animation Support" --style gradient --border double --animate
sleep $PAUSE

# ============================================================================
# STYLED BOXES
# ============================================================================
$TERMGFX box "STYLED BOXES" --style info --border double
sleep 0.5

$TERMGFX box "Info Style" --style info --border rounded --animate
sleep 0.3
$TERMGFX box "Success Style" --style success --border rounded --emoji "✅" --animate
sleep 0.3
$TERMGFX box "Warning Style" --style warning --border rounded --emoji "⚠️" --animate
sleep 0.3
$TERMGFX box "Danger Style" --style danger --border double --emoji "🚨" --animate
sleep $PAUSE

# ============================================================================
# BANNERS
# ============================================================================
$TERMGFX box "ASCII ART BANNERS" --style info --border double
sleep 0.5

$TERMGFX banner "HELLO" --animate
sleep 0.5
$TERMGFX banner "WORLD" --gradient green-cyan --animate
sleep $PAUSE

# ============================================================================
# PROGRESS BARS
# ============================================================================
$TERMGFX box "PROGRESS BARS" --style info --border double
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
echo "  Animated progress:"
$TERMGFX progress 100 --animate --duration 1500
sleep $PAUSE

# ============================================================================
# SPINNERS
# ============================================================================
$TERMGFX box "SPINNERS" --style info --border double
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
$TERMGFX box "TYPEWRITER EFFECT" --style info --border double
sleep 0.5

$TERMGFX typewriter "This text appears character by character..." --speed 30
echo ""
sleep $PAUSE

# ============================================================================
# SPARKLINES
# ============================================================================
$TERMGFX box "SPARKLINES" --style info --border double
sleep 0.5

echo "  CPU Usage (animated): "
$TERMGFX sparkline "10,25,50,75,60,40,80,95,70,45,30,55" --animate
echo ""
echo "  Memory Usage: "
$TERMGFX sparkline "20,20,25,30,35,40,45,50,55,60,65,70"
echo ""
sleep $PAUSE

# ============================================================================
# CHARTS
# ============================================================================
$TERMGFX box "CHARTS" --style info --border double
sleep 0.5

echo ""
echo "  Line Chart:"
$TERMGFX chart line --data "10,25,15,40,30,55,45,70" --title "Weekly Sales"
echo ""
sleep 0.5

echo "  Bar Chart (animated):"
$TERMGFX chart bar --data "Mon:20,Tue:35,Wed:25,Thu:45,Fri:30" --animate
echo ""
sleep 0.5

echo "  Pie Chart:"
$TERMGFX chart pie --data "Chrome:65,Firefox:20,Safari:10,Other:5"
echo ""
sleep $PAUSE

# ============================================================================
# TABLES (NEW!)
# ============================================================================
$TERMGFX box "TABLES" --style info --border double
sleep 0.5

echo "  Animated table:"
$TERMGFX table --headers "Name,Role,Status" --rows "Alice,Developer,Active|Bob,Designer,Active|Carol,Manager,Away|Dave,QA,Active" --border rounded --animate
sleep $PAUSE

# ============================================================================
# TREE (NEW!)
# ============================================================================
$TERMGFX box "TREE STRUCTURE" --style info --border double
sleep 0.5

echo "  Project tree (animated):"
$TERMGFX tree "project>src,tests,docs>main.rs,lib.rs>README.md" --animate
sleep $PAUSE

# ============================================================================
# DIFF
# ============================================================================
$TERMGFX box "FILE DIFF" --style info --border double
sleep 0.5

echo -e "line 1\nline 2\nline 3" > /tmp/file1.txt
echo -e "line 1\nmodified line\nline 3\nnew line" > /tmp/file2.txt
$TERMGFX diff /tmp/file1.txt /tmp/file2.txt
rm /tmp/file1.txt /tmp/file2.txt
sleep $PAUSE

# ============================================================================
# GAUGE (NEW!)
# ============================================================================
$TERMGFX box "GAUGES" --style info --border double
sleep 0.5

echo "  Semicircle gauge (animated):"
$TERMGFX gauge 75 --label "CPU" --style semicircle --color cyan --animate
echo ""
echo "  Full gauge:"
$TERMGFX gauge 60 --label "Memory" --style full --color green
echo ""
echo "  Minimal gauge:"
$TERMGFX gauge 90 --label "Disk" --style minimal --color yellow
sleep $PAUSE

# ============================================================================
# HEATMAP (NEW!)
# ============================================================================
$TERMGFX box "HEATMAP" --style info --border double
sleep 0.5

echo "  Activity heatmap (animated):"
$TERMGFX heatmap --data "1,2,3,4,5;2,4,6,8,10;3,6,9,12,15" --x-labels "Mon,Tue,Wed,Thu,Fri" --y-labels "Week1,Week2,Week3" --colors viridis --animate
sleep $PAUSE

# ============================================================================
# TIMELINE (NEW!)
# ============================================================================
$TERMGFX box "TIMELINE" --style info --border double
sleep 0.5

echo "  Project timeline (animated):"
$TERMGFX timeline --events "2024-01:Planning,2024-03:Development,2024-06:Testing,2024-09:Release" --style arrow --color cyan --animate
echo ""
sleep $PAUSE

# ============================================================================
# NOTIFICATIONS (NEW!)
# ============================================================================
$TERMGFX box "NOTIFICATIONS" --style info --border double
sleep 0.5

$TERMGFX notification "Build completed successfully!" --title "termgfx" --style success --terminal-only
sleep 0.5
$TERMGFX notification "Check your test results" --title "CI/CD" --style warning --terminal-only
sleep 0.5
$TERMGFX notification "Deployment failed!" --title "Alert" --style error --terminal-only
sleep $PAUSE

# ============================================================================
# DASHBOARD (NEW!)
# ============================================================================
$TERMGFX box "DASHBOARD" --style info --border double
sleep 0.5

echo "  Multi-panel dashboard:"
$TERMGFX dashboard --layout "2x2" --title "System Monitor" --panels "box:CPU OK,progress:75,sparkline:10,20,30,40,50,gauge:60" --border rounded
sleep $PAUSE

# ============================================================================
# ANIMATIONS
# ============================================================================
$TERMGFX box "ANIMATION EFFECTS" --style info --border double
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
# SCRIPT RUNNER
# ============================================================================
$TERMGFX box "SCRIPT RUNNER" --style info --border double
sleep 0.5

$TERMGFX script --inline 'box "From script!" style:success
progress 50
sparkline 1,2,3,4,5,4,3,2,1'
sleep $PAUSE

# ============================================================================
# FOOTER
# ============================================================================
echo ""
$TERMGFX banner "COMPLETE" --gradient purple-cyan --animate
$TERMGFX box "25 commands | 293 tests | TTY-aware animations" --style gradient --border double --animate
echo ""
echo "  GitHub: https://github.com/ybouhjira/termgfx"
echo ""
