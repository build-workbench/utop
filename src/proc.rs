//! Process row model and pure helpers shared across collection, sorting and UI.

use std::cmp::Ordering;

use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{Cell, Row},
};
use sysinfo::Pid;

#[derive(Clone, Debug)]
pub(crate) struct ProcRow {
    pub(crate) pid: Pid,
    pub(crate) name: String,
    /// Precomputed lowercase name so sorting/filtering never re-allocates.
    pub(crate) name_lc: String,
    pub(crate) cpu: f32,
    pub(crate) mem_mb: u64,
    /// Parent PID, used to build the tree view.
    pub(crate) ppid: Option<Pid>,
    /// Indentation level in tree view (0 in flat list mode).
    pub(crate) depth: u16,
    /// Whether this row has children in tree view.
    pub(crate) has_children: bool,
    /// Whether this row's children are hidden in tree view.
    pub(crate) collapsed: bool,
}

impl ProcRow {
    pub(crate) fn as_row(&self) -> Row<'static> {
        // Per-process CPU% is relative to one core, so it can exceed 100%
        // on multi-core machines; scale the thresholds accordingly.
        let cpu_color = if self.cpu < 50.0 {
            Color::Green
        } else if self.cpu < 150.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        Row::new(vec![
            Cell::from(self.pid.as_u32().to_string()),
            Cell::from(self.display_name()),
            Cell::from(Span::styled(
                format!("{:>6.1}", self.cpu),
                Style::default().fg(cpu_color),
            )),
            Cell::from(format!("{:>10}", self.mem_mb)),
        ])
    }

    /// Tree-indented name with a collapse marker; plain name in list mode.
    fn display_name(&self) -> String {
        if self.depth == 0 && !self.has_children {
            return self.name.clone();
        }
        let marker = if !self.has_children {
            "   "
        } else if self.collapsed {
            "[+]"
        } else {
            "[-]"
        };
        format!(
            "{}{} {}",
            "  ".repeat(self.depth as usize),
            marker,
            self.name
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SortKey {
    Cpu,
    Mem,
    Pid,
    Name,
}

pub(crate) fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort_key: SortKey) -> Ordering {
    match sort_key {
        SortKey::Cpu => a
            .cpu
            .partial_cmp(&b.cpu)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32()))
            .then_with(|| a.name_lc.cmp(&b.name_lc)),
        SortKey::Mem => a
            .mem_mb
            .cmp(&b.mem_mb)
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32()))
            .then_with(|| a.name_lc.cmp(&b.name_lc)),
        SortKey::Pid => a
            .pid
            .as_u32()
            .cmp(&b.pid.as_u32())
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.name_lc.cmp(&b.name_lc)),
        SortKey::Name => a
            .name_lc
            .cmp(&b.name_lc)
            .then_with(|| a.cpu.partial_cmp(&b.cpu).unwrap_or(Ordering::Equal))
            .then_with(|| a.mem_mb.cmp(&b.mem_mb))
            .then_with(|| a.pid.as_u32().cmp(&b.pid.as_u32())),
    }
}

pub(crate) fn filter_processes(processes: Vec<ProcRow>, query: &str) -> Vec<ProcRow> {
    if query.is_empty() {
        return processes;
    }
    let q = query.to_lowercase();
    processes
        .into_iter()
        .filter(|row| row.name_lc.contains(&q) || row.pid.as_u32().to_string().contains(&q))
        .collect()
}

pub(crate) fn selected_pid(processes: &[ProcRow], selected: usize) -> Option<Pid> {
    processes.get(selected).map(|row| row.pid)
}

pub(crate) fn resolve_selected_index(
    processes: &[ProcRow],
    preferred_pid: Option<Pid>,
    fallback_index: usize,
) -> usize {
    if processes.is_empty() {
        return 0;
    }
    if let Some(pid) = preferred_pid
        && let Some(index) = processes.iter().position(|row| row.pid == pid)
    {
        return index;
    }
    fallback_index.min(processes.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pid: u32, name: &str, cpu: f32, mem_mb: u64) -> ProcRow {
        ProcRow {
            pid: Pid::from_u32(pid),
            name: name.to_string(),
            name_lc: name.to_lowercase(),
            cpu,
            mem_mb,
            ppid: None,
            depth: 0,
            has_children: false,
            collapsed: false,
        }
    }

    #[test]
    fn compare_cpu_uses_mem_then_pid_as_tiebreakers() {
        let a = row(30, "zeta", 10.0, 100);
        let b = row(10, "alpha", 10.0, 200);
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
}
