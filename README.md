# jig

A simple mouse jiggler to prevent idle/sleep.

## Installation

```bash
cargo install --path .
```

Or download from [releases](https://github.com/yourusername/jig/releases).

## Usage

```bash
jig                    # default 60s interval
jig -i 300             # 5 minute interval
JIG_INTERVAL=120 jig   # env var
```

## Platform Notes

**Windows:** Works out of the box.

**Linux (X11):** Requires `libxdo-dev`:

```bash
sudo apt install libxdo-dev      # Ubuntu/Debian
sudo pacman -S xdotool          # Arch
```

**Linux (Wayland):** Not supported. Use an X11 session or XWayland.

**macOS:** Should work but untested.

## Building

```bash
cargo build --release

# Cross-compile for Windows (from Linux)
cargo build --release --target x86_64-pc-windows-gnu
```

## License

MIT
