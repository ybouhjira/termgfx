# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TermGFX is a Rust CLI for beautiful terminal graphics - styled boxes, charts, images, spinners, and interactive prompts. Single binary (~4.5MB) with optional WASM support for browser demos.

## Build & Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build (LTO enabled)

# Run
cargo run -- <command>         # Run CLI command
./target/release/termgfx       # Run release binary

# Test
cargo test                     # Run all tests
cargo test <test_name>         # Run specific test
cargo test e2e_                # Run E2E tests only

# Lint & Format
cargo fmt --all -- --check     # Check formatting
cargo fmt                      # Fix formatting
cargo clippy --all-targets --all-features -- -D warnings  # Lint

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Xml
```

## Architecture

```
src/
├── main.rs          # CLI entry point - clap command definitions
├── lib.rs           # Library exports + WASM bindings
├── output/          # Terminal output components
│   ├── styled_box   # Styled message boxes
│   ├── banner       # ASCII art banners with gradients
│   ├── progress     # Progress bars (gradient, blocks, classic)
│   ├── spinner      # Loading spinners (dots, arc, moon, etc.)
│   ├── gauge        # Radial dial indicators
│   ├── table        # Formatted tables
│   ├── tree         # Tree structure display
│   ├── timeline     # Horizontal/vertical timelines
│   ├── heatmap      # 2D heatmap visualization
│   ├── dashboard    # Multi-panel dashboards
│   ├── diff         # File diff display
│   ├── layout       # join/columns/stack layout helpers
│   ├── watch        # Watch mode (like unix watch)
│   ├── palette      # Color palette management
│   └── style        # Style preset previews
├── charts/          # Chart visualizations
│   ├── bar          # Bar charts
│   ├── line         # Line charts with animation
│   ├── pie          # ASCII pie charts
│   └── sparkline    # Inline mini-charts
├── interactive/     # User input prompts
│   ├── input        # Text input
│   ├── select       # Single/multi select
│   ├── confirm      # Yes/No confirmation
│   ├── file         # File/directory picker
│   ├── filter       # Fuzzy filter (fzf-like)
│   ├── pager        # Scrollable pager (less-like)
│   ├── form         # Multi-field forms
│   ├── wizard       # Multi-step wizards
│   ├── tui          # Full TUI mode with ratatui
│   └── playground   # Interactive component explorer
├── animation/       # Animation engine
│   ├── engine       # Core animation loop
│   ├── effects      # Typewriter, counter, etc.
│   └── demo         # Demo showcase
├── image/           # Image rendering (Kitty, Sixel, halfblock)
├── design/          # Theme system and presets
│   ├── theme        # Theme definitions
│   └── presets      # Corporate, playful, minimal, etc.
├── export/          # Export functionality
│   └── svg          # SVG export
└── script.rs        # Script runner for automation
```

## Key Dependencies

- **clap**: CLI parsing with derive macros
- **ratatui + crossterm**: TUI framework for interactive modes
- **ratatui-image**: Terminal image protocol support
- **owo-colors**: ANSI color styling
- **serde/serde_json**: Serialization for configs

## Feature Flags

- `cli` (default): Full CLI with all dependencies
- `wasm`: WebAssembly bindings for browser demos

## Testing

Tests are in `tests/` directory using:
- **assert_cmd**: CLI output assertions
- **predicates**: Output matching
- **rexpect**: PTY-based interactive testing
- **insta**: Snapshot testing

Run interactive tests: `cargo test e2e_interactive` (requires TTY)

## CI Pipeline

GitHub Actions runs on push/PR to master:
1. Format check (`cargo fmt`)
2. Clippy lints
3. Build (ubuntu + macos)
4. Tests
5. Coverage with tarpaulin
