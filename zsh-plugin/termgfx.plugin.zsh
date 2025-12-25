# termgfx oh-my-zsh plugin
# Beautiful terminal graphics for CLI applications
# https://github.com/ybouhjira/termgfx

# ============= Completions =============
_termgfx() {
    local -a commands
    commands=(
        'box:Display a styled box with message'
        'banner:Display a styled banner'
        'notification:Show desktop + terminal notification'
        'chart:Display charts (bar, line, pie)'
        'sparkline:Display inline sparkline'
        'gauge:Display radial/dial gauge'
        'heatmap:Display 2D heatmap'
        'table:Display formatted tables'
        'tree:Display tree structures'
        'diff:Show file differences'
        'timeline:Display event timeline'
        'input:Interactive text input'
        'select:Single selection menu'
        'choose:Multiple selection menu'
        'confirm:Yes/No confirmation'
        'spinner:Animated spinner'
        'progress:Progress bar display'
        'typewriter:Typewriter text effect'
        'animate:ASCII art animations'
        'image:Display images in terminal'
        'record:Record terminal sessions'
        'script:Execute script files'
        'dashboard:Terminal dashboard layouts'
        'demo:Run feature demonstrations'
        'help:Show help information'
    )

    local -a box_styles
    box_styles=('default' 'success' 'error' 'warning' 'info')

    local -a border_styles
    border_styles=('single' 'double' 'rounded' 'none')

    local -a chart_types
    chart_types=('bar' 'line' 'pie')

    local -a gradients
    gradients=('blue-purple' 'red-orange' 'green-cyan' 'pink-yellow' 'cyan-purple')

    _arguments -C \
        '1: :->command' \
        '*:: :->args' \
        '--help[Show help]' \
        '-h[Show help]' \
        '--version[Show version]' \
        '-V[Show version]'

    case $state in
        command)
            _describe 'command' commands
            ;;
        args)
            case $words[1] in
                box)
                    _arguments \
                        '1:message:' \
                        '--style[Box style]:style:(default success error warning info)' \
                        '-s[Box style]:style:(default success error warning info)' \
                        '--border[Border style]:border:(single double rounded none)' \
                        '-b[Border style]:border:(single double rounded none)' \
                        '--emoji[Add emoji]' \
                        '-e[Add emoji]' \
                        '--animate[Enable animation]' \
                        '-a[Enable animation]' \
                        '--animation-time[Animation duration in ms]:time:'
                    ;;
                banner)
                    _arguments \
                        '1:title:' \
                        '--gradient[Gradient colors]:gradient:(blue-purple red-orange green-cyan pink-yellow cyan-purple)' \
                        '-g[Gradient colors]:gradient:(blue-purple red-orange green-cyan pink-yellow cyan-purple)' \
                        '--animate[Enable animation]' \
                        '-a[Enable animation]' \
                        '--animation-time[Animation duration in ms]:time:'
                    ;;
                chart)
                    _arguments \
                        '1:type:(bar line pie)' \
                        '--data[Chart data]:data:' \
                        '-d[Chart data]:data:' \
                        '--title[Chart title]:title:'
                    ;;
                sparkline)
                    _arguments \
                        '1:data:' \
                        '--animate[Enable animation]' \
                        '-a[Enable animation]' \
                        '--animation-time[Animation duration in ms]:time:'
                    ;;
                gauge)
                    _arguments \
                        '1:value:' \
                        '--max[Maximum value]:max:' \
                        '--label[Gauge label]:label:' \
                        '--style[Gauge style]:style:(default arc dial)'
                    ;;
                table)
                    _arguments \
                        '--headers[Column headers]:headers:' \
                        '-H[Column headers]:headers:' \
                        '--rows[Table rows]:rows:' \
                        '-r[Table rows]:rows:' \
                        '--file[Input file]:file:_files' \
                        '-f[Input file]:file:_files' \
                        '--border[Border style]:border:(single double rounded none)' \
                        '--alignment[Text alignment]:align:(left center right)' \
                        '--animate[Enable animation]' \
                        '-a[Enable animation]' \
                        '--animation-time[Animation duration in ms]:time:'
                    ;;
                tree)
                    _arguments \
                        '--data[Tree data]:data:' \
                        '-d[Tree data]:data:' \
                        '--path[Directory path]:path:_files -/' \
                        '-p[Directory path]:path:_files -/' \
                        '--animate[Enable animation]' \
                        '-a[Enable animation]' \
                        '--animation-time[Animation duration in ms]:time:'
                    ;;
                progress)
                    _arguments \
                        '1:percentage:' \
                        '--style[Progress style]:style:(default gradient blocks)' \
                        '-s[Progress style]:style:(default gradient blocks)' \
                        '--animate[Enable animation]' \
                        '-a[Enable animation]'
                    ;;
                spinner)
                    _arguments \
                        '--style[Spinner style]:style:(dots line braille circle)' \
                        '-s[Spinner style]:style:(dots line braille circle)' \
                        '--message[Status message]:message:'
                    ;;
                image)
                    _arguments \
                        '1:source:_files' \
                        '--width[Image width]:width:' \
                        '-w[Image width]:width:' \
                        '--height[Image height]:height:' \
                        '--protocol[Render protocol]:protocol:(halfblock kitty sixel)'
                    ;;
                notification)
                    _arguments \
                        '1:message:' \
                        '--title[Notification title]:title:' \
                        '--style[Notification style]:style:(info success warning error)'
                    ;;
                *)
                    ;;
            esac
            ;;
    esac
}

compdef _termgfx termgfx

# ============= Aliases =============
alias tgfx='termgfx'
alias tbox='termgfx box'
alias tbanner='termgfx banner'
alias tchart='termgfx chart'
alias tspark='termgfx sparkline'
alias ttable='termgfx table'
alias ttree='termgfx tree'
alias tprogress='termgfx progress'
alias tspinner='termgfx spinner'
alias timage='termgfx image'
alias tnotify='termgfx notification'
alias tgauge='termgfx gauge'
alias theat='termgfx heatmap'
alias tdemo='termgfx demo'

# ============= Helper Functions =============

# Quick success box
tgfx-success() {
    termgfx box "$1" --style success --animate
}

# Quick error box
tgfx-error() {
    termgfx box "$1" --style error --animate
}

# Quick warning box
tgfx-warn() {
    termgfx box "$1" --style warning --animate
}

# Quick info box
tgfx-info() {
    termgfx box "$1" --style info --animate
}

# Pretty JSON table from stdin
tgfx-json-table() {
    termgfx table < /dev/stdin
}

# Directory tree
tgfx-ls() {
    local dir="${1:-.}"
    termgfx tree --path "$dir"
}

# Quick banner with gradient
tgfx-title() {
    termgfx banner "$1" --gradient cyan-purple --animate
}

# Show progress animation (for scripts)
tgfx-loading() {
    local msg="${1:-Loading...}"
    termgfx spinner --message "$msg"
}

# ============= Auto-Update =============

# Update plugin from repo
termgfx-update() {
    local REPO_DIR="$HOME/Projects/termgfx"
    local PLUGIN_DIR="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/plugins/termgfx"

    if [ ! -d "$REPO_DIR" ]; then
        echo "❌ termgfx repo not found at $REPO_DIR"
        echo "   Clone it: git clone https://github.com/ybouhjira/termgfx ~/Projects/termgfx"
        return 1
    fi

    echo "🔄 Updating termgfx plugin..."
    cd "$REPO_DIR" && git pull origin master

    if [ -f "$REPO_DIR/zsh-plugin/termgfx.plugin.zsh" ]; then
        cp "$REPO_DIR/zsh-plugin/termgfx.plugin.zsh" "$PLUGIN_DIR/"
        echo "✅ Plugin updated!"
        echo "🔃 Reload shell: source ~/.zshrc"
    else
        echo "❌ Plugin file not found in repo"
        return 1
    fi
}

# Rebuild termgfx from source
termgfx-rebuild() {
    local REPO_DIR="$HOME/Projects/termgfx"

    if [ ! -d "$REPO_DIR" ]; then
        echo "❌ termgfx repo not found at $REPO_DIR"
        return 1
    fi

    echo "🔨 Building termgfx..."
    cd "$REPO_DIR" && cargo build --release

    if [ $? -eq 0 ]; then
        echo "✅ Build successful!"
        echo "📍 Binary at: $REPO_DIR/target/release/termgfx"
    else
        echo "❌ Build failed"
        return 1
    fi
}
