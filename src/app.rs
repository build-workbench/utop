//! Application state and input modes. Pure logic only — no I/O, no sysinfo calls.

use std::collections::HashSet;

use sysinfo::Pid;

use crate::cli::Config;
use crate::proc::{ProcRow, SortKey, compare_proc_rows, resolve_selected_index, selected_pid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputMode {
    Normal,
    Searching,
    ConfirmKill,
}

/// Number of ticks a status message stays visible before being cleared.
pub(crate) const STATUS_TTL_TICKS: u8 = 6;

pub(crate) struct App {
    pub(crate) sort: SortKey,
    pub(crate) desc: bool,
    pub(crate) selected: usize,
    pub(crate) processes: Vec<ProcRow>,
    pub(crate) filter: String,
    pub(crate) mode: InputMode,
    pub(crate) paused: bool,
    pub(crate) status: String,
    pub(crate) status_ttl: u8,
    pub(crate) show_details: bool,
    pub(crate) kill_target: Option<Pid>,
    pub(crate) tree_mode: bool,
    /// PIDs whose children are hidden in tree view.
    pub(crate) collapsed: HashSet<Pid>,
}

impl App {
    pub(crate) fn new(config: &Config) -> Self {
        Self {
            sort: config.sort,
            desc: config.desc,
            selected: 0,
            processes: Vec::new(),
            filter: config.filter.clone(),
            mode: InputMode::Normal,
            paused: false,
            status: String::new(),
            status_ttl: 0,
            show_details: false,
            kill_target: None,
            tree_mode: config.tree,
            collapsed: HashSet::new(),
        }
    }

    pub(crate) fn set_status(&mut self, msg: String) {
        self.status = msg;
        self.status_ttl = STATUS_TTL_TICKS;
    }

    pub(crate) fn cycle_sort(&mut self) {
        self.sort = match self.sort {
            SortKey::Cpu => SortKey::Mem,
            SortKey::Mem => SortKey::Pid,
            SortKey::Pid => SortKey::Name,
            SortKey::Name => SortKey::Cpu,
        };
    }

    pub(crate) fn sort_processes(&mut self) {
        let preferred_pid = selected_pid(&self.processes, self.selected);
        self.sort_processes_with_selection(preferred_pid, self.selected);
    }

    pub(crate) fn sort_processes_with_selection(
        &mut self,
        preferred_pid: Option<Pid>,
        fallback_index: usize,
    ) {
        self.processes
            .sort_by(|a, b| compare_proc_rows(a, b, self.sort));
        if self.desc {
            self.processes.reverse();
        }
        self.selected = resolve_selected_index(&self.processes, preferred_pid, fallback_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        App::new(&Config::default())
    }

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
    fn cycle_sort_rotates_keys() {
        let mut app = app();
        app.cycle_sort();
        assert!(matches!(app.sort, SortKey::Mem));
        app.cycle_sort();
        assert!(matches!(app.sort, SortKey::Pid));
        app.cycle_sort();
        assert!(matches!(app.sort, SortKey::Name));
        app.cycle_sort();
        assert!(matches!(app.sort, SortKey::Cpu));
    }

    #[test]
    fn sort_processes_cpu_desc_with_tiebreakers() {
        let mut app = app();
        app.processes = vec![
            row(30, "zeta", 10.0, 100),
            row(10, "alpha", 10.0, 200),
            row(20, "beta", 5.0, 400),
        ];
        app.sort = SortKey::Cpu;
        app.desc = true;
        app.sort_processes();

        let pids: Vec<u32> = app.processes.iter().map(|r| r.pid.as_u32()).collect();
        assert_eq!(pids, vec![10, 30, 20]);
    }

    #[test]
    fn sort_processes_pid_ascending() {
        let mut app = app();
        app.processes = vec![
            row(30, "zeta", 10.0, 100),
            row(10, "alpha", 10.0, 200),
            row(20, "beta", 5.0, 400),
        ];
        app.sort = SortKey::Pid;
        app.desc = false;
        app.sort_processes();

        let pids: Vec<u32> = app.processes.iter().map(|r| r.pid.as_u32()).collect();
        assert_eq!(pids, vec![10, 20, 30]);
    }
}
