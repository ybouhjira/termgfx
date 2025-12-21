# Input Prompt Demo

The `termgfx input` command provides interactive text input with styling.

## Features

### 1. Basic Input
```bash
./target/release/termgfx input "Enter your name:"
```
- Cyan colored prompt
- Live character-by-character input
- Enter to submit
- Returns value to stdout

### 2. Placeholder Text
```bash
./target/release/termgfx input "Enter your name:" -P "John Doe"
```
- Grey placeholder text shown when empty
- Disappears on first keystroke
- Reappears if you delete all characters with backspace

### 3. Password Mode
```bash
./target/release/termgfx input "Password:" --password
```
- Shows `*` instead of actual characters
- Input is still captured correctly
- Returns actual password to stdout (not asterisks)

### 4. Cancellation
- Press `Ctrl+C` to cancel input
- Exits with error code 1
- Displays "Error: Cancelled by user"

## Implementation Details

- Uses `crossterm` for raw terminal mode
- Keyboard event handling with `KeyCode` matching
- Terminal styling with `SetForegroundColor`
- Cursor movement for placeholder display
- Graceful cleanup on exit (disables raw mode)

## Testing
The implementation has been tested with:
- ✅ Basic text input
- ✅ Placeholder functionality
- ✅ Password masking
- ✅ Backspace/delete handling
- ✅ Ctrl+C cancellation
