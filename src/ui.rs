//! Terminal UI rendering with ratatui.
//!
//! Pure rendering: reads only the snapshots stored on `App` (process rows,
//! `SysStats`, `ProcDetails`) and never the live system. If you can build
//! an `App`, you can render and test every function in this module.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Wrap},
};

use crate::app::{App, InputMode};
use crate::model::{ProcRow, SortKey, SysStats, display_name};

pub(crate) fn ui(frame: &mut ratatui::Frame<'_>, app: &App, table_state: &mut TableState) {
    let summary = summary_content(app);
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
        draw_process_details(frame, chunks[2], app);
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
fn summary_content(app: &App) -> Vec<Line<'static>> {
    let mut lines = vec![stats_line(&app.stats), status_line(app), key_help_line()];
    lines.extend(per_core_meters(&app.stats.core_usages));
    lines
}

/// First summary line: title, CPU average, memory, load average, uptime.
fn stats_line(stats: &SysStats) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            " utop  ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(
                "CPU {:.1}% ({} cores)  ",
                stats.cpu_avg,
                stats.core_usages.len()
            ),
            Style::default().fg(load_color(stats.cpu_avg)),
        ),
        Span::styled(
            format!(
                "Mem {:.2}/{:.2} GiB ({:.1}%)  ",
                stats.mem_used_gib, stats.mem_total_gib, stats.mem_pct
            ),
            Style::default().fg(load_color(stats.mem_pct)),
        ),
        Span::styled(
            format!(
                "Load {:.2} {:.2} {:.2}  ",
                stats.load_one, stats.load_five, stats.load_fifteen
            ),
            Style::default().fg(load_color(stats.load_pct)),
        ),
        Span::raw(format!("up {}", format_uptime(stats.uptime_secs))),
    ])
}

/// Second summary line: sort/view/filter/mode/paused indicators plus the
/// transient status message (if any).
fn status_line(app: &App) -> Line<'static> {
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

    let mut spans = vec![
        Span::raw(format!("Sort: {sort} ({order})  ")),
        Span::raw(format!("View: {view}  ")),
        Span::raw(format!("Filter: {filter_shown}  ")),
        Span::raw(format!("Mode: {mode}  ")),
        Span::raw(format!("Paused: {paused}")),
    ];
    if !app.status.is_empty() {
        spans.push(Span::styled(
            format!("  {}", app.status),
            Style::default().add_modifier(Modifier::BOLD),
        ));
    }
    Line::from(spans)
}

/// Third summary line: a compact reminder of the key bindings.
fn key_help_line() -> Line<'static> {
    Line::from(
        "q quit | \u{2191}/\u{2193} move | s sort | r reverse | / search | t tree | p pause | k kill | d details",
    )
}

/// Per-core CPU meters, two cores per row, capped so huge machines stay
/// readable. Returns zero or more lines plus a trailing "N cores not shown"
/// line if any were elided.
fn per_core_meters(core_usages: &[f32]) -> Vec<Line<'static>> {
    let shown = core_usages.len().min(MAX_CORE_ROWS * 2);
    let indexed: Vec<(usize, f32)> = core_usages
        .iter()
        .enumerate()
        .take(shown)
        .map(|(index, usage)| (index, *usage))
        .collect();

    let mut lines = Vec::new();
    for pair in indexed.chunks(2) {
        let mut spans = Vec::new();
        for (index, pct) in pair {
            spans.push(Span::styled(
                format!(" {:>2} {}  ", index, usage_bar(*pct)),
                Style::default().fg(load_color(*pct as f64)),
            ));
        }
        lines.push(Line::from(spans));
    }
    if core_usages.len() > shown {
        lines.push(Line::from(format!(
            " +{} cores not shown",
            core_usages.len() - shown
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
    // Order must match `proc_row` and `widths`: PID | NAME | CPU% | MEM(MiB).
    let header_cells: Vec<String> = [
        (SortKey::Pid, "PID"),
        (SortKey::Name, "NAME"),
        (SortKey::Cpu, "CPU%"),
        (SortKey::Mem, "MEM(MiB)"),
    ]
    .into_iter()
    .map(|(key, label)| {
        if app.sort == key {
            format!("{label} {arrow}")
        } else {
            label.to_string()
        }
    })
    .collect();
    let header = Row::new(header_cells).style(
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

fn draw_process_details(frame: &mut ratatui::Frame<'_>, area: Rect, app: &App) {
    let Some(row) = app.processes.get(app.selected) else {
        let para = Paragraph::new("No process selected")
            .block(Block::default().borders(Borders::ALL).title("Details"));
        frame.render_widget(para, area);
        return;
    };

    // Live snapshot of the selected process; falls back to the row data
    // once the process is gone.
    let (name, ppid, status, cpu, mem_mib, exe, cmd) = match &app.details {
        Some(d) => (
            d.name.clone(),
            d.ppid
                .map(|p| p.as_u32().to_string())
                .unwrap_or_else(|| "\u{2014}".to_string()),
            d.status.clone(),
            format!("{:.1}", d.cpu),
            format!("{:.1}", d.mem_mib),
            d.exe.clone(),
            d.cmd.clone(),
        ),
        None => (
            row.name.clone(),
            row.ppid
                .map(|p| p.as_u32().to_string())
                .unwrap_or_else(|| "\u{2014}".to_string()),
            "Unknown".to_string(),
            format!("{:.1}", row.cpu),
            format!("{:.1}", row.mem_mb as f64),
            String::new(),
            String::new(),
        ),
    };

    let lines = vec![
        Line::from(vec![
            Span::styled(
                format!(" PID: {}", row.pid.as_u32()),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("PPID: {ppid}  ")),
            Span::raw(format!("Status: {status}")),
        ]),
        Line::from(format!(" Name: {name}")),
        Line::from(format!(" CPU%: {cpu}  Mem: {mem_mib} MiB")),
        Line::from(format!(" Exe: {exe}")),
        Line::from(format!(" Cmd: {cmd}")),
    ];
    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Details"));
    frame.render_widget(para, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_bar_clamps_and_fills() {
        assert_eq!(
            usage_bar(0.0),
            "[\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}]   0.0%"
        );
        assert_eq!(
            usage_bar(100.0),
            "[\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}] 100.0%"
        );
        // Over 100% (per-core CPU can exceed 100): clamps at full.
        assert!(usage_bar(150.0).starts_with(
            "[\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}]"
        ));
        assert!(usage_bar(50.0).starts_with(
            "[\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2591}\u{2591}\u{2591}\u{2591}\u{2591}]"
        ));
    }

    #[test]
    fn format_uptime_variants() {
        assert_eq!(format_uptime(0), "0h 0m");
        assert_eq!(format_uptime(3_600), "1h 0m");
        assert_eq!(format_uptime(3_660), "1h 1m");
        assert_eq!(format_uptime(86_400), "1d 0h 0m");
        assert_eq!(format_uptime(2 * 86_400 + 3 * 3_600 + 4 * 60), "2d 3h 4m");
    }

    #[test]
    fn load_color_thresholds() {
        assert_eq!(load_color(0.0), Color::Green);
        assert_eq!(load_color(59.9), Color::Green);
        assert_eq!(load_color(60.0), Color::Yellow);
        assert_eq!(load_color(84.9), Color::Yellow);
        assert_eq!(load_color(85.0), Color::Red);
        assert_eq!(load_color(200.0), Color::Red);
    }

    #[test]
    fn per_core_meters_caps_and_reports_elision() {
        // Zero cores: no lines at all.
        assert!(per_core_meters(&[]).is_empty());
        // 8 cores fit exactly in 4 rows of 2: no elision note.
        let lines = per_core_meters(&[0.0; 8]);
        assert_eq!(lines.len(), 4);
        // 9 cores: one extra "not shown" line.
        let lines = per_core_meters(&[0.0; 9]);
        assert_eq!(lines.len(), 5);
        let last = format!("{:?}", lines.last().unwrap());
        assert!(last.contains("not shown"));
    }
}
