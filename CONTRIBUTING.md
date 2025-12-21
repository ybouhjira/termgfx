# 🤝 Contributing to TermGFX

First off, thanks for taking the time to contribute! 🎉

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Running Tests](#running-tests)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Commit Messages](#commit-messages)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

---

## 📜 Code of Conduct

Be kind, respectful, and constructive. We're all here to build something great together.

---

## 🚀 Getting Started

### Prerequisites

- **Rust 1.70+** - Install via [rustup](https://rustup.rs/)
- **Git** - For version control
- **A terminal with ANSI support** - For testing outputs

### Fork & Clone

```bash
# Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/termgfx
cd termgfx

# Add upstream remote
git remote add upstream https://github.com/ybouhjira/termgfx
```

---

## 🛠️ Development Setup

```bash
# Install dependencies and build
cargo build

# Run the CLI
cargo run -- box "Hello" --style success

# Run in release mode
cargo run --release -- banner "TERMGFX"
```

### Project Structure

```
termgfx/
├── src/
│   ├── main.rs           # CLI entry point, argument parsing
│   ├── output/           # Output components
│   │   ├── mod.rs
│   │   ├── styled_box.rs # Box rendering
│   │   ├── banner.rs     # ASCII banners
│   │   ├── spinner.rs    # Loading spinners
│   │   └── progress.rs   # Progress bars
│   ├── charts/           # Chart components
│   │   ├── mod.rs
│   │   ├── line.rs       # Line charts
│   │   ├── bar.rs        # Bar charts
│   │   ├── pie.rs        # Pie charts
│   │   └── sparkline.rs  # Sparklines
│   ├── image/            # Image rendering
│   │   └── mod.rs
│   └── interactive/      # User prompts
│       ├── mod.rs
│       ├── input.rs      # Text input
│       ├── select.rs     # Single select
│       ├── choose.rs     # Multi-select
│       └── confirm.rs    # Yes/No prompts
├── tests/                # Integration tests
├── Cargo.toml
├── README.md
├── CONTRIBUTING.md
└── LICENSE
```

---

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_box_render

# Run tests in release mode
cargo test --release

# Check code coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Writing Tests

Add tests in `tests/` directory or as unit tests within modules:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_renders_correctly() {
        // Arrange
        let message = "Hello";
        let style = "success";

        // Act
        let result = render_box(message, style);

        // Assert
        assert!(result.contains("Hello"));
    }
}
```

For CLI integration tests, use `assert_cmd`:

```rust
use assert_cmd::Command;

#[test]
fn test_box_command() {
    let mut cmd = Command::cargo_bin("termgfx").unwrap();
    cmd.args(["box", "Hello", "--style", "success"])
        .assert()
        .success();
}
```

---

## 📝 Code Style

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Fix automatically
cargo clippy --fix
```

### Style Guidelines

| Rule | Example |
|------|---------|
| Use descriptive names | `render_styled_box()` not `rsb()` |
| Document public APIs | Add `///` doc comments |
| Handle errors properly | Use `Result` or `Option` |
| Keep functions small | < 50 lines ideally |
| Write tests | Every feature needs tests |

### Example Function

```rust
/// Renders a styled box with the given message.
///
/// # Arguments
/// * `message` - The text to display inside the box
/// * `style` - Color style: "info", "success", "warning", "danger"
///
/// # Examples
/// ```
/// render_styled_box("Hello!", "success");
/// ```
pub fn render_styled_box(message: &str, style: &str) -> Result<String> {
    // Implementation
}
```

---

## 🔄 Pull Request Process

### 1. Create a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clean, documented code
- Add tests for new functionality
- Update README if needed

### 3. Test Everything

```bash
cargo fmt
cargo clippy
cargo test
```

### 4. Commit & Push

```bash
git add .
git commit -m "feat(box): add rainbow gradient option"
git push origin feature/your-feature-name
```

### 5. Open Pull Request

- Go to GitHub and open a PR
- Fill in the PR template
- Link related issues
- Wait for review

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Commit messages follow convention

---

## 💬 Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting, no code change |
| `refactor` | Code change, no new feature or fix |
| `test` | Adding tests |
| `chore` | Build, tooling, deps |

### Examples

```bash
feat(charts): add stacked bar chart support
fix(spinner): prevent cursor flickering on Windows
docs(readme): add installation instructions
test(box): add tests for gradient rendering
refactor(output): extract common border logic
```

---

## 🐛 Reporting Bugs

### Before Reporting

1. Check existing [issues](https://github.com/ybouhjira/termgfx/issues)
2. Update to latest version
3. Try to reproduce consistently

### Bug Report Template

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
1. Run command `termgfx box "test" --style gradient`
2. See error

**Expected behavior**
What you expected to happen.

**Environment**
- OS: [e.g., macOS 14.0]
- Terminal: [e.g., iTerm2, Alacritty]
- TermGFX version: [e.g., 0.1.0]
- Rust version: [e.g., 1.75.0]

**Screenshots**
If applicable, add screenshots.
```

---

## 💡 Suggesting Features

### Feature Request Template

```markdown
**Is this related to a problem?**
A description of the problem. Ex: I'm frustrated when [...]

**Describe the solution**
What you want to happen.

**Alternatives considered**
Any alternative solutions you've considered.

**Additional context**
Any other context or screenshots.
```

---

## 🏷️ Labels

| Label | Description |
|-------|-------------|
| `bug` | Something isn't working |
| `enhancement` | New feature or request |
| `documentation` | Documentation improvements |
| `good first issue` | Good for newcomers |
| `help wanted` | Extra attention needed |

---

## ❓ Questions?

- Open a [Discussion](https://github.com/ybouhjira/termgfx/discussions)
- Check existing issues

---

Thanks for contributing! 🙌
