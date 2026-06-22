//! Shared logic for the Rust `htop` implementations (Unix and Windows).
//!
//! Keeps process row modeling, sorting, filtering, selection stabilization,
//! and ratio-based coloring in one place so both platform UIs stay consistent.

use std::cmp::Ordering;

use ratatui::style::Color;
use ratatui::widgets::Row;
use sysinfo::{Pid, PidExt};

/// A single row in the process table.
#[derive(Clone, Debug)]
pub struct ProcRow {
    pub pid: Pid,
    pub name: String,
    pub cpu: f32,
    /// Memory usage in MiB.
    pub mem_mb: u64,
}

impl ProcRow {
    /// Render the row as a `ratatui` table row: `[pid, name, cpu, mem_mb]`.
    pub fn as_row(&self) -> Row<'static> {
        Row::new(vec![
            self.pid.as_u32().to_string(),
            self.name.clone(),
            format!("{:>6.1}", self.cpu),
            format!("{:>10}", self.mem_mb),
        ])
    }
}

/// Sort key for the process table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    Cpu,
    Mem,
    Pid,
    Name,
}

/// Compare two rows by the given sort key using deterministic tie-breakers.
///
/// Returns ascending order; callers reverse for descending display.
pub fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort_key: SortKey) -> Ordering {
    match sort_key {
        SortKey::Cpu => a
            .cpu
            .partial_cmp(&b.cpu)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32()))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortKey::Mem => a
            .mem_mb
            .cmp(&b.mem_mb)
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32()))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortKey::Pid => a
            .pid
            .as_u32()
            .cmp(&b.pid.as_u32())
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortKey::Name => a
            .name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32())),
    }
}

/// Filter rows by a case-insensitive query that matches the process name or PID.
pub fn filter_processes(processes: Vec<ProcRow>, query: &str) -> Vec<ProcRow> {
    if query.is_empty() {
        return processes;
    }
    let q = query.to_lowercase();
    processes
        .into_iter()
        .filter(|row| {
            row.name.to_lowercase().contains(&q) || row.pid.as_u32().to_string().contains(&q)
        })
        .collect()
}

/// Return the PID currently selected by index, if any.
pub fn selected_pid(processes: &[ProcRow], selected: usize) -> Option<Pid> {
    processes.get(selected).map(|row| row.pid)
}

/// Resolve the selected index after a resort, preferring the previously
/// selected PID, falling back to a clamped index.
pub fn resolve_selected_index(
    processes: &[ProcRow],
    preferred_pid: Option<Pid>,
    fallback_index: usize,
) -> usize {
    if processes.is_empty() {
        return 0;
    }
    if let Some(pid) = preferred_pid {
        if let Some(index) = processes.iter().position(|row| row.pid == pid) {
            return index;
        }
    }
    fallback_index.min(processes.len() - 1)
}

/// Map a 0.0..=1.0 ratio to a traffic-light color.
///
/// - `< 0.5`  → `LightGreen`
/// - `< 0.8`  → `Yellow`
/// - `>= 0.8` → `Red`
pub fn color_for_ratio(ratio: f32) -> Color {
    if ratio < 0.5 {
        Color::LightGreen
    } else if ratio < 0.8 {
        Color::Yellow
    } else {
        Color::Red
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pid: u32, name: &str, cpu: f32, mem_mb: u64) -> ProcRow {
        ProcRow {
            pid: Pid::from_u32(pid),
            name: name.to_string(),
            cpu,
            mem_mb,
        }
    }

    #[test]
    fn compare_cpu_uses_mem_then_pid_as_tiebreakers() {
        let a = row(30, "zeta", 10.0, 100);
        let b = row(10, "alpha", 10.0, 200);
        // cpu equal, mem 100 < 200 → a before b (ascending)
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Cpu), Ordering::Less);
    }

    #[test]
    fn compare_pid_ascending() {
        let a = row(30, "zeta", 10.0, 100);
        let b = row(10, "alpha", 10.0, 200);
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Pid), Ordering::Greater);
    }

    #[test]
    fn filter_matches_name_case_insensitively() {
        let rows = vec![row(1, "Python", 1.0, 10), row(2, "rust", 2.0, 20)];
        let out = filter_processes(rows, "py");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].pid.as_u32(), 1);
    }

    #[test]
    fn filter_matches_pid_string() {
        let rows = vec![row(1234, "Python", 1.0, 10), row(2, "rust", 2.0, 20)];
        let out = filter_processes(rows, "123");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].pid.as_u32(), 1234);
    }

    #[test]
    fn filter_empty_query_returns_all() {
        let rows = vec![row(1, "a", 1.0, 10), row(2, "b", 2.0, 20)];
        let out = filter_processes(rows, "");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn resolve_selected_prefers_existing_pid() {
        let rows = vec![
            row(1, "a", 1.0, 10),
            row(2, "b", 2.0, 20),
            row(3, "c", 3.0, 30),
        ];
        let idx = resolve_selected_index(&rows, Some(Pid::from_u32(2)), 0);
        assert_eq!(idx, 1);
    }

    #[test]
    fn resolve_selected_falls_back_when_pid_missing() {
        let rows = vec![row(1, "a", 1.0, 10), row(2, "b", 2.0, 20)];
        let idx = resolve_selected_index(&rows, Some(Pid::from_u32(99)), 5);
        assert_eq!(idx, 1);
    }

    #[test]
    fn resolve_selected_empty_returns_zero() {
        let idx = resolve_selected_index(&[], Some(Pid::from_u32(99)), 5);
        assert_eq!(idx, 0);
    }

    #[test]
    fn color_for_ratio_thresholds() {
        assert_eq!(color_for_ratio(0.0), Color::LightGreen);
        assert_eq!(color_for_ratio(0.49), Color::LightGreen);
        assert_eq!(color_for_ratio(0.5), Color::Yellow);
        assert_eq!(color_for_ratio(0.79), Color::Yellow);
        assert_eq!(color_for_ratio(0.8), Color::Red);
        assert_eq!(color_for_ratio(1.0), Color::Red);
    }
}
