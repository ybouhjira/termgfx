# VHS tape for extra features demo
Output docs/gifs/extras.gif

Set FontSize 14
Set Width 900
Set Height 600
Set Theme "Catppuccin Mocha"

Type "./target/release/termgfx sparkline '1,4,2,8,5,7,3,9,6,5'"
Enter
Sleep 1.5s

Type "./target/release/termgfx gauge 75 --label 'CPU Usage'"
Enter
Sleep 1.5s

Type "./target/release/termgfx table --headers 'Name,Age,City' --rows 'Alice,30,NYC|Bob,25,LA|Carol,35,SF'"
Enter
Sleep 1.5s

Type "./target/release/termgfx banner 'TERMGFX' --gradient cyan-purple"
Enter
Sleep 2s
