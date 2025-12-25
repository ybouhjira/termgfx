# termgfx UX Analysis

## Executive Summary

The CLI has **28 commands** with inconsistent patterns that hurt discoverability and AI usability. Key issues: redundant commands, inconsistent data input, confusing defaults, and poor grouping.

---

## Issues by Category

### 1. REDUNDANT COMMANDS

| Issue | Commands | Problem |
|-------|----------|---------|
| `typewriter` vs `animate -t typewriter` | Both exist | Duplicate functionality |
| `progress` vs `animate -t progress` | Both exist | Duplicate functionality |
| `select` vs `choose` | Very similar | Confusing which to use |

**Recommendation:** Remove `animate` command entirely. Each effect should be its own command with `--animate` flag.

---

### 2. INCONSISTENT DATA INPUT PATTERNS

| Command | Data Format | Problem |
|---------|-------------|---------|
| `sparkline` | `"1,4,2,8"` (positional) | Good |
| `chart bar` | `--data "A:10,B:20"` (flag) | Why not positional? |
| `table` | `--headers` + `--rows` | Two flags required |
| `tree` | `"root>src,docs"` (positional) | Different separator |
| `heatmap` | `--data "1,2;3,4"` (flag) | Semicolon separator |
| `timeline` | `--events "A,B,C"` (flag) | Required flag |
| `gauge` | `75` (positional) | Good |
| `dashboard` | `--panels "box:Hi,progress:75"` | Yet another format |

**Recommendation:** Standardize:
- Simple values: positional argument
- Key-value pairs: `key:value,key:value`
- 2D data: rows separated by `|` or newlines
- Always allow stdin as alternative

---

### 3. CONFUSING DEFAULTS

| Command | Default | Problem |
|---------|---------|---------|
| `animate` | `--effect-type progress` | Running `termgfx animate` shows progress bar - confusing |
| `confirm --default yes` | Default is "yes" | Should be no default, force explicit choice |
| `box --style info` | "info" style | Should be "default" or no styling |

**Recommendation:**
- `animate` should REQUIRE `--effect-type` (no default)
- `confirm` should have no default answer
- Use "default" as style name, not "info"

---

### 4. INCONSISTENT FLAG NAMES

| Concept | Variations | Should Be |
|---------|------------|-----------|
| Animation duration | `--animation-time`, `--duration`, `--speed` | `--duration` everywhere |
| Visual style | `--style`, `--colors`, `--gradient` | `--style` or `--theme` |
| Data input | `--data`, `--events`, `--panels` | `--data` or positional |

**Recommendation:** Standardize on:
- `--duration` for all timing
- `--style` for visual appearance
- `--data` or positional for input data

---

### 5. POOR COMMAND GROUPING

Current flat list of 28 commands is overwhelming.

**Recommendation:** Use subcommands:

```
termgfx output box "Hello"
termgfx output banner "Title"
termgfx output notification "Alert"

termgfx chart bar --data "A:10"
termgfx chart line --data "1,2,3"
termgfx chart sparkline "1,2,3"
termgfx chart gauge 75
termgfx chart heatmap --data "1,2;3,4"

termgfx data table --headers "A,B"
termgfx data tree "root>child"
termgfx data diff file1 file2
termgfx data timeline --events "A,B,C"

termgfx input text "Question?"
termgfx input select "Pick one" opt1 opt2
termgfx input confirm "Sure?"

termgfx animate spinner "Loading..."
termgfx animate progress 75
termgfx animate typewriter "Hello"
```

---

### 6. AI USABILITY ISSUES

| Issue | Example | Fix |
|-------|---------|-----|
| Magic separators | `>`, `;`, `\|`, `:` | Document clearly or use JSON |
| Required flags | `--events` on timeline | Make positional |
| Hidden options | Style values not in help | List in `--help` |
| No JSON input | Must use custom formats | Add `--json` flag |

**Recommendation for AI:**
```bash
# Current (hard to generate)
termgfx dashboard --panels "box:Hello,progress:75,sparkline:1,2,3"

# Better (AI-friendly)
termgfx dashboard --json '{"panels":[{"type":"box","text":"Hello"}]}'

# Or even better (pipe-friendly)
echo '{"panels":[...]}' | termgfx dashboard
```

---

### 7. MISSING FEATURES FOR DISCOVERABILITY

| Missing | Why It Matters |
|---------|----------------|
| `termgfx list` | Show all commands grouped |
| `termgfx examples` | Interactive examples |
| `--dry-run` | Preview without rendering |
| Style previews | `termgfx styles box` to see all box styles |

---

## Proposed Command Structure

```
termgfx
├── box <message>                    # Styled message box
├── banner <title>                   # Large banner
├── notify <message>                 # Notification
│
├── chart
│   ├── bar <data>                   # Bar chart
│   ├── line <data>                  # Line chart
│   ├── pie <data>                   # Pie chart
│   ├── spark <data>                 # Sparkline
│   ├── gauge <value>                # Gauge/dial
│   └── heat <data>                  # Heatmap
│
├── data
│   ├── table                        # Table from CSV/JSON
│   ├── tree                         # Tree structure
│   ├── diff <file1> <file2>         # File diff
│   └── timeline <events>            # Timeline
│
├── prompt
│   ├── input <question>             # Text input
│   ├── select <question> <opts...>  # Single select
│   ├── multi <question> <opts...>   # Multi select
│   └── confirm <question>           # Yes/no
│
├── animate
│   ├── spinner <message>            # Loading spinner
│   ├── progress <percent>           # Progress bar
│   └── typewriter <text>            # Typewriter effect
│
├── image <path>                     # Display image
├── record                           # Terminal recording
├── dashboard                        # Multi-panel TUI
├── demo                             # Interactive demo
│
└── help
    ├── examples                     # Show examples
    └── styles <command>             # Preview styles
```

---

## Priority Fixes

### P0 - Breaking/Confusing
1. Remove `animate` command defaults - require `--effect-type`
2. Remove duplicate `typewriter` (keep as `animate typewriter`)
3. Standardize `--duration` flag name

### P1 - Consistency
4. Make `timeline --events` positional
5. Make `chart bar --data` positional
6. Rename `choose` to `multiselect` or merge with `select --multi`

### P2 - Discoverability
7. Add `termgfx examples` command
8. Add `termgfx styles <command>` preview
9. Group commands with subcommands

### P3 - AI-Friendly
10. Add `--json` input support for complex commands
11. Support stdin for all data commands
12. Add `--format json` output option

---

## Migration Path

To avoid breaking changes:
1. Add new grouped commands as aliases first
2. Deprecate old patterns with warnings
3. Remove deprecated in next major version

```rust
// Example deprecation
#[deprecated(note = "Use `termgfx animate typewriter` instead")]
Commands::Typewriter { ... }
```
