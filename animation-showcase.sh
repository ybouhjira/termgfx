#!/bin/bash
# Animation Showcase for TermGFX
# Demonstrates all animation effects and spinners

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
MAGENTA='\033[0;35m'
NC='\033[0m'

clear

# Header
termgfx banner "Animation Showcase" --style gradient --font slant 2>/dev/null || echo "=== Animation Showcase ==="
echo ""

# ============================================================================
# SECTION 1: All Spinners Side by Side (using parallel processes)
# ============================================================================
echo -e "${MAGENTA}━━━ ALL SPINNERS (2 seconds each) ━━━${NC}"
echo ""

# Run all 8 spinners sequentially with labels
spinners=("dots" "line" "arc" "bouncing" "clock" "circle" "bounce" "moon")

for style in "${spinners[@]}"; do
    printf "  %-10s " "$style:"
    termgfx spinner "Loading..." --style "$style" --duration 1
    echo ""
done

echo ""
echo -e "${GREEN}✓ All spinner styles demonstrated${NC}"
echo ""

# ============================================================================
# SECTION 2: Progress Bar Animations
# ============================================================================
echo -e "${MAGENTA}━━━ PROGRESS BAR ANIMATIONS ━━━${NC}"
echo ""

echo -e "${CYAN}Gradient style:${NC}"
termgfx animate --effect-type progress --duration 2 --style gradient
echo ""

echo -e "${CYAN}Blocks style:${NC}"
termgfx animate --effect-type progress --duration 2 --style blocks
echo ""

echo -e "${CYAN}Classic style:${NC}"
termgfx animate --effect-type progress --duration 2 --style classic
echo ""

# ============================================================================
# SECTION 3: Typewriter Effect
# ============================================================================
echo -e "${MAGENTA}━━━ TYPEWRITER ANIMATION ━━━${NC}"
echo ""

echo -e "${CYAN}Fast typing (60 chars/sec):${NC}"
termgfx animate --effect-type typewriter --text "Welcome to TermGFX - Beautiful Terminal Graphics!" --speed 60
echo ""
echo ""

echo -e "${CYAN}Slow typing (20 chars/sec):${NC}"
termgfx animate --effect-type typewriter --text "Every character appears one by one..." --speed 20
echo ""
echo ""

# ============================================================================
# SECTION 4: Counter Animation
# ============================================================================
echo -e "${MAGENTA}━━━ COUNTER ANIMATIONS ━━━${NC}"
echo ""

echo -e "${CYAN}Count from 0 to 100:${NC}"
termgfx animate --effect-type counter --from 0 --to 100 --duration 2
echo ""

echo -e "${CYAN}Percentage counter:${NC}"
termgfx animate --effect-type counter --from 0 --to 100 --suffix "%" --duration 2
echo ""

echo -e "${CYAN}Download counter:${NC}"
termgfx animate --effect-type counter --from 0 --to 1024 --suffix " MB" --prefix "Downloaded: " --duration 2
echo ""

# ============================================================================
# SECTION 5: Chart Build Animation
# ============================================================================
echo -e "${MAGENTA}━━━ CHART BUILD ANIMATION ━━━${NC}"
echo ""

echo -e "${CYAN}Bar chart appearing:${NC}"
termgfx animate --effect-type chart-build --data "Sales:80,Costs:40,Profit:60,Marketing:30" --duration 3
echo ""

# ============================================================================
# SECTION 6: Bars Animation
# ============================================================================
echo -e "${MAGENTA}━━━ BARS ANIMATION ━━━${NC}"
echo ""

echo -e "${CYAN}Animated bars:${NC}"
termgfx animate --effect-type bars --data "Q1:25,Q2:45,Q3:65,Q4:85" --duration 3
echo ""

# ============================================================================
# SECTION 7: Parallel Animations Demo
# ============================================================================
echo -e "${MAGENTA}━━━ PARALLEL DEMO (3 spinners at once) ━━━${NC}"
echo ""

# Create a simple parallel display using subshells
echo "Running 3 spinners simultaneously for 3 seconds..."
echo ""

# We'll run 3 spinners in the background and wait
{
    echo -n "  Spinner 1: "
    termgfx spinner "dots..." --style dots --duration 3
} &
PID1=$!

sleep 1

{
    echo -n "  Spinner 2: "
    termgfx spinner "arc..." --style arc --duration 2
} &
PID2=$!

sleep 1

{
    echo -n "  Spinner 3: "
    termgfx spinner "moon..." --style moon --duration 2
} &
PID3=$!

# Wait for all to complete
wait $PID1 $PID2 $PID3 2>/dev/null

echo ""
echo ""

# ============================================================================
# FINALE
# ============================================================================
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
termgfx box "Animation Showcase Complete!" --style success --border double
echo ""
echo -e "${YELLOW}Animation types demonstrated:${NC}"
echo "  • 8 Spinner styles (dots, line, arc, bouncing, clock, circle, bounce, moon)"
echo "  • 3 Progress bar styles (gradient, blocks, classic)"
echo "  • Typewriter effect (variable speed)"
echo "  • Counter animation (with prefix/suffix)"
echo "  • Chart build animation"
echo "  • Bars animation"
echo ""
echo -e "${CYAN}Run individual animations:${NC}"
echo "  termgfx spinner \"Loading...\" --style moon --duration 3"
echo "  termgfx animate --effect-type typewriter --text \"Hello\" --speed 30"
echo "  termgfx animate --effect-type progress --style gradient --duration 2"
echo ""
