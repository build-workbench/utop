# utop

A lightweight htop clone built with Rust, ratatui, and sysinfo.

## Features

- Per-core CPU meters with load coloring (green / yellow / red)
- Process table sortable by CPU, memory, PID, or name (ascending/descending)
- Incremental search filter (matches process name or PID)
- Tree view with collapsible subtrees
- Kill processes with confirmation and signal choice (SIGTERM / SIGKILL)
- Process details panel (state, PPID, executable, command line)
- Pause/resume refresh, adjustable refresh interval
- Mouse wheel scrolling
- Command-line options for initial sort, filter, delay, and view

## Build & Run

```sh
cargo build --release
./target/release/utop
```

Or directly:

```sh
cargo run --release
```

## Usage

```
utop [OPTIONS]

Options:
  -h, --help           Print help and exit
  -s, --sort <KEY>     Initial sort key: cpu | mem | pid | name [default: cpu]
  -a, --asc            Start in ascending order [default: descending]
  -d, --delay <MS>     Refresh interval in milliseconds, clamped to 100..=5000
                       [default: 500]
  -f, --filter <STR>   Initial process filter (matches name or PID)
  -t, --tree           Start in tree view
```

## Keys

| Key | Action |
|-----|--------|
| q / Ctrl+C | Quit |
| Up/Down, PgUp/PgDn, Home/End, mouse wheel | Navigate |
| s | Cycle sort key (CPU/MEM/PID/NAME) |
| r | Toggle ascending/descending |
| / | Search processes (Enter to confirm, Esc to clear) |
| Esc | Clear filter |
| t | Toggle tree view |
| Space | Collapse/expand subtree (tree view) |
| p | Pause/resume refresh |
| F5 | Force refresh |
| k | Kill selected process (then y = SIGTERM, K = SIGKILL, Esc = cancel) |
| d / Enter | Toggle process details |
| - / + | Decrease / increase refresh interval (100 ms steps) |

Note: filtering temporarily flattens the tree view, since a partial tree is
harder to read than a plain list.

## Project layout

| Module | Responsibility |
|--------|----------------|
| `src/main.rs` | Entry point and event loop |
| `src/app.rs` | Application state and input modes |
| `src/proc.rs` | Process row model, sorting and filtering helpers |
| `src/collect.rs` | sysinfo snapshots and tree construction |
| `src/ui.rs` | ratatui rendering |
| `src/cli.rs` | Command-line argument parsing |

## License

MIT OR Apache-2.0
