# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-08-05

### Added

- Load average (1/5/15 min, colored relative to core count) and uptime
  in the summary panel.
- Process count in the table title: `Processes (N)`.
- `-V` / `--version` flag.
- Status-line feedback when adjusting the refresh interval with `-` / `+`.
- CI now also runs on macOS; package metadata (repository, readme,
  keywords, categories) in Cargo.toml.

### Changed

- Startup only samples CPU, memory and processes — disks, networks,
  components and users were enumerated twice but never displayed
  (~560 ms of wasted work removed, startup is ~2x faster).
- The terminal is restored if utop panics, instead of leaving the
  shell stuck in raw mode behind the alternate screen.
- Summary is split into three lines (stats / mode / key help); the old
  216-character single line was truncated on terminals under ~200
  columns.
- The NAME column now fills the remaining table width instead of a
  fixed 55%.

### Fixed

- First frame showed 0.0% CPU everywhere: the two startup samples were
  taken back-to-back with no interval between them. Samples are now
  spaced 150 ms apart, so the first paint already has real numbers.
- Process memory column and details panel displayed KiB labeled as MB;
  the summary displayed MiB labeled as GiB. Both are now true MiB/GiB.
- Collapsed-subtree markers of dead processes were kept forever; the
  set is now pruned on every rebuild.

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
