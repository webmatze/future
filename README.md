# Future Terminal

A futuristic "Hollywood hacker" terminal dashboard built in Rust. Blade Runner meets Star Trek meets Hackers.

![Future Terminal Screenshot](screenshot.png)

## Features

- **Matrix Rain** - Classic falling green characters
- **World Map** - Global network with animated connections between cities
- **System Monitors** - Real CPU, memory, and network stats with sparklines
- **Fake Logs** - Scrolling hacker-style log messages
- **Source Code** - Syntax-highlighted code streams
- **Countdown Timer** - Large ASCII digits with dramatic effects
- **Hex Dump** - Scrolling data stream
- **Progress Bars** - Animated operations (DECRYPTING, UPLOADING, etc.)

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
./target/release/future
```

## Controls

| Key | Action |
|-----|--------|
| `q` / `ESC` | Quit |
| `Space` | Pause/Resume |
| `+` / `-` | Speed up/down |
| `r` | Reset countdown |
| `?` / `h` | Help |

## Requirements

- Terminal with true color support (most modern terminals)
- Minimum 80x24 terminal size

## License

MIT
