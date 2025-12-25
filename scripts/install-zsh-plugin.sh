#!/bin/bash
# Install/Update termgfx oh-my-zsh plugin

PLUGIN_DIR="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/plugins/termgfx"
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "🔧 Installing termgfx oh-my-zsh plugin..."

# Create plugin directory
mkdir -p "$PLUGIN_DIR"

# Copy plugin file from repo
if [ -f "$REPO_DIR/zsh-plugin/termgfx.plugin.zsh" ]; then
    cp "$REPO_DIR/zsh-plugin/termgfx.plugin.zsh" "$PLUGIN_DIR/"
    echo "✅ Plugin installed to $PLUGIN_DIR"
else
    echo "❌ Plugin file not found in repo"
    exit 1
fi

# Check if plugin is in .zshrc
if grep -q "termgfx" ~/.zshrc; then
    echo "✅ Plugin already in .zshrc plugins list"
else
    echo "⚠️  Add 'termgfx' to your plugins=(...) in ~/.zshrc"
fi

echo ""
echo "🎉 Done! Restart your shell or run: source ~/.zshrc"
