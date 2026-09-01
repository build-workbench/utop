//! Process row model and pure helpers shared across collection, sorting and UI.
//!
//! Everything here is pure: no sysinfo, no I/O. This is the module to read
//! if you want to understand how a process list is sorted, filtered, or
//! shaped into a tree — all of which can be reasoned about and tested in
//! isolation from the live system.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

/// Process identifier. A plain newtype so the pure modules never need to
/// import anything from sysinfo; `collect` converts to and from sysinfo's
/// own PID type at the boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pid(u32);

impl Pid {
    pub fn from_u32(pid: u32) -> Self {
        Self(pid)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// Signal to deliver when killing a process, as chosen in the kill
/// confirmation. Maps to `sysinfo::Signal` inside `collect`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SignalKind {
    Term,
    Kill,
}

impl SignalKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            SignalKind::Term => "SIGTERM",
            SignalKind::Kill => "SIGKILL",
        }
    }
}

/// System-wide stats snapshot for the summary panel, taken by `collect` on
/// every refresh. Plain data so the UI never needs the live system.
#[derive(Clone, Debug, Default)]
pub(crate) struct SysStats {
    pub(crate) cpu_avg: f64,
    /// Per-core usage percentages, in sysinfo's core order.
    pub(crate) core_usages: Vec<f32>,
    pub(crate) mem_used_gib: f64,
    pub(crate) mem_total_gib: f64,
    pub(crate) mem_pct: f64,
    pub(crate) load_one: f64,
    pub(crate) load_five: f64,
    pub(crate) load_fifteen: f64,
    /// 1-minute load relative to the core count, so the color thresholds
    /// mean the same thing on any machine.
    pub(crate) load_pct: f64,
    pub(crate) uptime_secs: u64,
}

/// Details of the currently selected process, snapshotted by `collect`.
/// `None` once the process no longer exists.
#[derive(Clone, Debug)]
pub(crate) struct ProcDetails {
    pub(crate) name: String,
    pub(crate) ppid: Option<Pid>,
    pub(crate) status: String,
    pub(crate) cpu: f32,
    pub(crate) mem_mib: f64,
    pub(crate) exe: String,
    pub(crate) cmd: String,
}

#[derive(Clone, Debug)]
pub struct ProcRow {
    pub pid: Pid,
    pub name: String,
    /// Precomputed lowercase name so sorting/filtering never re-allocates.
    pub name_lc: String,
    pub cpu: f32,
    pub mem_mb: u64,
    /// Parent PID, used to build the tree view.
    pub ppid: Option<Pid>,
    /// Indentation level in tree view (0 in flat list mode).
    pub depth: u16,
    /// Whether this row has children in tree view.
    pub has_children: bool,
    /// Whether this row's children are hidden in tree view.
    pub collapsed: bool,
}

/// Tree-indented name with a collapse marker; plain name in list mode.
pub fn display_name(row: &ProcRow) -> String {
    if row.depth == 0 && !row.has_children {
        return row.name.clone();
    }
    let marker = if !row.has_children {
        "   "
    } else if row.collapsed {
        "[+]"
    } else {
        "[-]"
    };
    format!("{}{} {}", "  ".repeat(row.depth as usize), marker, row.name)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    Cpu,
    Mem,
    Pid,
    Name,
}

pub fn compare_proc_rows(a: &ProcRow, b: &ProcRow, sort_key: SortKey) -> Ordering {
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

/// Shared sort-then-reverse policy, used for both the flat list and the
/// tree's sibling groups so the two orderings can never diverge.
pub(crate) fn sort_rows(list: &mut [ProcRow], key: SortKey, desc: bool) {
    list.sort_by(|a, b| compare_proc_rows(a, b, key));
    if desc {
        list.reverse();
    }
}

pub fn filter_processes(processes: Vec<ProcRow>, query: &str) -> Vec<ProcRow> {
    if query.is_empty() {
        return processes;
    }
    let q = query.to_lowercase();
    processes
        .into_iter()
        .filter(|row| row.name_lc.contains(&q) || row.pid.as_u32().to_string().contains(&q))
        .collect()
}

pub fn selected_pid(processes: &[ProcRow], selected: usize) -> Option<Pid> {
    processes.get(selected).map(|row| row.pid)
}

pub fn resolve_selected_index(
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

/// Orders rows as a depth-first tree: roots and sibling groups sorted by
/// `sort_key`/`desc`, descendants of collapsed nodes omitted.
///
/// Pure: takes a flat list of rows (each carrying its `ppid`) and the set of
/// collapsed PIDs, returns a new list ordered for tree display with `depth`
/// and `has_children`/`collapsed` filled in. No sysinfo involved.
pub fn build_tree(
    rows: Vec<ProcRow>,
    sort_key: SortKey,
    desc: bool,
    collapsed: &HashSet<Pid>,
) -> Vec<ProcRow> {
    let pids: HashSet<Pid> = rows.iter().map(|r| r.pid).collect();
    let mut children: HashMap<Pid, Vec<ProcRow>> = HashMap::new();
    let mut roots: Vec<ProcRow> = Vec::new();
    for row in rows {
        match row.ppid {
            Some(ppid) if pids.contains(&ppid) => children.entry(ppid).or_default().push(row),
            _ => roots.push(row),
        }
    }

    let order = |list: &mut Vec<ProcRow>| sort_rows(list, sort_key, desc);
    order(&mut roots);
    for list in children.values_mut() {
        order(list);
    }

    // Iterative DFS; push in reverse so the best-sorted item pops first.
    let mut out = Vec::new();
    let mut stack: Vec<(ProcRow, u16)> = roots.into_iter().rev().map(|r| (r, 0)).collect();
    while let Some((mut row, depth)) = stack.pop() {
        let kids = children.remove(&row.pid).unwrap_or_default();
        row.depth = depth;
        row.has_children = !kids.is_empty();
        row.collapsed = collapsed.contains(&row.pid);
        let hide_kids = row.collapsed;
        out.push(row);
        if !hide_kids {
            for kid in kids.into_iter().rev() {
                stack.push((kid, depth + 1));
            }
        }
    }
    out
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::*;

    /// Shared `ProcRow` fixture builder for this module's and `app`'s tests.
    pub(crate) fn row(pid: u32, name: &str, cpu: f32, mem_mb: u64) -> ProcRow {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use testutil::row;

    fn tree_row(pid: u32, ppid: Option<u32>, name: &str, cpu: f32) -> ProcRow {
        ProcRow {
            pid: Pid::from_u32(pid),
            name: name.to_string(),
            name_lc: name.to_lowercase(),
            cpu,
            mem_mb: 10,
            ppid: ppid.map(Pid::from_u32),
            depth: 0,
            has_children: false,
            collapsed: false,
        }
    }

    fn pids(rows: &[ProcRow]) -> Vec<u32> {
        rows.iter().map(|r| r.pid.as_u32()).collect()
    }

    #[test]
    fn compare_cpu_uses_mem_then_pid_as_tiebreakers() {
        let a = row(30, "zeta", 10.0, 100);
        let b = row(10, "alpha", 10.0, 200);
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Cpu), Ordering::Less);
    }

    #[test]
    fn compare_mem_orders_by_memory_then_cpu() {
        let a = row(30, "zeta", 10.0, 100);
        let b = row(10, "alpha", 5.0, 200);
        // Primary key: memory (a has less, so a comes first ascending).
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Mem), Ordering::Less);
        // Tie on memory: lower CPU first.
        let b = row(10, "alpha", 15.0, 100);
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Mem), Ordering::Less);
        // Tie on memory and CPU: lower PID first.
        let b = row(10, "alpha", 10.0, 100);
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Mem), Ordering::Greater);
    }

    #[test]
    fn compare_name_orders_case_insensitively_then_cpu() {
        let a = row(1, "alpha", 1.0, 10);
        let b = row(2, "BETA", 2.0, 10);
        // "alpha" < "beta" after lowercasing.
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Name), Ordering::Less);
        // Case-insensitive tie: lower CPU first.
        let b = row(2, "ALPHA", 2.0, 10);
        assert_eq!(compare_proc_rows(&a, &b, SortKey::Name), Ordering::Less);
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
    fn tree_nests_children_under_parent() {
        // init(1) -> bash(2) -> vim(3); sorting by PID ascending.
        let rows = vec![
            tree_row(3, Some(2), "vim", 0.0),
            tree_row(1, None, "init", 0.0),
            tree_row(2, Some(1), "bash", 0.0),
        ];
        let out = build_tree(rows, SortKey::Pid, false, &HashSet::new());
        assert_eq!(pids(&out), vec![1, 2, 3]);
        let depths: Vec<u16> = out.iter().map(|r| r.depth).collect();
        assert_eq!(depths, vec![0, 1, 2]);
        assert!(out[0].has_children);
        assert!(out[1].has_children);
        assert!(!out[2].has_children);
    }

    #[test]
    fn collapsed_node_hides_descendants() {
        let rows = vec![
            tree_row(1, None, "init", 0.0),
            tree_row(2, Some(1), "bash", 0.0),
            tree_row(3, Some(2), "vim", 0.0),
        ];
        let mut collapsed = HashSet::new();
        collapsed.insert(Pid::from_u32(2));
        let out = build_tree(rows, SortKey::Pid, false, &collapsed);
        assert_eq!(pids(&out), vec![1, 2]);
        assert!(out[1].collapsed);
    }

    #[test]
    fn orphan_with_missing_parent_becomes_root() {
        let rows = vec![
            tree_row(5, Some(999), "orphan", 0.0),
            tree_row(1, None, "init", 0.0),
        ];
        let out = build_tree(rows, SortKey::Pid, false, &HashSet::new());
        assert_eq!(pids(&out), vec![1, 5]);
        assert_eq!(out.iter().map(|r| r.depth).collect::<Vec<_>>(), vec![0, 0]);
    }

    #[test]
    fn siblings_are_sorted_within_parent() {
        let rows = vec![
            tree_row(1, None, "init", 0.0),
            tree_row(20, Some(1), "zz", 9.0),
            tree_row(10, Some(1), "aa", 1.0),
        ];
        // Descending CPU: the hotter sibling comes first under init.
        let out = build_tree(rows, SortKey::Cpu, true, &HashSet::new());
        assert_eq!(pids(&out), vec![1, 20, 10]);
    }
}
