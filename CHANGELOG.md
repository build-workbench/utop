# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-07-28

### Added

- Per-core CPU meters in the summary panel with load coloring
  (green < 60%, yellow < 85%, red ≥ 85%).
- Load-colored CPU% values in the process table and summary.
- Tree view (`t`) built from parent PIDs, with collapsible subtrees
  (Space to collapse/expand); sorting applies within sibling groups.
- Kill confirmation with signal choice: `k` then `y` = SIGTERM,
  `K` = SIGKILL, Esc = cancel.
- Command-line options: `--sort`, `--asc`, `--delay`, `--filter`, `--tree`,
  `--help`.
- Mouse wheel scrolling of the process list.
- PPID shown in the process details panel.
- CI workflow (rustfmt, clippy with `-D warnings`, tests, release build).
- This changelog.

### Changed

- Upgraded `sysinfo` from 0.29 to 0.38 (migrated off the removed
  `SystemExt`/`ProcessExt`/`CpuExt`/`PidExt` trait APIs).
- Split the single 664-line `main.rs` into focused modules:
  `app`, `proc`, `collect`, `ui`, `cli`.
- Status messages now auto-clear after a few ticks instead of lingering
  forever.
- Ctrl+C now exits from any input mode (raw mode swallows SIGINT, so it is
  handled as a key event).

### Fixed

- Search mode no longer rebuilds the process list twice when Esc is pressed.
- Sorting and filtering no longer re-allocate a lowercased copy of every
  process name on each comparison; the lowercase name is precomputed once
  per snapshot.

## [0.2.0] - earlier

- Single-crate rewrite: ratatui-based TUI with sorting, filtering, pause,
  kill, and a details panel.
