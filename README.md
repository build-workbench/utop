# utop

A lightweight htop clone built with Rust, ratatui, and sysinfo.

## Build & Run

```sh
cargo run --release
```

## Keys

| Key | Action |
|-----|--------|
| q | Quit |
| Up/Down, PgUp/PgDn, Home/End | Navigate |
| s | Cycle sort key (CPU/MEM/PID/NAME) |
| r | Toggle ascending/descending |
| / | Search processes |
| Esc | Clear filter |
| p | Pause/resume refresh |
| F5 | Force refresh |
| k | Kill selected process |
| d / Enter | Toggle process details |
| -/+ | Adjust refresh interval |

## License

MIT OR Apache-2.0
