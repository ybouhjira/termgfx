# VHS tape for charts demo
Output docs/gifs/charts.gif

Set FontSize 14
Set Width 900
Set Height 600
Set Theme "Catppuccin Mocha"

Type "./target/release/termgfx chart bar --data 'Python:85,Go:70,Rust:92,Java:65'"
Enter
Sleep 2s

Type "./target/release/termgfx chart line --data '10,25,18,35,28,42,38,55' --title 'Growth' --animate"
Enter
Sleep 2s

Type "./target/release/termgfx chart pie --data 'Desktop:45,Mobile:35,Tablet:20' --animate"
Enter
Sleep 2s
