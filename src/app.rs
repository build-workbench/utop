//! Application state and input modes. Pure logic only — no I/O, no sysinfo calls.
//!
//! `App` is the single authority on every piece of mutable state in utop:
//! which process is selected, what the filter is, which mode we're in,
//! whether the tree is collapsed, etc. The event loop in `main` reads a
//! key, calls an `App` method to mutate state, then decides whether a
//! sysinfo rebuild is needed. State never changes behind `App`'s back.

use std::collections::HashSet;

use sysinfo::Pid;

use crate::cli::Config;
use crate::model::{ProcRow, SortKey, compare_proc_rows, resolve_selected_index, selected_pid};

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

    // -- view toggles ------------------------------------------------------

    pub(crate) fn toggle_paused(&mut self) {
        self.paused = !self.paused;
    }

    pub(crate) fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    pub(crate) fn toggle_tree(&mut self) {
        self.tree_mode = !self.tree_mode;
    }

    /// Toggles the collapsed state of the selected row's subtree (tree view
    /// only). Returns the PID if a toggle actually happened, so the caller
    /// can trigger a rebuild.
    pub(crate) fn toggle_collapse_selected(&mut self) -> Option<Pid> {
        if !self.tree_mode {
            return None;
        }
        let row = self.processes.get(self.selected)?;
        let pid = row.pid;
        if !self.collapsed.remove(&pid) {
            self.collapsed.insert(pid);
        }
        Some(pid)
    }

    // -- selection ---------------------------------------------------------

    pub(crate) fn move_up(&mut self, step: usize) {
        self.selected = self.selected.saturating_sub(step);
    }

    pub(crate) fn move_down(&mut self, step: usize) {
        if self.selected + step < self.processes.len() {
            self.selected += step;
        } else {
            self.selected = self.processes.len().saturating_sub(1);
        }
    }

    pub(crate) fn select_first(&mut self) {
        self.selected = 0;
    }

    pub(crate) fn select_last(&mut self) {
        self.selected = self.processes.len().saturating_sub(1);
    }

    // -- filter (search mode) ---------------------------------------------

    pub(crate) fn push_filter_char(&mut self, c: char) {
        self.filter.push(c);
    }

    pub(crate) fn pop_filter(&mut self) {
        self.filter.pop();
    }

    pub(crate) fn clear_filter(&mut self) {
        self.filter.clear();
    }

    pub(crate) fn enter_search(&mut self) {
        self.mode = InputMode::Searching;
    }

    // -- kill flow ---------------------------------------------------------

    /// Starts the kill confirmation for the selected process. Returns the
    /// PID and name so the caller can show a status prompt.
    pub(crate) fn begin_kill_selected(&mut self) -> Option<(Pid, String)> {
        let row = self.processes.get(self.selected)?;
        let pid = row.pid;
        let name = row.name.clone();
        self.kill_target = Some(pid);
        self.mode = InputMode::ConfirmKill;
        Some((pid, name))
    }

    pub(crate) fn take_kill_target(&mut self) -> Option<Pid> {
        self.kill_target.take()
    }

    pub(crate) fn enter_normal(&mut self) {
        self.mode = InputMode::Normal;
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

    #[test]
    fn move_down_clamps_at_last_row() {
        let mut app = app();
        app.processes = vec![row(1, "a", 1.0, 10), row(2, "b", 2.0, 20)];
        app.move_down(1);
        assert_eq!(app.selected, 1);
        // Past the end: clamps to last index, not beyond.
        app.move_down(1);
        assert_eq!(app.selected, 1);
        // Big step also clamps.
        app.move_down(100);
        assert_eq!(app.selected, 1);
    }

    #[test]
    fn move_up_clamps_at_zero() {
        let mut app = app();
        app.processes = vec![row(1, "a", 1.0, 10), row(2, "b", 2.0, 20)];
        app.selected = 1;
        app.move_up(1);
        assert_eq!(app.selected, 0);
        // Below zero: saturates at 0.
        app.move_up(1);
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn move_down_empty_list_stays_zero() {
        let mut app = app();
        app.move_down(5);
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn toggle_collapse_selected_only_in_tree_mode() {
        let mut app = app();
        app.processes = vec![row(1, "init", 0.0, 10)];
        app.selected = 0;
        // Not tree mode: no-op.
        assert_eq!(app.toggle_collapse_selected(), None);
        assert!(app.collapsed.is_empty());
        // Tree mode: toggles the PID into the set.
        app.tree_mode = true;
        assert_eq!(app.toggle_collapse_selected(), Some(Pid::from_u32(1)));
        assert!(app.collapsed.contains(&Pid::from_u32(1)));
        // Toggle again: removes it.
        assert_eq!(app.toggle_collapse_selected(), Some(Pid::from_u32(1)));
        assert!(app.collapsed.is_empty());
    }

    #[test]
    fn toggle_collapse_selected_empty_list_returns_none() {
        let mut app = app();
        app.tree_mode = true;
        assert_eq!(app.toggle_collapse_selected(), None);
    }

    #[test]
    fn begin_kill_selected_returns_pid_and_name() {
        let mut app = app();
        app.processes = vec![row(42, "vim", 5.0, 30)];
        app.selected = 0;
        let (pid, name) = app.begin_kill_selected().unwrap();
        assert_eq!(pid.as_u32(), 42);
        assert_eq!(name, "vim");
        assert_eq!(app.kill_target, Some(Pid::from_u32(42)));
        assert!(matches!(app.mode, InputMode::ConfirmKill));
    }

    #[test]
    fn begin_kill_selected_empty_list_returns_none() {
        let mut app = app();
        assert_eq!(app.begin_kill_selected(), None);
        assert!(matches!(app.mode, InputMode::Normal));
        assert_eq!(app.kill_target, None);
    }

    #[test]
    fn take_kill_target_drains_the_target() {
        let mut app = app();
        app.kill_target = Some(Pid::from_u32(7));
        assert_eq!(app.take_kill_target(), Some(Pid::from_u32(7)));
        assert_eq!(app.take_kill_target(), None);
    }
}
