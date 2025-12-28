#!/bin/bash
# Interactive Demo for TermGFX
# Showcases all major features

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

clear

show_header() {
    clear
    termgfx banner "TermGFX Demo" --style gradient --font slant 2>/dev/null || echo "=== TermGFX Demo ==="
    echo ""
}

press_enter() {
    echo ""
    echo -e "${CYAN}Press Enter to continue...${NC}"
    read -r
}

demo_boxes() {
    show_header
    echo -e "${GREEN}=== Styled Boxes ===${NC}"
    echo ""

    echo "Info style:"
    termgfx box "Welcome to TermGFX!" --style info
    echo ""

    echo "Success style:"
    termgfx box "Operation completed successfully" --style success
    echo ""

    echo "Warning style:"
    termgfx box "Please review before continuing" --style warning
    echo ""

    echo "Danger style:"
    termgfx box "This action cannot be undone!" --style danger
    echo ""

    echo "Different borders:"
    termgfx box "Rounded border" --border rounded --style info
    termgfx box "Double border" --border double --style success

    press_enter
}

demo_progress() {
    show_header
    echo -e "${GREEN}=== Progress Bars ===${NC}"
    echo ""

    echo "Animated gradient progress bar:"
    for i in 0 20 40 60 80 100; do
        printf "\r"
        termgfx progress $i --style gradient
        sleep 0.3
    done
    echo ""
    echo ""

    echo "Block progress bar:"
    termgfx progress 75 --style blocks
    echo ""

    echo "Classic progress bar:"
    termgfx progress 60 --style classic

    press_enter
}

demo_charts() {
    show_header
    echo -e "${GREEN}=== Charts ===${NC}"
    echo ""

    echo "Bar chart:"
    termgfx chart bar --data "Sales:100,Costs:60,Profit:40,Marketing:25"
    echo ""

    echo "Sparkline:"
    termgfx sparkline "10,25,15,40,30,50,45,60,55,70"
    echo ""

    echo "Pie chart:"
    termgfx chart pie --data "JavaScript:40,Python:30,Rust:20,Go:10"

    press_enter
}

demo_spinners() {
    show_header
    echo -e "${GREEN}=== Spinners ===${NC}"
    echo ""

    echo "Dots spinner (2s):"
    termgfx spinner "Loading..." --style dots --duration 2
    echo ""

    echo "Arc spinner (2s):"
    termgfx spinner "Processing..." --style arc --duration 2
    echo ""

    echo "Moon spinner (2s):"
    termgfx spinner "Please wait..." --style moon --duration 2

    press_enter
}

demo_tables() {
    show_header
    echo -e "${GREEN}=== Tables ===${NC}"
    echo ""

    echo "Basic table:"
    termgfx table --headers "Name,Language,Stars" --rows "React,JavaScript,200k|Vue,JavaScript,200k|Svelte,JavaScript,70k|Angular,TypeScript,90k"
    echo ""

    echo "With double border style:"
    termgfx table --headers "Feature,Status" --rows "Export,Done|Preview,Done|Filter,Done" --border double

    press_enter
}

demo_trees() {
    show_header
    echo -e "${GREEN}=== Tree Structures ===${NC}"
    echo ""

    echo "Project structure:"
    termgfx tree "termgfx>src,tests,docs>main.rs,lib.rs,output>e2e_box.rs,e2e_chart.rs>README.md"

    press_enter
}

demo_preview() {
    show_header
    echo -e "${GREEN}=== Preview Pane ===${NC}"
    echo ""

    echo "File deletion preview (danger style):"
    termgfx preview --title "Files to delete" --items "cache.db,temp.log,debug.log" --action "Delete All" --style danger
    echo ""

    echo "With columns (tabular data):"
    termgfx preview --title "Log files" --items "access.log|15KB|Jan 15,error.log|3KB|Jan 14,debug.log|128KB|Jan 10" --columns "Name,Size,Date" --action "Clean up"

    press_enter
}

demo_regex_filter() {
    show_header
    echo -e "${GREEN}=== Regex Filter ===${NC}"
    echo ""

    echo "Filter .log files:"
    termgfx regex-filter --pattern '\.log$' --items "app.log,config.json,error.log,debug.log,settings.yaml" --action "Select"
    echo ""

    echo "Case-insensitive error/warning matching:"
    termgfx regex-filter --pattern 'error|warn' -I --items "ERROR.log,warning.txt,info.log,config.json" --action "Review"

    press_enter
}

demo_themes() {
    show_header
    echo -e "${GREEN}=== Themes ===${NC}"
    echo ""

    echo "Available themes:"
    termgfx theme list
    echo ""

    echo "Nord theme preview:"
    termgfx theme preview nord

    press_enter
}

demo_export() {
    show_header
    echo -e "${GREEN}=== SVG Export ===${NC}"
    echo ""

    echo "Export box to SVG (showing first 10 lines):"
    termgfx export box "Hello TermGFX!" --style success | head -10
    echo "..."
    echo ""

    echo "Export progress bar to SVG:"
    termgfx export progress 75 --style info | head -10
    echo "..."
    echo ""

    echo -e "${YELLOW}Save to file: termgfx export box 'Hello' -o output.svg${NC}"

    press_enter
}

demo_danger_zone() {
    show_header
    echo -e "${GREEN}=== Danger Zone ===${NC}"
    echo ""

    echo "Warning box for destructive operations:"
    termgfx danger-zone "This will permanently delete all data.
Are you sure you want to continue?" --title "DATABASE WIPE"

    press_enter
}

demo_interactive() {
    show_header
    echo -e "${GREEN}=== Interactive Prompts ===${NC}"
    echo ""

    echo "These require user input - try them yourself!"
    echo ""
    echo -e "  ${CYAN}termgfx input \"What's your name?\"${NC}"
    echo -e "  ${CYAN}termgfx select \"Pick a color\" --options \"Red,Green,Blue\"${NC}"
    echo -e "  ${CYAN}termgfx confirm \"Continue?\"${NC}"
    echo -e "  ${CYAN}echo -e 'file1\\nfile2\\nfile3' | termgfx filter${NC}"

    press_enter
}

show_menu() {
    show_header
    echo -e "${MAGENTA}Select a demo to run:${NC}"
    echo ""
    echo "  1) Styled Boxes"
    echo "  2) Progress Bars"
    echo "  3) Charts (Bar, Pie, Sparkline)"
    echo "  4) Spinners"
    echo "  5) Tables"
    echo "  6) Tree Structures"
    echo "  7) Preview Pane"
    echo "  8) Regex Filter"
    echo "  9) Themes"
    echo " 10) SVG Export"
    echo " 11) Danger Zone"
    echo " 12) Interactive Prompts"
    echo ""
    echo "  a) Run ALL demos"
    echo "  q) Quit"
    echo ""
    echo -n "Choice: "
}

run_all() {
    demo_boxes
    demo_progress
    demo_charts
    demo_spinners
    demo_tables
    demo_trees
    demo_preview
    demo_regex_filter
    demo_themes
    demo_export
    demo_danger_zone
    demo_interactive

    show_header
    echo -e "${GREEN}Demo complete!${NC}"
    echo ""
    termgfx box "Thanks for trying TermGFX!" --style success --border double
    echo ""
    echo "Install: cargo install --git https://github.com/ybouhjira/termgfx"
    echo "Docs:    termgfx --help"
}

# Main loop
while true; do
    show_menu
    read -r choice

    case $choice in
        1) demo_boxes ;;
        2) demo_progress ;;
        3) demo_charts ;;
        4) demo_spinners ;;
        5) demo_tables ;;
        6) demo_trees ;;
        7) demo_preview ;;
        8) demo_regex_filter ;;
        9) demo_themes ;;
        10) demo_export ;;
        11) demo_danger_zone ;;
        12) demo_interactive ;;
        a|A) run_all ;;
        q|Q)
            clear
            echo "Goodbye!"
            exit 0
            ;;
        *)
            echo "Invalid choice"
            sleep 1
            ;;
    esac
done
