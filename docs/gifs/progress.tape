# VHS tape for progress and spinner demo
Output docs/gifs/progress.gif

Set FontSize 16
Set Width 800
Set Height 400
Set Theme "Catppuccin Mocha"

Type "./target/release/termgfx progress 75 --style gradient"
Enter
Sleep 1s

Type "./target/release/termgfx progress 50 --style blocks"
Enter
Sleep 1s

Type "./target/release/termgfx spinner 'Loading...' --style dots --duration 2"
Enter
Sleep 3s
