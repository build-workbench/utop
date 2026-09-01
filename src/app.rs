//! Application state and input modes. Pure logic only — no I/O, no sysinfo.
//!
//! `App` is the single authority on every piece of mutable state in utop:
//! which process is selected, what the filter is, which mode we're in,
//! whether the tree is collapsed, etc. Input handlers only call `App`
//! methods and record what collection work they need via `Followup`; the
//! event loop performs that work and stores fresh snapshots back into `App`.
//! State never changes behind `App`'s back.

use std::collections::HashSet;

use crate::cli::{Config, MAX_DELAY_MS, MIN_DELAY_MS};
use crate::model::{
    Pid, ProcDetails, ProcRow, SignalKind, SortKey, SysStats, resolve_selected_index, sort_rows,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputMode {
    Normal,
    Searching,
    ConfirmKill,
}

/// Number of ticks a status message stays visible before being cleared.
pub(crate) const STATUS_TTL_TICKS: u8 = 6;

/// Collection work the event loop should perform on the app's behalf after
/// handling input. Keeps input handlers free of any `System` handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Followup {
    /// Nothing to do.
    None,
    /// Re-order / re-shape the rows already in `App` (no new samples).
    Reshape,
    /// Re-collect processes from the last samples and rebuild the view.
    Rebuild,
    /// Take fresh samples and rebuild everything.
    Refresh,
}

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
    /// Kill target armed by `k`, consumed by the confirmation step.
    pub(crate) kill_target: Option<Pid>,
    /// (pid, signal) confirmed in the kill prompt; executed by the event loop.
    pub(crate) kill_request: Option<(Pid, SignalKind)>,
    pub(crate) tree_mode: bool,
    /// PIDs whose children are hidden in tree view.
    pub(crate) collapsed: HashSet<Pid>,
    /// Refresh interval in milliseconds, clamped to `cli::MIN/MAX_DELAY_MS`.
    pub(crate) tick_rate_ms: u64,
    /// Latest system-wide stats snapshot (written by `collect`).
    pub(crate) stats: SysStats,
    /// Live details of the selected process (written by `collect`).
    pub(crate) details: Option<ProcDetails>,
    /// Collection work requested by the last input handler.
    pub(crate) followup: Followup,
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
            kill_request: None,
            tree_mode: config.tree,
            collapsed: HashSet::new(),
            tick_rate_ms: config.delay_ms,
            stats: SysStats::default(),
            details: None,
            followup: Followup::None,
        }
    }

    pub(crate) fn set_status(&mut self, msg: String) {
        self.status = msg;
        self.status_ttl = STATUS_TTL_TICKS;
    }

    /// Decays the status message TTL by one tick, clearing the message at
    /// zero. Called once per refresh tick by the event loop.
    pub(crate) fn tick_status(&mut self) {
        if self.status_ttl > 0 {
            self.status_ttl -= 1;
            if self.status_ttl == 0 {
                self.status.clear();
            }
        }
    }

    /// Adjusts the refresh interval by `delta_ms`, clamped to the shared
    /// CLI bounds so both entry paths enforce the same range.
    pub(crate) fn adjust_tick_rate(&mut self, delta_ms: i64) {
        let next = self.tick_rate_ms as i64 + delta_ms;
        self.tick_rate_ms = next.clamp(MIN_DELAY_MS as i64, MAX_DELAY_MS as i64) as u64;
    }

    /// Tree view is only meaningful when no filter is applied: a partial
    /// tree is more confusing than a flat list. Single source of truth for
    /// that rule.
    pub(crate) fn tree_active(&self) -> bool {
        self.tree_mode && self.filter.is_empty()
    }

    pub(crate) fn cycle_sort(&mut self) {
        self.sort = match self.sort {
            SortKey::Cpu => SortKey::Mem,
            SortKey::Mem => SortKey::Pid,
            SortKey::Pid => SortKey::Name,
            SortKey::Name => SortKey::Cpu,
        };
    }

    pub(crate) fn sort_processes_with_selection(
        &mut self,
        preferred_pid: Option<Pid>,
        fallback_index: usize,
    ) {
        sort_rows(&mut self.processes, self.sort, self.desc);
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

    pub(crate) fn toggle_desc(&mut self) {
        self.desc = !self.desc;
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

    /// Confirms the armed kill with the chosen signal. Returns false when
    /// no kill was armed.
    pub(crate) fn arm_kill(&mut self, kind: SignalKind) -> bool {
        match self.kill_target.take() {
            Some(pid) => {
                self.kill_request = Some((pid, kind));
                true
            }
            None => false,
        }
    }

    pub(crate) fn take_kill_request(&mut self) -> Option<(Pid, SignalKind)> {
        self.kill_request.take()
    }

    pub(crate) fn enter_normal(&mut self) {
        self.mode = InputMode::Normal;
    }

    // -- followup (collection work requested by input handlers) ------------

    pub(crate) fn request(&mut self, followup: Followup) {
        self.followup = followup;
    }

    pub(crate) fn take_followup(&mut self) -> Followup {
        std::mem::replace(&mut self.followup, Followup::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::testutil::row;

    fn app() -> App {
        App::new(&Config::default())
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
        app.sort_processes_with_selection(None, 0);

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
        app.sort_processes_with_selection(None, 0);

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
    fn arm_kill_moves_target_to_request() {
        let mut app = app();
        app.kill_target = Some(Pid::from_u32(7));
        assert!(app.arm_kill(SignalKind::Term));
        assert_eq!(app.kill_target, None);
        assert_eq!(
            app.take_kill_request(),
            Some((Pid::from_u32(7), SignalKind::Term))
        );
        // Drained.
        assert_eq!(app.take_kill_request(), None);
    }

    #[test]
    fn arm_kill_without_target_is_false() {
        let mut app = app();
        assert!(!app.arm_kill(SignalKind::Kill));
        assert_eq!(app.take_kill_request(), None);
    }

    #[test]
    fn filter_editing_updates_state() {
        let mut app = app();
        app.enter_search();
        assert!(matches!(app.mode, InputMode::Searching));
        app.push_filter_char('r');
        app.push_filter_char('U');
        app.push_filter_char('S');
        app.push_filter_char('T');
        assert_eq!(app.filter, "rUST");
        app.pop_filter();
        assert_eq!(app.filter, "rUS");
        app.clear_filter();
        assert_eq!(app.filter, "");
    }

    #[test]
    fn tree_active_requires_tree_mode_and_empty_filter() {
        let mut app = app();
        // Default: list mode, no filter -> inactive.
        assert!(!app.tree_active());
        app.tree_mode = true;
        assert!(app.tree_active());
        // A filter flattens the tree.
        app.push_filter_char('x');
        assert!(!app.tree_active());
        app.clear_filter();
        assert!(app.tree_active());
    }

    #[test]
    fn tick_status_clears_message_after_ttl() {
        let mut app = app();
        app.set_status("hello".into());
        assert_eq!(app.status_ttl, STATUS_TTL_TICKS);
        // Decays but stays visible.
        for _ in 0..STATUS_TTL_TICKS - 1 {
            app.tick_status();
            assert!(!app.status.is_empty());
        }
        // Last tick clears the message.
        app.tick_status();
        assert!(app.status.is_empty());
        assert_eq!(app.status_ttl, 0);
        // Idempotent once cleared.
        app.tick_status();
        assert!(app.status.is_empty());
    }

    #[test]
    fn toggle_desc_flips_direction() {
        let mut app = app();
        let before = app.desc;
        app.toggle_desc();
        assert_eq!(app.desc, !before);
        app.toggle_desc();
        assert_eq!(app.desc, before);
    }

    #[test]
    fn adjust_tick_rate_clamps_to_shared_bounds() {
        let mut app = app();
        app.tick_rate_ms = 500;
        app.adjust_tick_rate(-100);
        assert_eq!(app.tick_rate_ms, 400);
        app.adjust_tick_rate(100);
        assert_eq!(app.tick_rate_ms, 500);
        // Clamps at the shared bounds, not below/above.
        app.adjust_tick_rate(-10_000);
        assert_eq!(app.tick_rate_ms, MIN_DELAY_MS);
        app.adjust_tick_rate(10_000);
        assert_eq!(app.tick_rate_ms, MAX_DELAY_MS);
    }

    #[test]
    fn followup_requests_are_taken_once() {
        let mut app = app();
        assert_eq!(app.take_followup(), Followup::None);
        app.request(Followup::Rebuild);
        assert_eq!(app.take_followup(), Followup::Rebuild);
        // Taken: back to None.
        assert_eq!(app.take_followup(), Followup::None);
    }
}
