<p align="center">
  <pre>
████████╗███████╗██████╗ ███╗   ███╗ ██████╗ ███████╗██╗  ██╗
╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝ ██╔════╝╚██╗██╔╝
   ██║   █████╗  ██████╔╝██╔████╔██║██║  ███╗█████╗   ╚███╔╝
   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║   ██║██╔══╝   ██╔██╗
   ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╔╝██║     ██╔╝ ██╗
   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝
  </pre>
  <h3>🎨 Beautiful Terminal Graphics for CLI Applications</h3>
  <p>Styled boxes, charts, images, spinners, and interactive prompts - all in one blazing fast Rust CLI</p>
</p>

<p align="center">
  <a href="https://github.com/ybouhjira/termgfx/actions"><img src="https://github.com/ybouhjira/termgfx/workflows/CI/badge.svg" alt="CI Status"></a>
  <a href="https://crates.io/crates/termgfx"><img src="https://img.shields.io/crates/v/termgfx.svg" alt="Crates.io"></a>
  <a href="https://github.com/ybouhjira/termgfx"><img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust Version"></a>
  <a href="https://github.com/ybouhjira/termgfx/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  <a href="https://github.com/ybouhjira/termgfx"><img src="https://img.shields.io/github/stars/ybouhjira/termgfx?style=social" alt="GitHub Stars"></a>
</p>

---

## ✨ Features at a Glance

| Feature | Description |
|---------|-------------|
| 📦 **Styled Boxes** | Beautiful message boxes with borders and gradients |
| 🎨 **Banners** | ASCII art titles with gradient colors |
| ⏳ **Spinners** | Animated loading indicators |
| 📊 **Progress Bars** | Visual progress with multiple styles |
| 📈 **Charts** | Line, bar, pie charts and sparklines |
| 🖼️ **Images** | Display images in terminal (Kitty, Sixel, halfblock) |
| 💬 **Interactive Prompts** | Input, select, multi-select, confirm dialogs |

---

## 🚀 Quick Start

### Installation

```bash
# From crates.io (recommended)
cargo install termgfx

# From source
git clone https://github.com/ybouhjira/termgfx
cd termgfx && cargo install --path .

# Homebrew (coming soon)
brew install termgfx
```

### Your First Commands

```bash
# Display a styled box
termgfx box "Hello, World!" --style success --border rounded

# Show a gradient banner
termgfx banner "TERMGFX" --gradient cyan-purple

# Quick progress bar
termgfx progress 75 --style blocks

# Interactive selection
termgfx select "Choose your OS:" "Linux" "macOS" "Windows"
```

---

## 📖 Command Reference

### 📦 Box - Styled Message Boxes

Display messages in beautiful bordered boxes with colors and emojis.

```bash
termgfx box <MESSAGE> [OPTIONS]
```

| Option | Description | Values | Default |
|--------|-------------|--------|---------|
| `-s, --style` | Color style | `info`, `success`, `warning`, `danger`, `gradient` | `info` |
| `-b, --border` | Border style | `single`, `double`, `rounded`, `thick` | `rounded` |
| `-e, --emoji` | Add emoji | Any emoji | none |

**Examples:**

```bash
# Success message
termgfx box "Build completed!" --style success --emoji "✅"

# Warning with double border
termgfx box "Disk space low" --style warning --border double

# Danger alert
termgfx box "Critical error occurred!" --style danger --border thick
```

**Output:**
```
╭──────────────────────────╮
│ ✅ Build completed!      │
╰──────────────────────────╯
```

---

### 🎨 Banner - ASCII Art Titles

Create eye-catching ASCII banners with gradient colors.

```bash
termgfx banner <TITLE> [OPTIONS]
```

| Option | Description | Values | Default |
|--------|-------------|--------|---------|
| `-g, --gradient` | Color gradient | `blue-purple`, `cyan-pink`, `green-yellow`, `red-orange` | none |

**Examples:**

```bash
# Simple banner
termgfx banner "HELLO"

# With gradient
termgfx banner "TERMGFX" --gradient blue-purple
```

**Output:**
```
████████╗███████╗██████╗ ███╗   ███╗ ██████╗ ███████╗██╗  ██╗
╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝ ██╔════╝╚██╗██╔╝
   ██║   █████╗  ██████╔╝██╔████╔██║██║  ███╗█████╗   ╚███╔╝
   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║   ██║██╔══╝   ██╔██╗
   ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╔╝██║     ██╔╝ ██╗
   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝
```

---

### ⏳ Spinner - Loading Indicators

Show animated spinners while tasks are running.

```bash
termgfx spinner <MESSAGE> [OPTIONS]
```

| Option | Description | Values | Default |
|--------|-------------|--------|---------|
| `-s, --style` | Animation style | `dots`, `circle`, `bounce`, `moon` | `dots` |

**Examples:**

```bash
# Default dots spinner
termgfx spinner "Loading..."

# Moon phases
termgfx spinner "Processing..." --style moon

# Bouncing ball
termgfx spinner "Please wait..." --style bounce
```

**Output:**
```
⠋ Loading...
```

---

### 📊 Progress - Progress Bars

Display beautiful progress indicators.

```bash
termgfx progress <PERCENT> [OPTIONS]
```

| Option | Description | Values | Default |
|--------|-------------|--------|---------|
| `-s, --style` | Visual style | `gradient`, `animated`, `blocks` | `gradient` |

**Examples:**

```bash
# Gradient progress
termgfx progress 65 --style gradient

# Block style
termgfx progress 80 --style blocks

# Animated
termgfx progress 50 --style animated
```

**Output:**
```
Progress: [████████████████░░░░░░░░] 65%
```

---

### 📈 Charts - Data Visualization

Create terminal charts for quick data visualization.

#### Line Chart

```bash
termgfx chart line --data "1,3,2,5,4,6" --title "Sales Trend"
```

**Output:**
```
Sales Trend
    │
  6 ┤                    ╭─
  5 ┤          ╭─╮      ╭╯
  4 ┤         ╭╯ ╰╮    ╭╯
  3 ┤   ╭─╮  ╭╯   ╰╮  ╭╯
  2 ┤  ╭╯ ╰──╯     ╰──╯
  1 ┼──╯
    └────────────────────
```

#### Bar Chart

```bash
termgfx chart bar --data "Jan:10,Feb:25,Mar:18,Apr:30"
```

**Output:**
```
Jan ████████░░░░░░░░░░░░ 10
Feb ████████████████████ 25
Mar ██████████████░░░░░░ 18
Apr ████████████████████████ 30
```

#### Pie Chart

```bash
termgfx chart pie --data "Desktop:45,Mobile:35,Tablet:20"
```

**Output:**
```
    ╭──────────╮
   ╱  Desktop  ╲    Desktop: 45% ████████████████
  │    45%     │    Mobile:  35% ████████████
   ╲  Mobile   ╱    Tablet:  20% ███████
    ╰──────────╯
```

#### Sparkline

```bash
termgfx sparkline --data "1,3,2,5,4,7,6,8"
```

**Output:**
```
▁▃▂▅▄▇▆█
```

---

### 🖼️ Image - Terminal Images

Display images directly in your terminal.

```bash
termgfx image <PATH> [OPTIONS]
```

| Option | Description | Values | Default |
|--------|-------------|--------|---------|
| `-p, --protocol` | Rendering protocol | `auto`, `kitty`, `sixel`, `halfblock` | `auto` |

**Examples:**

```bash
# Auto-detect best protocol
termgfx image ./photo.png

# Force Kitty protocol
termgfx image ./logo.jpg --protocol kitty

# Use halfblock for compatibility
termgfx image ./chart.png --protocol halfblock
```

---

### 💬 Interactive Prompts

Build interactive CLI experiences with user prompts.

#### Text Input

```bash
termgfx input <PROMPT> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-P, --placeholder` | Placeholder text |
| `--password` | Hide input (password mode) |

**Examples:**

```bash
# Simple input
termgfx input "Enter your name:"

# With placeholder
termgfx input "Email:" --placeholder "user@example.com"

# Password input
termgfx input "Password:" --password
```

#### Select (Single Choice)

```bash
termgfx select <PROMPT> <OPTIONS...>
```

**Examples:**

```bash
termgfx select "Choose framework:" "React" "Vue" "Svelte" "Angular"
```

**Output:**
```
? Choose framework:
  ❯ React
    Vue
    Svelte
    Angular
```

#### Choose (Multi-select)

```bash
termgfx choose <PROMPT> <OPTIONS...> [--multi]
```

**Examples:**

```bash
termgfx choose "Select features:" "Auth" "API" "Database" "Cache" --multi
```

**Output:**
```
? Select features:
  ◉ Auth
  ◯ API
  ◉ Database
  ◯ Cache
```

#### Confirm (Yes/No)

```bash
termgfx confirm <PROMPT> [OPTIONS]
```

| Option | Description | Values | Default |
|--------|-------------|--------|---------|
| `-d, --default` | Default answer | `yes`, `no` | `yes` |
| `-S, --style` | Visual style | `normal`, `danger` | `normal` |

**Examples:**

```bash
# Simple confirmation
termgfx confirm "Continue?"

# Dangerous action
termgfx confirm "Delete all files?" --default no --style danger
```

---

## 🔥 Why TermGFX over Gum?

| Feature | TermGFX | Gum |
|---------|---------|-----|
| ⚡ **Speed** | Blazing fast (Rust) | Fast (Go) |
| 📈 **Charts** | Line, Bar, Pie, Sparkline | ❌ No charts |
| 🖼️ **Images** | Kitty, Sixel, Halfblock | ❌ No images |
| 🎨 **Gradients** | Full gradient support | Limited |
| 📦 **Single Binary** | Yes, ~2MB | Yes, ~5MB |
| 🔧 **Customization** | Extensive options | Good options |
| 📊 **Data Viz** | Built-in | Requires external tools |

---

## 🛠️ Use Cases

### Shell Scripts

```bash
#!/bin/bash

# Show styled header
termgfx banner "DEPLOY" --gradient blue-purple

# Confirm before proceeding
if termgfx confirm "Deploy to production?" --style danger; then
    termgfx spinner "Deploying..." &
    SPINNER_PID=$!
    # ... deployment commands ...
    kill $SPINNER_PID 2>/dev/null
    termgfx box "Deployment complete!" --style success --emoji "🚀"
fi
```

### Progress in Scripts

```bash
#!/bin/bash
for i in {0..100..10}; do
    termgfx progress $i --style blocks
    sleep 0.5
    printf "\033[1A"  # Move cursor up
done
termgfx box "Download complete!" --style success
```

### Interactive Menus

```bash
#!/bin/bash
CHOICE=$(termgfx select "What would you like to do?" \
    "Run tests" \
    "Build project" \
    "Deploy" \
    "Exit")

case "$CHOICE" in
    "Run tests") npm test ;;
    "Build project") npm run build ;;
    "Deploy") ./deploy.sh ;;
esac
```

---

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/ybouhjira/termgfx
cd termgfx

# Build release binary
cargo build --release

# The binary will be at ./target/release/termgfx
```

### Requirements

- Rust 1.70 or later
- A terminal with ANSI color support

---

## 📚 Documentation

- [Full API Reference](https://docs.rs/termgfx)
- [Contributing Guide](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/termgfx
cd termgfx

# Create feature branch
git checkout -b feature/amazing-feature

# Make changes and test
cargo test

# Submit PR
```

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Credits

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [Clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/ybouhjira">Youssef Bouhjira</a>
</p>
