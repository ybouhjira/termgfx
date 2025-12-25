#!/usr/bin/env python3
"""
Plotext Full Demo - All Features
Compare with: termgfx chart, sparkline, gauge, heatmap
"""

import plotext as plt
import math
import time
import random

def pause():
    input("\n[Press Enter to continue...]")
    print("\033[H\033[J", end="")  # Clear screen

def header(title):
    print("\n" + "=" * 60)
    print(f"  {title}")
    print("=" * 60 + "\n")

# ============================================================================
# INTRO
# ============================================================================
print("\033[H\033[J")  # Clear
print("""
╔═══════════════════════════════════════════════════════════════╗
║                    PLOTEXT FULL DEMO                          ║
║           Terminal Plotting Library for Python                ║
╚═══════════════════════════════════════════════════════════════╝
""")
print("Plotext version:", plt.__version__)
print("\nThis demo covers ALL major Plotext features.")
print("Compare each with termgfx equivalent.\n")
pause()

# ============================================================================
# 1. BAR CHARTS
# ============================================================================
header("1. BAR CHARTS")

# Simple bar chart
plt.clear_figure()
plt.bar(["Python", "JavaScript", "Rust", "Go", "Java"], [85, 78, 92, 65, 70])
plt.title("Programming Language Scores")
plt.xlabel("Language")
plt.ylabel("Score")
plt.show()

print("\n📊 termgfx equivalent:")
print('   termgfx chart bar --data "Python:85,JavaScript:78,Rust:92,Go:65,Java:70"')
pause()

# Horizontal bar chart
plt.clear_figure()
plt.bar(["Q1", "Q2", "Q3", "Q4"], [120, 150, 180, 200], orientation="h")
plt.title("Quarterly Revenue ($K)")
plt.show()

print("\n📊 termgfx: Horizontal bars not yet supported")
pause()

# Stacked bar chart
plt.clear_figure()
plt.stacked_bar(
    ["Jan", "Feb", "Mar", "Apr"],
    [[10, 20, 15, 25], [15, 10, 20, 15], [5, 15, 10, 20]],
    labels=["Product A", "Product B", "Product C"]
)
plt.title("Monthly Sales by Product")
plt.show()

print("\n📊 termgfx: Stacked bars not yet supported")
pause()

# ============================================================================
# 2. LINE CHARTS
# ============================================================================
header("2. LINE CHARTS")

# Simple line
plt.clear_figure()
x = list(range(1, 13))
y = [12, 15, 13, 18, 22, 25, 28, 26, 24, 20, 16, 14]
plt.plot(x, y, marker="braille")
plt.title("Monthly Temperature (°C)")
plt.xlabel("Month")
plt.ylabel("Temperature")
plt.show()

print("\n📊 termgfx equivalent:")
print('   termgfx chart line --data "12,15,13,18,22,25,28,26,24,20,16,14" --title "Temperature"')
pause()

# Multiple lines
plt.clear_figure()
x = list(range(1, 11))
plt.plot(x, [i**2 for i in x], label="Quadratic", marker="braille")
plt.plot(x, [i*10 for i in x], label="Linear", marker="braille")
plt.plot(x, [2**i for i in x], label="Exponential", marker="braille")
plt.title("Growth Comparison")
plt.legend()
plt.show()

print("\n📊 termgfx: Multiple lines not yet supported")
pause()

# ============================================================================
# 3. SCATTER PLOTS
# ============================================================================
header("3. SCATTER PLOTS")

plt.clear_figure()
x = [random.gauss(50, 10) for _ in range(100)]
y = [random.gauss(50, 15) for _ in range(100)]
plt.scatter(x, y, marker="dot")
plt.title("Random Distribution")
plt.xlabel("X Values")
plt.ylabel("Y Values")
plt.show()

print("\n📊 termgfx: Scatter plots not yet supported")
pause()

# ============================================================================
# 4. HISTOGRAMS
# ============================================================================
header("4. HISTOGRAMS")

plt.clear_figure()
data = [random.gauss(100, 15) for _ in range(1000)]
plt.hist(data, bins=20)
plt.title("Normal Distribution Histogram")
plt.xlabel("Value")
plt.ylabel("Frequency")
plt.show()

print("\n📊 termgfx: Histograms not yet supported")
pause()

# ============================================================================
# 5. CANDLESTICK CHARTS
# ============================================================================
header("5. CANDLESTICK / OHLC CHARTS")

plt.clear_figure()
dates = ["Mon", "Tue", "Wed", "Thu", "Fri"]
opens = [100, 105, 102, 108, 110]
highs = [108, 110, 109, 115, 118]
lows = [98, 102, 100, 106, 108]
closes = [105, 103, 108, 112, 115]
plt.candlestick(dates, {"Open": opens, "Close": closes, "High": highs, "Low": lows})
plt.title("Stock Price OHLC")
plt.show()

print("\n📊 termgfx: Candlestick charts not yet supported")
pause()

# ============================================================================
# 6. BOX PLOTS
# ============================================================================
header("6. BOX PLOTS")

plt.clear_figure()
data1 = [random.gauss(50, 10) for _ in range(100)]
data2 = [random.gauss(60, 15) for _ in range(100)]
data3 = [random.gauss(45, 8) for _ in range(100)]
plt.box([data1, data2, data3], labels=["Group A", "Group B", "Group C"])
plt.title("Distribution Comparison")
plt.show()

print("\n📊 termgfx: Box plots not yet supported")
pause()

# ============================================================================
# 7. HEATMAPS / MATRICES
# ============================================================================
header("7. HEATMAPS / MATRIX PLOTS")

plt.clear_figure()
matrix = [[i + j for j in range(10)] for i in range(10)]
plt.matrix_plot(matrix)
plt.title("Heatmap Matrix")
plt.show()

print("\n📊 termgfx equivalent:")
print('   termgfx heatmap --data "1,2,3;4,5,6;7,8,9" --colors viridis')
pause()

# ============================================================================
# 8. IMAGE PLOTS
# ============================================================================
header("8. IMAGE FROM DATA")

plt.clear_figure()
# Create a simple gradient pattern
size = 30
image = [[(i + j) % 256 for j in range(size)] for i in range(size)]
plt.image_plot(image)
plt.title("Generated Image Pattern")
plt.show()

print("\n📊 termgfx equivalent:")
print('   termgfx image ./image.png')
pause()

# ============================================================================
# 9. DATETIME PLOTS
# ============================================================================
header("9. DATETIME PLOTS")

plt.clear_figure()
from datetime import datetime, timedelta

dates = [datetime(2024, 1, 1) + timedelta(days=i*30) for i in range(12)]
values = [100 + i*10 + random.randint(-20, 20) for i in range(12)]
plt.date_form("Y-m-d")
plt.plot([d.strftime("%Y-%m-%d") for d in dates], values, marker="braille")
plt.title("2024 Monthly Progress")
plt.show()

print("\n📊 termgfx: Date-aware plots not yet supported")
pause()

# ============================================================================
# 10. SUBPLOTS
# ============================================================================
header("10. SUBPLOTS (Multiple Charts)")

plt.clear_figure()
plt.subplots(2, 2)

plt.subplot(1, 1)
plt.bar(["A", "B", "C"], [10, 20, 15])
plt.title("Bar")

plt.subplot(1, 2)
plt.plot([1, 2, 3, 4], [1, 4, 2, 3])
plt.title("Line")

plt.subplot(2, 1)
plt.scatter([1, 2, 3, 4, 5], [5, 2, 4, 1, 3])
plt.title("Scatter")

plt.subplot(2, 2)
plt.hist([random.gauss(0, 1) for _ in range(100)], bins=10)
plt.title("Histogram")

plt.show()

print("\n📊 termgfx equivalent:")
print('   termgfx dashboard --layout 2x2 --panels "chart:bar,chart:line,chart:scatter,chart:hist"')
pause()

# ============================================================================
# 11. ANIMATIONS
# ============================================================================
header("11. REAL-TIME ANIMATIONS")

print("Animating a growing line chart...")
time.sleep(1)

data = []
for i in range(30):
    plt.clear_figure()
    data.append(math.sin(i * 0.3) * 10 + random.random() * 2)
    plt.plot(data, marker="braille")
    plt.title(f"Live Data Stream (frame {i+1}/30)")
    plt.ylim(-15, 15)
    plt.show()
    plt.sleep(0.1)

print("\n📊 termgfx equivalent:")
print('   termgfx sparkline "..." --animate')
print('   termgfx progress 75 --animate')
pause()

# ============================================================================
# 12. THEMES AND COLORS
# ============================================================================
header("12. THEMES AND COLORS")

plt.clear_figure()
plt.theme("pro")  # dark theme
x = list(range(10))
y1 = [i**2 for i in x]
y2 = [i*8 for i in x]
plt.plot(x, y1, label="Quadratic", color="red")
plt.plot(x, y2, label="Linear", color="cyan")
plt.title("Pro Theme with Colors")
plt.legend()
plt.show()

print("\nAvailable themes: default, clear, pro, matrix, windows, girly, dark, retro")
pause()

# ============================================================================
# 13. SPECIAL MARKERS
# ============================================================================
header("13. MARKER STYLES")

plt.clear_figure()
x = list(range(1, 8))
plt.plot(x, [i for i in x], marker="dot", label="dot")
plt.plot(x, [i+2 for i in x], marker="hd", label="hd (high-def)")
plt.plot(x, [i+4 for i in x], marker="braille", label="braille")
plt.plot(x, [i+6 for i in x], marker="fhd", label="fhd (full-hd)")
plt.title("Different Marker Resolutions")
plt.legend()
plt.show()

print("\n📊 Higher resolution = smoother curves but needs Unicode support")
pause()

# ============================================================================
# 14. POLAR / RADAR CHARTS
# ============================================================================
header("14. POLAR COORDINATES")

plt.clear_figure()
angles = [i * 30 for i in range(12)]
radii = [1, 3, 2, 4, 3, 5, 4, 3, 2, 4, 3, 2]
plt.polar(angles, radii)
plt.title("Polar Plot")
plt.show()

print("\n📊 termgfx: Polar charts not yet supported")
pause()

# ============================================================================
# 15. TEXT AND ANNOTATIONS
# ============================================================================
header("15. TEXT AND ANNOTATIONS")

plt.clear_figure()
plt.plot([1, 2, 3, 4, 5], [2, 4, 3, 5, 4])
plt.text("Peak", 4, 5)
plt.text("Valley", 3, 3)
plt.title("Chart with Annotations")
plt.show()

print("\n📊 termgfx: Annotations not yet supported")
pause()

# ============================================================================
# 16. 3D SURFACE PLOTS (if supported)
# ============================================================================
header("16. ERROR BARS")

plt.clear_figure()
x = [1, 2, 3, 4, 5]
y = [10, 15, 12, 18, 14]
errors = [1, 2, 1.5, 2.5, 1]
plt.error(x, y, yerr=errors)
plt.title("Measurements with Error Bars")
plt.show()

print("\n📊 termgfx: Error bars not yet supported")
pause()

# ============================================================================
# 17. FILL BETWEEN
# ============================================================================
header("17. FILL BETWEEN (Area Charts)")

plt.clear_figure()
x = list(range(1, 11))
y1 = [i for i in x]
y2 = [i**1.5 for i in x]
plt.fill(x, y1, y2, fillx=True)
plt.title("Area Between Curves")
plt.show()

print("\n📊 termgfx: Area charts not yet supported")
pause()

# ============================================================================
# 18. STREAMING DATA
# ============================================================================
header("18. STREAMING / LIVE DATA")

print("Simulating live CPU monitoring...")
time.sleep(1)

cpu_data = []
for i in range(40):
    plt.clear_figure()
    cpu = 30 + math.sin(i * 0.5) * 20 + random.randint(-10, 10)
    cpu_data.append(max(0, min(100, cpu)))
    if len(cpu_data) > 20:
        cpu_data.pop(0)

    plt.plot(cpu_data, marker="braille", color="green" if cpu < 70 else "red")
    plt.title(f"CPU Usage: {cpu_data[-1]:.1f}%")
    plt.ylim(0, 100)
    plt.xlim(0, 20)
    plt.show()
    plt.sleep(0.15)

print("\n📊 termgfx equivalent:")
print('   termgfx gauge 75 --label "CPU" --animate')
pause()

# ============================================================================
# SUMMARY
# ============================================================================
header("PLOTEXT FEATURE SUMMARY")

print("""
┌─────────────────────────────────────────────────────────────────┐
│                    PLOTEXT CAPABILITIES                         │
├─────────────────────────────────────────────────────────────────┤
│  ✅ Bar Charts (vertical, horizontal, stacked)                  │
│  ✅ Line Charts (single, multiple, with legend)                 │
│  ✅ Scatter Plots                                                │
│  ✅ Histograms                                                   │
│  ✅ Candlestick / OHLC Charts                                   │
│  ✅ Box Plots                                                    │
│  ✅ Heatmaps / Matrix Plots                                     │
│  ✅ Image Plots                                                  │
│  ✅ Datetime-aware Plots                                        │
│  ✅ Subplots (grid layouts)                                     │
│  ✅ Real-time Animations                                        │
│  ✅ Themes and Colors                                           │
│  ✅ Multiple Marker Types                                       │
│  ✅ Polar / Radar Charts                                        │
│  ✅ Text Annotations                                            │
│  ✅ Error Bars                                                   │
│  ✅ Fill/Area Charts                                            │
│  ✅ Streaming Data                                              │
├─────────────────────────────────────────────────────────────────┤
│  ❌ CLI interface (library only)                                │
│  ❌ Single binary (requires Python)                             │
│  ❌ Shell script friendly                                       │
│  ❌ Styled boxes, banners                                       │
│  ❌ Interactive prompts                                         │
│  ❌ Notifications                                               │
│  ❌ Tables, Trees                                               │
│  ❌ Progress bars, spinners                                     │
└─────────────────────────────────────────────────────────────────┘

VERDICT:
  Plotext is EXCELLENT for Python data science applications.
  termgfx is BETTER for shell scripts and CLI usage.

  Consider adding to termgfx:
    - Scatter plots
    - Histograms
    - Stacked bar charts
    - Multiple line series
    - Polar/radar charts
    - Error bars
""")

print("\n🎉 Demo complete!")
