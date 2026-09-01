//! Integration tests through the crate's public API. These link against
//! the `utop` library the way an external consumer would — the unit tests
//! inside each module cover the internals.

use std::collections::HashSet;

use utop::cli::{Action, parse_args};
use utop::model::{
    Pid, ProcRow, SortKey, build_tree, filter_processes, resolve_selected_index, selected_pid,
};

fn row(pid: u32, name: &str, ppid: Option<u32>) -> ProcRow {
    ProcRow {
        pid: Pid::from_u32(pid),
        name: name.to_string(),
        name_lc: name.to_lowercase(),
        cpu: 0.0,
        mem_mb: 10,
        ppid: ppid.map(Pid::from_u32),
        depth: 0,
        has_children: false,
        collapsed: false,
    }
}

fn run_config(args: &[&str]) -> utop::cli::Config {
    match parse_args(args.iter().map(|s| s.to_string()).collect::<Vec<_>>()) {
        Ok(Action::Run(config)) => config,
        other => panic!("expected Run, got {other:?}"),
    }
}

#[test]
fn parse_empty_args_yields_default_run_config() {
    let config = run_config(&[]);
    assert_eq!(config.sort, SortKey::Cpu);
    assert!(config.desc);
    assert_eq!(config.delay_ms, 500);
    assert_eq!(config.filter, "");
    assert!(!config.tree);
}

#[test]
fn parse_rejects_unknown_arguments() {
    let result = parse_args(vec!["--bogus".to_string()]);
    assert!(result.is_err());
}

#[test]
fn filter_and_tree_compose_through_the_public_api() {
    let rows = vec![
        row(1, "systemd", None),
        row(2, "bash", Some(1)),
        row(3, "vim", Some(2)),
        row(10, "python", None),
    ];

    // Filtering matches on the lowercase name.
    let filtered = filter_processes(rows.clone(), "py");
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].pid.as_u32(), 10);

    // Tree building nests children and exposes shape via depth/has_children.
    let tree = build_tree(rows, SortKey::Pid, false, &HashSet::new());
    assert_eq!(
        tree.iter().map(|r| r.pid.as_u32()).collect::<Vec<_>>(),
        vec![1, 2, 3, 10]
    );
    assert_eq!(
        tree.iter().map(|r| r.depth).collect::<Vec<_>>(),
        vec![0, 1, 2, 0]
    );
}

#[test]
fn selection_resolution_survives_reordering() {
    let rows = vec![row(1, "a", None), row(2, "b", None), row(3, "c", None)];

    // The selected PID is found at its new position.
    let before = selected_pid(&rows, 2);
    assert_eq!(before, Some(Pid::from_u32(3)));
    assert_eq!(resolve_selected_index(&rows, before, 2), 2);

    // A vanished PID falls back to the clamped index.
    assert_eq!(resolve_selected_index(&rows, Some(Pid::from_u32(99)), 2), 2);
}
