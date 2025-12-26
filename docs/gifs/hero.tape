# TermGFX Hero Demo - All Features Showcase
Output hero.gif

Set Shell "zsh"
Set FontSize 16
Set Width 900
Set Height 550
Set Theme "Catppuccin Mocha"
Set Padding 15
Set TypingSpeed 30ms

Sleep 300ms

# Banner
Type "termgfx banner 'TERMGFX' --gradient cyan-purple"
Enter
Sleep 2s

# Styled Boxes
Type "termgfx box 'Success!' --style success"
Enter
Sleep 1.2s

Type "termgfx box 'Warning!' --style warning"
Enter
Sleep 1.2s

# Bar Chart
Type "termgfx chart bar --data 'Rust:95,Go:80,Python:70,JS:85'"
Enter
Sleep 1.5s

# Progress bar
Type "termgfx progress 75 --style gradient"
Enter
Sleep 1.2s

# Sparkline
Type "termgfx sparkline '1,4,2,8,5,7,3,9,6'"
Enter
Sleep 1.2s

# Gauge
Type "termgfx gauge 85 --label 'CPU'"
Enter
Sleep 1.5s

# Table
Type "termgfx table --headers 'Name,Lang,Stars' --rows 'termgfx,Rust,⭐⭐⭐'"
Enter
Sleep 1.5s

# Final banner
Type "termgfx box '⚡ Fast • 🎨 Beautiful • 🦀 Rust' --style info"
Enter
Sleep 2s
