#!/bin/bash
# TermGFX Demo: ANIMATED (Current Capabilities)
# Shows what's possible NOW with shell loops + termgfx

TERMGFX="./target/release/termgfx"
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
DIM='\033[2m'
BOLD='\033[1m'
NC='\033[0m'

# Typewriter effect
typewriter() {
    local text="$1"
    local delay="${2:-0.03}"
    for ((i=0; i<${#text}; i++)); do
        echo -n "${text:$i:1}"
        sleep "$delay"
    done
    echo ""
}

# Animated progress bar
animate_progress() {
    local label="$1"
    local style="${2:-gradient}"
    local speed="${3:-2}"

    echo -e "${DIM}$label${NC}"
    for i in $(seq 0 $speed 100); do
        echo -ne "\033[1A\033[K"  # Move up, clear line
        echo -e "${DIM}$label${NC}"
        $TERMGFX progress $i --style "$style"
        sleep 0.03
    done
}

# Animated counter
animate_counter() {
    local from="$1"
    local to="$2"
    local prefix="$3"
    local step="${4:-1}"

    for i in $(seq $from $step $to); do
        printf "\r${BOLD}${prefix}${i}${NC}  "
        sleep 0.02
    done
    echo ""
}

# Chart reveal (data point by point)
animate_sparkline() {
    local data="$1"
    IFS=',' read -ra points <<< "$data"
    local partial=""

    for point in "${points[@]}"; do
        if [ -n "$partial" ]; then
            partial="${partial},${point}"
        else
            partial="$point"
        fi
        echo -ne "\033[1A\033[K"
        $TERMGFX sparkline "$partial"
        sleep 0.15
    done
}

# Clear and hide cursor
clear
tput civis
trap 'tput cnorm; exit' INT TERM EXIT

# ═══════════════════════════════════════════════════════════════
# INTRO
# ═══════════════════════════════════════════════════════════════

echo ""
$TERMGFX banner "termgfx" --gradient cyan-purple
echo ""
sleep 0.5

typewriter "  Terminal Graphics Library - Animated Demo" 0.02
echo ""
sleep 0.3
typewriter "  Showcasing what's possible with current capabilities..." 0.015
echo ""
sleep 1

# ═══════════════════════════════════════════════════════════════
# SECTION 1: ANIMATED PROGRESS BARS
# ═══════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}━━━ ANIMATED PROGRESS BARS ━━━${NC}"
echo ""
sleep 0.3

typewriter "  Downloading packages..." 0.02
echo ""
animate_progress "  npm install" "gradient" 3

sleep 0.3
typewriter "  Compiling source..." 0.02
echo ""
animate_progress "  cargo build" "blocks" 4

sleep 0.3
typewriter "  Running tests..." 0.02
echo ""
animate_progress "  pytest" "thin" 5

$TERMGFX box "All tasks completed!" --style success
echo ""
sleep 1

# ═══════════════════════════════════════════════════════════════
# SECTION 2: ANIMATED COUNTERS
# ═══════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}━━━ ANIMATED COUNTERS ━━━${NC}"
echo ""
sleep 0.3

echo -e "  ${DIM}Files processed:${NC}"
animate_counter 0 247 "  " 5
echo ""

echo -e "  ${DIM}Revenue generated:${NC}"
animate_counter 0 12500 "  \$" 250
echo ""

echo -e "  ${DIM}Users online:${NC}"
animate_counter 100 1842 "  " 35
echo ""
sleep 0.5

# ═══════════════════════════════════════════════════════════════
# SECTION 3: ANIMATED SPARKLINES
# ═══════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}━━━ ANIMATED CHARTS ━━━${NC}"
echo ""
sleep 0.3

typewriter "  Building CPU usage graph..." 0.02
echo ""  # Placeholder line for sparkline
animate_sparkline "20,35,28,45,52,48,60,75,82,68,55,42,38,25,30,45,62,78,65,48"
echo ""
sleep 0.3

typewriter "  Building stock price chart..." 0.02
echo ""
animate_sparkline "100,105,98,110,108,115,120,118,125,130,128,135,142,138,145"
echo ""
sleep 0.5

# ═══════════════════════════════════════════════════════════════
# SECTION 4: SPINNER STYLES
# ═══════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}━━━ SPINNER ANIMATIONS ━━━${NC}"
echo ""
sleep 0.3

styles=("dots" "arc" "moon" "circle" "bounce")
for style in "${styles[@]}"; do
    echo -e "  ${DIM}Style: ${style}${NC}"
    $TERMGFX spinner "Processing with $style style..." --style "$style" --duration 2
    echo -e "  ${GREEN}✓${NC} Done"
    sleep 0.2
done
echo ""
sleep 0.5

# ═══════════════════════════════════════════════════════════════
# SECTION 5: COMBINED ANIMATION
# ═══════════════════════════════════════════════════════════════

echo ""
echo -e "${CYAN}━━━ DEPLOYMENT SIMULATION ━━━${NC}"
echo ""
sleep 0.3

$TERMGFX box "Starting deployment to production..." --style info
echo ""
sleep 0.5

# Step 1
typewriter "  [1/5] Building Docker image..." 0.015
$TERMGFX spinner "  Building..." --style dots --duration 2
echo -e "  ${GREEN}✓${NC} Image built: app:v2.4.1"
echo ""

# Step 2
typewriter "  [2/5] Pushing to registry..." 0.015
animate_progress "  Upload progress" "gradient" 2
echo -e "  ${GREEN}✓${NC} Pushed to gcr.io/myproject/app:v2.4.1"
echo ""

# Step 3
typewriter "  [3/5] Scaling down old pods..." 0.015
$TERMGFX spinner "  Scaling..." --style arc --duration 1
echo -e "  ${GREEN}✓${NC} Old pods terminated"
echo ""

# Step 4
typewriter "  [4/5] Deploying new pods..." 0.015
$TERMGFX spinner "  Deploying..." --style moon --duration 2
echo -e "  ${GREEN}✓${NC} 3/3 pods running"
echo ""

# Step 5
typewriter "  [5/5] Running health checks..." 0.015
for i in 1 2 3; do
    echo -ne "\r  Checking pod $i/3..."
    sleep 0.5
done
echo -e "\r  ${GREEN}✓${NC} All pods healthy     "
echo ""

$TERMGFX box "Deployment successful!" --style success
echo ""
sleep 0.5

# Final stats
echo -e "  ${DIM}Deployment Stats:${NC}"
$TERMGFX chart bar --data "Build:45,Push:30,Deploy:25,Verify:15"
echo ""

# ═══════════════════════════════════════════════════════════════
# OUTRO
# ═══════════════════════════════════════════════════════════════

echo ""
$TERMGFX banner "Complete!" --gradient green-cyan
echo ""
typewriter "  This demo used ONLY shell scripting + termgfx commands." 0.02
typewriter "  No additional dependencies required!" 0.02
echo ""
echo -e "  ${DIM}Techniques used:${NC}"
echo -e "    • Cursor escape codes (\\033[1A, \\033[K)"
echo -e "    • Shell loops with sleep"
echo -e "    • termgfx spinner --duration for timed animations"
echo -e "    • Typewriter effect with character iteration"
echo ""
echo -e "  ${CYAN}GitHub:${NC} https://github.com/ybouhjira/termgfx"
echo ""

# Restore cursor
tput cnorm
