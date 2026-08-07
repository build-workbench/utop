//! Snapshot collection: turns sysinfo state into the app's process list.
//!
//! This is the *only* module that talks to sysinfo. Everything it produces
//! — a `Vec<ProcRow>` — is plain data that the rest of the program can
//! sort, filter, and render without ever touching the live system. If you
//! want the pure logic (sorting, filtering, tree building), read `model`
//! instead — that's where the unit-tested algorithms live. This module is
//! thin glue: ask sysinfo for fresh numbers, hand them to `model`'s
//! functions, store the result back into `App`.

use std::collections::HashSet;

use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::app::App;
use crate::model::{ProcRow, build_tree, filter_processes, resolve_selected_index, selected_pid};

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
