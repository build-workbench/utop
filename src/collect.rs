//! Snapshot collection: the *only* module that talks to sysinfo.
//!
//! Everything it produces — a `Vec<ProcRow>` for the process list plus
//! `SysStats` / `ProcDetails` snapshots stored on `App` — is plain data that
//! the rest of the program can sort, filter, and render without ever
//! touching the live system. If you want the pure logic (sorting, filtering,
//! tree building), read `model` instead — that's where the unit-tested
//! algorithms live. This module is thin glue: ask sysinfo for fresh numbers,
//! hand them to `model`'s functions, store the result back into `App`.

use std::collections::HashSet;
use std::time::Duration;

use sysinfo::{Pid as SysPid, ProcessRefreshKind, ProcessesToUpdate, Signal, UpdateKind};

use crate::app::App;
use crate::model::{
    Pid, ProcDetails, ProcRow, SignalKind, SysStats, build_tree, filter_processes,
    resolve_selected_index, selected_pid,
};

// Re-exported so the event loop can name the `System` type without
// importing sysinfo itself: `collect` stays the single sysinfo gateway.
pub(crate) use sysinfo::System;

/// What utop displays per process: CPU, memory, executable path, command
/// line, and the thread list (threads are rows too, nested under their
/// parent in tree view). Disk usage is never displayed, so it is skipped.
fn refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing()
        .with_memory()
        .with_cpu()
        .with_exe(UpdateKind::OnlyIfNotSet)
        .with_cmd(UpdateKind::OnlyIfNotSet)
        .with_tasks()
}

fn collect_processes(sys: &System) -> Vec<ProcRow> {
    sys.processes()
        .iter()
        .map(|(pid, p)| {
            let name = p.name().to_string_lossy().into_owned();
            let name_lc = name.to_lowercase();
            ProcRow {
                pid: Pid::from_u32(pid.as_u32()),
                name,
                name_lc,
                cpu: p.cpu_usage(),
                // sysinfo reports bytes; the table column is MiB.
                mem_mb: p.memory() / (1024 * 1024),
                ppid: p.parent().map(|pp| Pid::from_u32(pp.as_u32())),
                depth: 0,
                has_children: false,
                collapsed: false,
            }
        })
        .collect()
}

/// Creates the system sampler and takes the two startup samples sysinfo
/// needs before CPU percentages are meaningful. sysinfo discards samples
/// closer together than `MINIMUM_CPU_UPDATE_INTERVAL` (200 ms on Linux), so
/// the gap between the two samples is at least that. Call this *before*
/// taking over the terminal so the sampling gap is spent while the user's
/// shell is still visible instead of on a blank alternate screen.
pub(crate) fn init_system() -> System {
    let mut sys = System::new();
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh_kind());
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL + Duration::from_millis(20));
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh_kind());
    sys
}

/// Rebuilds the process list from the current samples and refreshes the
/// stats snapshot. Does not sample; use [`do_refresh`] for that.
pub(crate) fn sync_app(sys: &mut System, app: &mut App) {
    rebuild_processes(sys, app);
    app.stats = collect_stats(sys);
}

/// Takes fresh samples and rebuilds everything on `App`.
pub(crate) fn do_refresh(sys: &mut System, app: &mut App) {
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, refresh_kind());
    sync_app(sys, app);
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
    let tree_active = app.tree_active();
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

/// Snapshots the currently selected process into `App::details`, or `None`
/// if the selection is empty or the process no longer exists. Cheap: one
/// process lookup, called once per frame by the event loop so the details
/// panel tracks selection changes immediately.
pub(crate) fn update_details(sys: &mut System, app: &mut App) {
    app.details = app.processes.get(app.selected).and_then(|row| {
        let p = sys.process(SysPid::from_u32(row.pid.as_u32()))?;
        Some(ProcDetails {
            name: p.name().to_string_lossy().into_owned(),
            ppid: p.parent().map(|pp| Pid::from_u32(pp.as_u32())),
            status: format!("{:?}", p.status()),
            cpu: p.cpu_usage(),
            mem_mib: p.memory() as f64 / (1024.0 * 1024.0),
            exe: p.exe().map(|e| e.display().to_string()).unwrap_or_default(),
            cmd: p
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" "),
        })
    });
}

/// What happened when trying to kill a process.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KillOutcome {
    /// Signal delivered.
    Sent,
    /// The platform refused (permission, zombie, ...).
    Failed,
    /// The signal is not supported on this platform.
    Unsupported,
    /// The process no longer exists.
    Gone,
}

/// Sends the signal to the process. The structured outcome lets the event
/// loop turn it into a status message (and tests can assert on it).
pub(crate) fn kill(sys: &mut System, pid: Pid, kind: SignalKind) -> KillOutcome {
    let signal = match kind {
        SignalKind::Term => Signal::Term,
        SignalKind::Kill => Signal::Kill,
    };
    let Some(process) = sys.process(SysPid::from_u32(pid.as_u32())) else {
        return KillOutcome::Gone;
    };
    match process.kill_with(signal) {
        Some(true) => KillOutcome::Sent,
        Some(false) => KillOutcome::Failed,
        None => KillOutcome::Unsupported,
    }
}

fn collect_stats(sys: &System) -> SysStats {
    let cpus = sys.cpus();
    let core_usages: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();
    let cores = cpus.len();
    let cpu_avg = if cores > 0 {
        core_usages.iter().map(|u| *u as f64).sum::<f64>() / cores as f64
    } else {
        0.0
    };

    let total = sys.total_memory().max(1);
    let used = sys.used_memory().min(total);
    let load = System::load_average();

    SysStats {
        cpu_avg,
        core_usages,
        mem_used_gib: used as f64 / (1024.0 * 1024.0 * 1024.0),
        mem_total_gib: total as f64 / (1024.0 * 1024.0 * 1024.0),
        mem_pct: used as f64 * 100.0 / total as f64,
        load_one: load.one,
        load_five: load.five,
        load_fifteen: load.fifteen,
        load_pct: if cores > 0 {
            load.one * 100.0 / cores as f64
        } else {
            0.0
        },
        uptime_secs: System::uptime(),
    }
}
