#!/bin/bash
# TermGFX Studio Demo Script
# Demonstrates the interactive TUI component explorer

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Build if needed
if [[ ! -f "$PROJECT_DIR/target/release/termgfx" ]]; then
    echo -e "${YELLOW}Building termgfx...${NC}"
    cd "$PROJECT_DIR" && cargo build --release
fi

TERMGFX="$PROJECT_DIR/target/release/termgfx"

clear

echo -e "${BOLD}${CYAN}"
cat << 'EOF'
╔════════════════════════════════════════════════════════════════╗
║                                                                ║
║   ████████╗███████╗██████╗ ███╗   ███╗ ██████╗ ███████╗██╗  ██╗ ║
║   ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝ ██╔════╝╚██╗██╔╝ ║
║      ██║   █████╗  ██████╔╝██╔████╔██║██║  ███╗█████╗   ╚███╔╝  ║
║      ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║   ██║██╔══╝   ██╔██╗  ║
║      ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╔╝██║     ██╔╝ ██╗ ║
║      ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝ ║
║                                                                ║
║                    S T U D I O   D E M O                       ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${GREEN}Welcome to TermGFX Studio!${NC}"
echo ""
echo -e "TermGFX Studio is a ${BOLD}fullscreen IDE-like TUI${NC} for exploring"
echo -e "and configuring all termgfx components with ${BOLD}live preview${NC}."
echo ""

echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}Features:${NC}"
echo ""
echo -e "  ${CYAN}◆${NC} Browse all components in categorized sidebar"
echo -e "  ${CYAN}◆${NC} Edit parameters with custom widgets (sliders, dropdowns)"
echo -e "  ${CYAN}◆${NC} Live preview updates as you change values"
echo -e "  ${CYAN}◆${NC} Copy generated command with one key"
echo -e "  ${CYAN}◆${NC} Mouse support: click, scroll, drag to resize"
echo -e "  ${CYAN}◆${NC} Keyboard-driven: vim-style navigation"
echo -e "  ${CYAN}◆${NC} Save favorites for quick access"
echo -e "  ${CYAN}◆${NC} History tracking of recent configurations"
echo ""
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${BOLD}Keyboard Shortcuts:${NC}"
echo ""
echo -e "  ${YELLOW}Navigation${NC}"
echo -e "    Tab/Shift+Tab  - Cycle panels"
echo -e "    1/2/3          - Jump to panel"
echo -e "    j/k or ↑/↓     - Navigate items"
echo -e "    Enter/l        - Select/Edit"
echo ""
echo -e "  ${YELLOW}Editing${NC}"
echo -e "    Space          - Toggle bool / Cycle enum"
echo -e "    h/l or ←/→     - Adjust slider"
echo -e "    r              - Reset parameters"
echo ""
echo -e "  ${YELLOW}Actions${NC}"
echo -e "    c              - Copy command"
echo -e "    s              - Save favorite"
echo -e "    d              - Delete favorite"
echo -e "    f/H            - Jump to Favorites/History"
echo -e "    ?              - Show help overlay"
echo -e "    q/Esc          - Quit"
echo ""
echo -e "  ${YELLOW}Resizing${NC}"
echo -e "    Ctrl+←/→       - Resize sidebar"
echo -e "    Ctrl+↑/↓       - Resize params panel"
echo -e "    Shift+R        - Reset layout"
echo -e "    Mouse drag     - Drag panel dividers"
echo ""
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${GREEN}Press Enter to launch TermGFX Studio...${NC}"
read -r

# Launch studio
exec "$TERMGFX" studio
