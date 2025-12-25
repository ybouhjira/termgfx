# TermGFX Competitive Analysis

> Comprehensive comparison of termgfx against leading terminal graphics tools

## Executive Summary

TermGFX stands out as the **most feature-complete CLI-first terminal graphics tool** available today. While competitors excel in specific areas, termgfx uniquely combines:

- **Data Visualization**: Charts, sparklines, heatmaps, gauges, timelines (Gum has NONE)
- **Rich Output**: Boxes, banners, tables, trees, notifications, dashboards
- **Interactive Prompts**: Input, select, multi-select, confirm
- **Animations**: Progress bars, spinners, typewriter effects
- **Media**: Terminal images (Kitty, Sixel, halfblock protocols)
- **Single Binary**: 4.5MB Rust binary vs 13MB Go binary (Gum)

---

## Competitor Overview

| Tool | Language | Primary Use | Installation |
|------|----------|-------------|--------------|
| **Gum** | Go | Interactive shell scripts | `brew install gum` |
| **Rich** | Python | Library for Python apps | `pip install rich` |
| **Plotext** | Python | Terminal plotting library | `pip install plotext` |
| **Figlet** | C | ASCII art banners | `brew install figlet` |
| **Boxes** | C | Text boxes | `brew install boxes` |
| **Timg** | C++ | Terminal images | `brew install timg` |

---

## Feature Comparison Matrix

### Legend
- **++** = Best in class
- **+** = Good support
- **-** = Basic/limited
- **X** = Not available

| Feature | termgfx | Gum | Rich | Plotext | Figlet | Boxes |
|---------|---------|-----|------|---------|--------|-------|
| **Styled Boxes** | ++ | + | ++ | X | X | + |
| **ASCII Banners** | ++ | X | X | X | ++ | X |
| **Gradient Colors** | ++ | + | + | - | X | X |
| **Bar Charts** | ++ | X | - | ++ | X | X |
| **Line Charts** | ++ | X | X | ++ | X | X |
| **Pie Charts** | + | X | X | + | X | X |
| **Sparklines** | ++ | X | + | + | X | X |
| **Heatmaps** | ++ | X | X | + | X | X |
| **Gauges** | ++ | X | X | + | X | X |
| **Timelines** | ++ | X | X | X | X | X |
| **Tables** | ++ | + | ++ | X | X | X |
| **Trees** | + | X | ++ | X | X | X |
| **Progress Bars** | ++ | + | ++ | X | X | X |
| **Spinners** | ++ | + | ++ | X | X | X |
| **Notifications** | ++ | X | X | X | X | X |
| **Dashboards** | ++ | X | - | X | X | X |
| **Diff Viewer** | + | X | - | X | X | X |
| **Text Input** | + | + | X | X | X | X |
| **Select Prompt** | + | ++ | X | X | X | X |
| **Multi-Select** | + | ++ | X | X | X | X |
| **Confirm Dialog** | + | + | X | X | X | X |
| **File Picker** | X | + | X | X | X | X |
| **Terminal Images** | ++ | X | - | X | X | + |
| **Animations** | ++ | + | + | X | X | X |
| **Typewriter** | + | X | X | X | X | X |
| **Recording** | + | X | X | X | X | X |
| **Scripting** | + | X | X | X | X | X |
| **CLI Usage** | ++ | ++ | - | - | + | + |
| **Library API** | + | X | ++ | ++ | X | X |

---

## Detailed Comparison

### 1. Styled Boxes / Panels

#### termgfx
```bash
termgfx box "Build completed!" --style success --border rounded --emoji "OK"
```
Output:
```
+---------------------------+
| OK Build completed!       |
+---------------------------+
```
- **Styles**: info, success, warning, danger, gradient
- **Borders**: single, double, rounded, thick, ascii
- **Animations**: Box drawing animation support

#### Gum
```bash
gum style --border rounded --padding "1 2" "Hello!"
```
- Limited to border styling
- No semantic styles (success, danger)
- No emoji integration
- No gradient support

#### Rich (Python)
```python
from rich.panel import Panel
Console().print(Panel("Hello!", title="Info"))
```
- Excellent styling options
- Requires Python runtime
- Not CLI-friendly for shell scripts

**Winner: termgfx** - Best combination of CLI simplicity + rich features

---

### 2. Charts & Data Visualization

#### termgfx
```bash
# Bar chart
termgfx chart bar --data "Sales:100,Costs:60,Profit:40"

# Line chart
termgfx chart line --data "1,3,2,5,4,6" --title "Trend"

# Pie chart
termgfx chart pie --data "Desktop:45,Mobile:35,Tablet:20"

# Sparkline
termgfx sparkline "1,4,2,8,5,7"

# Gauge
termgfx gauge 75 --label "CPU" --style semicircle

# Heatmap
termgfx heatmap --data "1,2,3;4,5,6;7,8,9" --colors viridis

# Timeline
termgfx timeline --events "Start,Middle,End" --style arrow
```

#### Gum
**NO CHART SUPPORT AT ALL** - This is a major gap

#### Plotext (Python)
```python
import plotext as plt
plt.bar(["A","B","C"], [10,20,30])
plt.show()
```
- Excellent chart quality
- Requires Python
- Not CLI-friendly

**Winner: termgfx** - Only CLI tool with comprehensive charts

---

### 3. Interactive Prompts

#### termgfx
```bash
termgfx select "Framework:" "React" "Vue" "Svelte"
termgfx input "Name:" --placeholder "John"
termgfx confirm "Deploy?" --style danger
termgfx choose "Features:" "Auth" "API" --multi
```

#### Gum
```bash
gum choose "React" "Vue" "Svelte"
gum input --placeholder "Name"
gum confirm "Deploy?"
gum file  # File picker (unique to Gum)
gum filter < list.txt  # Fuzzy filter (unique to Gum)
```

**Winner: Gum** - More polished prompts + file picker + fuzzy filter

#### Improvement Areas for termgfx:
- Add `termgfx file` command for file picking
- Add `termgfx filter` for fuzzy filtering
- Better visual styling on prompts

---

### 4. ASCII Art Banners

#### termgfx
```bash
termgfx banner "HELLO" --gradient cyan-purple
```
- Gradient color support
- Animation support
- Clean, modern block font

#### Figlet
```bash
figlet "HELLO"
```
- Many font options
- No color support (requires piping to lolcat)
- No animation

**Winner: termgfx** - Built-in gradients + animations

---

### 5. Progress & Spinners

#### termgfx
```bash
termgfx progress 75 --style gradient --animate
termgfx spinner "Loading..." --style dots --duration 5
```
- Multiple styles: gradient, blocks, classic, thin, modern
- Custom colors with --from and --to
- Animation support

#### Gum
```bash
gum spin --spinner dot --title "Loading" -- sleep 5
```
- Good spinner variety
- Progress bar available but basic

#### Rich
```python
from rich.progress import track
for i in track(range(100)):
    process()
```
- Excellent, but requires Python

**Winner: termgfx** - Best CLI progress bars with animations

---

### 6. Tables

#### termgfx
```bash
termgfx table --headers "Name,Age,City" --rows "Alice,30,NYC|Bob,25,LA"
```
- File input support (--file)
- Border styles
- Alignment options
- Animation support

#### Gum
```bash
echo -e "Name,Age\nAlice,30" | gum table
```
- Clean styling
- Requires TTY (fails in pipes)

#### Rich
```python
from rich.table import Table
table = Table()
# ... best table styling
```
- Best visual quality
- Requires Python

**Winner: Rich** for quality, **termgfx** for CLI usability

---

### 7. Terminal Images

#### termgfx
```bash
termgfx image ./photo.png --protocol auto
```
- Kitty protocol
- Sixel protocol
- Halfblock fallback
- Auto-detection

#### Timg
```bash
timg photo.png
```
- Dedicated image tool
- More format support
- Animations/GIFs

**Winner: Timg** for images only, **termgfx** for all-in-one

---

### 8. Unique termgfx Features

Features that NO competitor has:

| Feature | termgfx | Why It Matters |
|---------|---------|----------------|
| **Dashboards** | `dashboard --layout 2x2 --panels "..."` | Multi-panel TUI from CLI |
| **Heatmaps** | `heatmap --data "1,2;3,4"` | 2D data visualization |
| **Timelines** | `timeline --events "A,B,C"` | Project/event visualization |
| **Gauges** | `gauge 75 --style semicircle` | Radial indicators |
| **Notifications** | `notification "Done!" --sound` | Desktop + terminal alerts |
| **Recording** | `record start/play/export` | Terminal session recording |
| **Scripting** | `script --file demo.txt` | Animation sequences |
| **Diff Viewer** | `diff file1 file2` | Side-by-side comparison |

---

## Performance Comparison

### Binary Size
| Tool | Size | Notes |
|------|------|-------|
| **termgfx** | 4.5 MB | Single Rust binary, optimized |
| **Gum** | 13 MB | Single Go binary |
| **Figlet** | 100 KB | C binary, minimal |
| **Boxes** | 150 KB | C binary, minimal |
| **Rich** | 243 KB + Python | Requires Python runtime |
| **Plotext** | 137 KB + Python | Requires Python runtime |

### Startup Time (estimated)
| Tool | Cold Start | Warm Start |
|------|-----------|------------|
| termgfx | ~10ms | ~5ms |
| Gum | ~15ms | ~8ms |
| Python tools | ~100-300ms | ~50ms |

**Winner: termgfx** - Smallest single-binary solution with full features

---

## Ecosystem & Community

| Tool | GitHub Stars | Maintainer | Activity |
|------|-------------|------------|----------|
| Gum | 17k+ | Charmbracelet | Very Active |
| Rich | 47k+ | Will McGugan | Very Active |
| Plotext | 3k+ | Savino Piccolomo | Active |
| Figlet | Classic | Various | Stable |
| termgfx | New | ybouhjira | Active |

---

## Use Case Recommendations

### When to use termgfx:
- Shell scripts needing charts and data visualization
- All-in-one terminal graphics (boxes + charts + prompts)
- Minimal dependencies (single binary)
- Animations and visual effects
- Dashboard displays

### When to use Gum:
- Interactive prompts are primary need
- File picking required
- Fuzzy filtering needed
- Already using other Charm tools

### When to use Rich:
- Python applications
- Complex table formatting
- Rich text logging
- Library integration preferred

### When to use Plotext:
- Data science / analysis
- Complex charts needed
- Python environment available

---

## Gap Analysis: What termgfx Should Add

### High Priority (Match Gum):
1. **File Picker** - `termgfx file` or `termgfx pick`
2. **Fuzzy Filter** - `termgfx filter < list.txt`
3. **Pager** - `termgfx pager < file.txt`
4. **Better prompt styling** - Match Gum's polish

### Medium Priority (Differentiation):
1. **More chart types** - Scatter plots, histograms
2. **Calendar view** - Monthly/yearly calendars
3. **Kanban boards** - Task board visualization
4. **Git-style graphs** - Commit history visualization

### Low Priority (Nice to have):
1. **Theme system** - Custom color schemes
2. **Config file** - `.termgfxrc` for defaults
3. **Plugin system** - Extensibility
4. **WASM web demo** - Interactive docs

---

## Conclusion

**termgfx is the most feature-rich CLI terminal graphics tool available.**

### Strengths:
- Only CLI tool with charts, gauges, heatmaps, timelines
- Smallest binary (4.5MB vs 13MB Gum)
- Built-in animations throughout
- Dashboards from command line
- Desktop notifications
- Session recording

### Weaknesses:
- Interactive prompts less polished than Gum
- Missing file picker and fuzzy filter
- Newer/smaller community
- No plugin system

### Verdict:
For **data visualization in shell scripts**, termgfx has NO competition. For **interactive prompts**, Gum is slightly better. For **Python apps**, Rich is better.

termgfx fills a unique niche: **comprehensive terminal graphics from the command line**.

---

*Analysis Date: December 2024*
*termgfx version: 0.2.1*
*Gum version: 0.17.0*
