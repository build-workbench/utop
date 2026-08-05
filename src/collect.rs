//! Snapshot collection: turns sysinfo state into the app's process list.

use std::collections::{HashMap, HashSet};

use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::app::App;
use crate::proc::{
    compare_proc_rows, filter_processes, resolve_selected_index, selected_pid, ProcRow, SortKey,
};

fn collect_processes(sys: &System) -> Vec<ProcRow> {
    sys.processes()
        .iter()
        .map(|(pid, p)| {
            let name = p.name().to_string_lossy().into_owned();
            let name_lc = name.to_lowercase();
            ProcRow {
                pid: *pid,
                name,
                name_lc,
                cpu: p.cpu_usage(),
                // sysinfo reports bytes; the table column is MiB.
                mem_mb: p.memory() / (1024 * 1024),
                ppid: p.parent(),
                depth: 0,
                has_children: false,
                collapsed: false,
            }
        })
        .collect()
}

/// Orders rows as a depth-first tree: roots and sibling groups sorted by
/// `sort_key`/`desc`, descendants of collapsed nodes omitted.
pub(crate) fn build_tree(
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

    let cmp = |a: &ProcRow, b: &ProcRow| compare_proc_rows(a, b, sort_key);
    let order = |list: &mut Vec<ProcRow>| {
        list.sort_by(cmp);
        if desc {
            list.reverse();
        }
    };
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

pub(crate) fn rebuild_processes(sys: &System, app: &mut App) {
    let preferred_pid = selected_pid(&app.processes, app.selected);
    let fallback_index = app.selected;
    let mut processes = collect_processes(sys);
    // Drop collapse markers of processes that no longer exist so the set
    // cannot grow unboundedly in long-running sessions.
    if !app.collapsed.is_empty() {
        let alive: HashSet<Pid> = processes.iter().map(|r| r.pid).collect();
        app.collapsed.retain(|pid| alive.contains(pid));
    }
    // Filtering flattens the view: a partial tree is more confusing than a list.
    let tree_active = app.tree_mode && app.filter.is_empty();
    if !app.filter.is_empty() {
        processes = filter_processes(processes, &app.filter);
    }
    if tree_active {
        app.processes = build_tree(processes, app.sort, app.desc, &app.collapsed);
        app.selected = resolve_selected_index(&app.processes, preferred_pid, fallback_index);
    } else {
        app.processes = processes;
        app.sort_processes_with_selection(preferred_pid, fallback_index);
    }
}

pub(crate) fn do_refresh(sys: &mut System, app: &mut App) {
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    rebuild_processes(sys, app);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pid: u32, ppid: Option<u32>, name: &str, cpu: f32) -> ProcRow {
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
    fn tree_nests_children_under_parent() {
        // init(1) -> bash(2) -> vim(3); sorting by PID ascending.
        let rows = vec![
            row(3, Some(2), "vim", 0.0),
            row(1, None, "init", 0.0),
            row(2, Some(1), "bash", 0.0),
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
            row(1, None, "init", 0.0),
            row(2, Some(1), "bash", 0.0),
            row(3, Some(2), "vim", 0.0),
        ];
        let mut collapsed = HashSet::new();
        collapsed.insert(Pid::from_u32(2));
        let out = build_tree(rows, SortKey::Pid, false, &collapsed);
        assert_eq!(pids(&out), vec![1, 2]);
        assert!(out[1].collapsed);
    }

    #[test]
    fn orphan_with_missing_parent_becomes_root() {
        let rows = vec![row(5, Some(999), "orphan", 0.0), row(1, None, "init", 0.0)];
        let out = build_tree(rows, SortKey::Pid, false, &HashSet::new());
        assert_eq!(pids(&out), vec![1, 5]);
        assert_eq!(out.iter().map(|r| r.depth).collect::<Vec<_>>(), vec![0, 0]);
    }

    #[test]
    fn siblings_are_sorted_within_parent() {
        let rows = vec![
            row(1, None, "init", 0.0),
            row(20, Some(1), "zz", 9.0),
            row(10, Some(1), "aa", 1.0),
        ];
        // Descending CPU: the hotter sibling comes first under init.
        let out = build_tree(rows, SortKey::Cpu, true, &HashSet::new());
        assert_eq!(pids(&out), vec![1, 20, 10]);
    }
}
