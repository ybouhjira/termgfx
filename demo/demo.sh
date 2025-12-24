#!/bin/bash
# termgfx Animated Demo - Full animations!

TERMGFX="cargo run --quiet --"

clear

# ============================================================================
# INTRO
# ============================================================================
$TERMGFX banner "termgfx"
sleep 0.5
$TERMGFX typewriter "The ultimate terminal graphics toolkit" --speed 20
sleep 1
clear

# ============================================================================
# ANIMATED PROGRESS BARS
# ============================================================================
$TERMGFX box "ANIMATED PROGRESS BARS" --style info --border double
echo ""
sleep 0.3

echo "Installing dependencies..."
for i in $(seq 0 5 100); do
    tput cuu1 2>/dev/null || printf "\033[1A"
    tput el 2>/dev/null || printf "\033[K"
    printf "Installing dependencies... "
    $TERMGFX progress $i --style gradient
    sleep 0.05
done
echo ""

echo "Compiling project..."
for i in $(seq 0 4 100); do
    tput cuu1 2>/dev/null || printf "\033[1A"
    tput el 2>/dev/null || printf "\033[K"
    printf "Compiling project... "
    $TERMGFX progress $i --style blocks
    sleep 0.04
done
echo ""

echo "Deploying to production..."
for i in $(seq 0 3 100); do
    tput cuu1 2>/dev/null || printf "\033[1A"
    tput el 2>/dev/null || printf "\033[K"
    printf "Deploying to production... "
    $TERMGFX progress $i --style animated
    sleep 0.03
done
echo ""

$TERMGFX box "Deployment complete!" --style success
sleep 1
clear

# ============================================================================
# LIVE SPARKLINES
# ============================================================================
$TERMGFX box "LIVE DATA MONITORING" --style info --border double
echo ""
sleep 0.3

echo "CPU Usage:"
data=""
for i in $(seq 1 20); do
    val=$((RANDOM % 60 + 20))
    [ -z "$data" ] && data="$val" || data="$data,$val"
    tput cuu1 2>/dev/null || printf "\033[1A"
    tput el 2>/dev/null || printf "\033[K"
    echo -n "CPU Usage: "
    $TERMGFX sparkline "$data"
    sleep 0.1
done
echo ""

echo "Memory:"
data=""
for i in $(seq 1 20); do
    val=$((RANDOM % 40 + 40))
    [ -z "$data" ] && data="$val" || data="$data,$val"
    tput cuu1 2>/dev/null || printf "\033[1A"
    tput el 2>/dev/null || printf "\033[K"
    echo -n "Memory: "
    $TERMGFX sparkline "$data"
    sleep 0.1
done
echo ""
sleep 1
clear

# ============================================================================
# BOXES CASCADE
# ============================================================================
$TERMGFX box "STYLED BOXES" --style info --border double
echo ""
sleep 0.3

$TERMGFX typewriter "Info..." --speed 30
$TERMGFX box "This is an informational message" --style info
sleep 0.3

$TERMGFX typewriter "Success..." --speed 30
$TERMGFX box "Operation completed successfully!" --style success
sleep 0.3

$TERMGFX typewriter "Warning..." --speed 30
$TERMGFX box "Please review before continuing" --style warning
sleep 0.3

$TERMGFX typewriter "Danger..." --speed 30
$TERMGFX box "Critical error detected!" --style danger
sleep 1
clear

# ============================================================================
# TABLE + TREE
# ============================================================================
$TERMGFX box "DATA VISUALIZATION" --style info --border double
echo ""

$TERMGFX typewriter "Building status table..." --speed 25
sleep 0.3
$TERMGFX table --headers "Command,Status,Type" --rows "box,Ready,Output|banner,Ready,Output|progress,Ready,Output|table,Ready,Output|tree,Ready,Output|diff,Ready,Output|script,Ready,Automation" --border rounded
echo ""
sleep 0.5

$TERMGFX typewriter "Scanning project..." --speed 25
sleep 0.3
echo '{"src":{"main.rs":"entry","output":{"box.rs":"done","banner.rs":"done","table.rs":"done"},"charts":{"line.rs":"done","pie.rs":"done"}},"Cargo.toml":"config"}' | $TERMGFX tree
sleep 1
clear

# ============================================================================
# CHARTS
# ============================================================================
$TERMGFX box "CHARTS" --style info --border double
echo ""

$TERMGFX typewriter "Revenue growth..." --speed 25
$TERMGFX chart line --data "10,25,18,35,42,55,62,70,85,90" --title "2024 Revenue"
sleep 1

echo ""
$TERMGFX typewriter "Sales by quarter..." --speed 25
$TERMGFX chart bar --data "Q1:35,Q2:48,Q3:62,Q4:85"
sleep 1

echo ""
$TERMGFX typewriter "Market share..." --speed 25
$TERMGFX chart pie --data "Us:55,Competitor:30,Other:15"
sleep 1
clear

# ============================================================================
# DIFF
# ============================================================================
$TERMGFX box "DIFF COMPARISON" --style info --border double
echo ""

echo "v1.0" > /tmp/v1.txt
echo "Initial release" >> /tmp/v1.txt
echo "v2.0" > /tmp/v2.txt
echo "Initial release" >> /tmp/v2.txt
echo "New features" >> /tmp/v2.txt
echo "Bug fixes" >> /tmp/v2.txt

$TERMGFX typewriter "Comparing versions..." --speed 25
sleep 0.3
$TERMGFX diff /tmp/v1.txt /tmp/v2.txt
rm /tmp/v1.txt /tmp/v2.txt
sleep 1
clear

# ============================================================================
# SCRIPT ANIMATION
# ============================================================================
$TERMGFX box "SCRIPT SEQUENCES" --style info --border double
echo ""

$TERMGFX typewriter "Running animation script..." --speed 25
sleep 0.3

$TERMGFX script --inline "box \"Step 1: Initialize\" style:info
wait 400ms
box \"Step 2: Process data\" style:warning
wait 400ms
box \"Step 3: Complete!\" style:success
wait 300ms
typewriter \"Animation finished!\" speed:20"
sleep 1
clear

# ============================================================================
# FINALE
# ============================================================================
$TERMGFX banner "Complete"
sleep 0.3

for i in 25 50 75 100; do
    tput cuu1 2>/dev/null || printf "\033[1A"
    tput el 2>/dev/null || printf "\033[K"
    $TERMGFX progress $i --style gradient
    sleep 0.2
done
echo ""

$TERMGFX box "All features demonstrated!" --style success --border double
echo ""
$TERMGFX typewriter "github.com/ybouhjira/termgfx" --speed 15
echo ""
