# VHS tape for box command demo
Output docs/gifs/box.gif

Set FontSize 16
Set Width 800
Set Height 400
Set Theme "Catppuccin Mocha"

Type "./target/release/termgfx box 'Hello World!' --style success"
Enter
Sleep 1s

Type "./target/release/termgfx box 'Warning: Low disk space' --style warning"
Enter
Sleep 1s

Type "./target/release/termgfx box 'Error: Connection failed' --style danger"
Enter
Sleep 1s
