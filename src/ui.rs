//! Terminal UI rendering with ratatui.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
};
use sysinfo::System;

use crate::app::{App, InputMode};
use crate::proc::{ProcRow, SortKey, display_name};

pub(crate) fn ui(
    frame: &mut ratatui::Frame<'_>,
    sys: &System,
    app: &App,
    table_state: &mut TableState,
) {
    let summary = summary_content(sys, app);
    // +2 for the block borders.
    let summary_height = summary.len() as u16 + 2;

    let mut constraints = vec![Constraint::Length(summary_height), Constraint::Min(5)];
    if app.show_details {
        constraints.push(Constraint::Length(7));
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());

    draw_summary(frame, chunks[0], summary);
    draw_process_table(frame, chunks[1], app, table_state);
    if app.show_details {
        draw_process_details(frame, chunks[2], sys, app);
    }
}

/// Width of the ASCII usage bars in characters.
const BAR_WIDTH: usize = 10;
/// At most this many rows of per-core meters (two cores per row).
const MAX_CORE_ROWS: usize = 4;

/// Green below 60%, yellow below 85%, red at or above.
fn load_color(pct: f64) -> Color {
    if pct < 60.0 {
        Color::Green
    } else if pct < 85.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Renders `[██████░░░░]  60.0%` for one meter.
fn usage_bar(pct: f32) -> String {
    let filled = ((pct as f64 / 100.0) * BAR_WIDTH as f64).round() as usize;
    let filled = filled.min(BAR_WIDTH);
    format!(
        "[{}{}] {:5.1}%",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(BAR_WIDTH - filled),
        pct
    )
}

/// `3d 4h 12m` style uptime for the summary line.
fn format_uptime(secs: u64) -> String {
    let (days, rem) = (secs / 86_400, secs % 86_400);
    let (hours, mins) = (rem / 3_600, rem % 3_600 / 60);
    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else {
        format!("{hours}h {mins}m")
    }
}

/// Builds the summary block: a stats line, a mode line with transient
/// status, a key-help line, and per-core CPU meters.
fn summary_content(sys: &System, app: &App) -> Vec<Line<'static>> {
    let cpus = sys.cpus();
    let (cpu_avg, cores) = if cpus.is_empty() {
        (0.0_f64, 0_usize)
    } else {
        let sum: f64 = cpus.iter().map(|c| c.cpu_usage() as f64).sum();
        (sum / cpus.len() as f64, cpus.len())
    };

    let total = sys.total_memory().max(1);
    let used = sys.used_memory().min(total);
    let mem_pct = (used as f64) * 100.0 / (total as f64);
    let used_gb = (used as f64) / (1024.0 * 1024.0 * 1024.0);
    let total_gb = (total as f64) / (1024.0 * 1024.0 * 1024.0);

    let load = System::load_average();
    // Color the 1-minute load relative to the core count so the same
    // thresholds mean the same thing on any machine.
    let load_pct = if cores > 0 {
        load.one * 100.0 / cores as f64
    } else {
        0.0
    };

    let sort = match app.sort {
        SortKey::Cpu => "CPU",
        SortKey::Mem => "MEM",
        SortKey::Pid => "PID",
        SortKey::Name => "NAME",
    };
    let order = if app.desc { "desc" } else { "asc" };
    let mode = match app.mode {
        InputMode::Normal => "NORMAL",
        InputMode::Searching => "SEARCH",
        InputMode::ConfirmKill => "KILL?",
    };
    let view = if app.tree_mode { "TREE" } else { "LIST" };
    let filter_shown: String = if app.filter.is_empty() {
        "\u{2014}".into()
    } else {
        app.filter.clone()
    };
    let paused = if app.paused { "PAUSED" } else { "RUN" };

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                " utop  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("CPU {cpu_avg:.1}% ({cores} cores)  "),
                Style::default().fg(load_color(cpu_avg)),
            ),
            Span::styled(
                format!("Mem {used_gb:.2}/{total_gb:.2} GiB ({mem_pct:.1}%)  "),
                Style::default().fg(load_color(mem_pct)),
            ),
            Span::styled(
                format!(
                    "Load {:.2} {:.2} {:.2}  ",
                    load.one, load.five, load.fifteen
                ),
                Style::default().fg(load_color(load_pct)),
            ),
            Span::raw(format!("up {}", format_uptime(System::uptime()))),
        ]),
        Line::from(vec![
            Span::raw(format!("Sort: {sort} ({order})  ")),
            Span::raw(format!("View: {view}  ")),
            Span::raw(format!("Filter: {filter_shown}  ")),
            Span::raw(format!("Mode: {mode}  ")),
            Span::raw(format!("Paused: {paused}")),
            if app.status.is_empty() {
                Span::raw("")
            } else {
                Span::styled(
                    format!("  {}", app.status),
                    Style::default().add_modifier(Modifier::BOLD),
                )
            },
        ]),
        Line::from(
            "q quit | \u{2191}/\u{2193} move | s sort | r reverse | / search | t tree | p pause | k kill | d details",
        ),
    ];

    // Per-core meters, two cores per row, capped so huge machines stay readable.
    let shown = cpus.len().min(MAX_CORE_ROWS * 2);
    let indexed: Vec<(usize, _)> = cpus.iter().enumerate().take(shown).collect();
    for pair in indexed.chunks(2) {
        let mut spans = Vec::new();
        for (index, cpu) in pair {
            let pct = cpu.cpu_usage();
            spans.push(Span::styled(
                format!(" {:>2} {}  ", index, usage_bar(pct)),
                Style::default().fg(load_color(pct as f64)),
            ));
        }
        lines.push(Line::from(spans));
    }
    if cpus.len() > shown {
        lines.push(Line::from(format!(
            " +{} cores not shown",
            cpus.len() - shown
        )));
    }

    lines
}

fn draw_summary(frame: &mut ratatui::Frame<'_>, area: Rect, lines: Vec<Line<'static>>) {
    let para = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Summary"));
    frame.render_widget(para, area);
}

/// Renders a `ProcRow` as a ratatui `Row`. Per-process CPU% is relative to
/// one core, so it can exceed 100% on multi-core machines; the thresholds
/// are scaled accordingly.
fn proc_row(row: &ProcRow) -> Row<'static> {
    let cpu_color = if row.cpu < 50.0 {
        Color::Green
    } else if row.cpu < 150.0 {
        Color::Yellow
    } else {
        Color::Red
    };
    Row::new(vec![
        Cell::from(row.pid.as_u32().to_string()),
        Cell::from(display_name(row)),
        Cell::from(Span::styled(
            format!("{:>6.1}", row.cpu),
            Style::default().fg(cpu_color),
        )),
        Cell::from(format!("{:>10}", row.mem_mb)),
    ])
}

fn draw_process_table(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    app: &App,
    table_state: &mut TableState,
) {
    let arrow = if app.desc { "\u{2193}" } else { "\u{2191}" };
    let pid_h = if matches!(app.sort, SortKey::Pid) {
        format!("PID {arrow}")
    } else {
        "PID".into()
    };
    let cpu_h = if matches!(app.sort, SortKey::Cpu) {
        format!("CPU% {arrow}")
    } else {
        "CPU%".into()
    };
    let mem_h = if matches!(app.sort, SortKey::Mem) {
        format!("MEM(MiB) {arrow}")
    } else {
        "MEM(MiB)".into()
    };
    let header = Row::new(vec![pid_h, "NAME".to_string(), cpu_h, mem_h]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows = app.processes.iter().map(proc_row);

    let widths = [
        Constraint::Length(8),
        Constraint::Min(20),
        Constraint::Length(8),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Processes ({})", app.processes.len())),
        )
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("\u{25b6} ");

    frame.render_stateful_widget(table, area, table_state);
}

fn draw_process_details(frame: &mut ratatui::Frame<'_>, area: Rect, sys: &System, app: &App) {
    let row = match app.processes.get(app.selected) {
        Some(r) => r,
        None => {
            let para = Paragraph::new("No process selected")
                .block(Block::default().borders(Borders::ALL).title("Details"));
            frame.render_widget(para, area);
            return;
        }
    };
    let pid = row.pid;
    let (name, ppid, status, cpu, mem_mb, exe, cmd) = if let Some(p) = sys.process(pid) {
        let name = p.name().to_string_lossy().into_owned();
        let ppid = p
            .parent()
            .map(|p| p.as_u32().to_string())
            .unwrap_or_else(|| "\u{2014}".to_string());
        let status = format!("{:?}", p.status());
        let cpu = format!("{:.1}", p.cpu_usage());
        let mem_mb = format!("{:.1}", p.memory() as f64 / (1024.0 * 1024.0));
        let exe = p.exe().map(|e| e.display().to_string()).unwrap_or_default();
        let cmd = p
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        (name, ppid, status, cpu, mem_mb, exe, cmd)
    } else {
        (
            row.name.clone(),
            row.ppid
                .map(|p| p.as_u32().to_string())
                .unwrap_or_else(|| "\u{2014}".to_string()),
            "Unknown".to_string(),
            format!("{:.1}", row.cpu),
            format!("{:.1}", row.mem_mb),
            String::new(),
            String::new(),
        )
    };

    let lines = vec![
        Line::from(vec![
            Span::styled(
                format!(" PID: {}  ", row.pid),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("PPID: {ppid}  ")),
            Span::raw(format!("Status: {status}")),
        ]),
        Line::from(format!(" Name: {name}")),
        Line::from(format!(" CPU%: {cpu}  Mem: {mem_mb} MB")),
        Line::from(format!(" Exe: {exe}")),
        Line::from(format!(" Cmd: {cmd}")),
    ];
    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Details"));
    frame.render_widget(para, area);
}
