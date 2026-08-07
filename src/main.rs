//! utop — a lightweight htop clone. Entry point and event loop.

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers, MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::TableState};
use sysinfo::{Signal, System};

mod app;
mod cli;
mod collect;
mod model;
mod ui;

use app::{App, InputMode};
use cli::Config;
use collect::{do_refresh, rebuild_processes};
use ui::ui;

/// Lines moved per mouse-wheel scroll tick.
const SCROLL_STEP: usize = 3;

/// What a key handler wants the event loop to do after processing a key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    /// Keep running the event loop.
    Continue,
    /// Exit the event loop cleanly (user pressed `q`).
    Quit,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = cli::parse();

    install_panic_hook();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &config);

    disable_raw_mode().ok();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .ok();
    terminal.show_cursor().ok();

    res
}

/// Restores the terminal if the app panics so utop never leaves the shell
/// stuck in raw mode behind the alternate screen.
fn install_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        disable_raw_mode().ok();
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).ok();
        original(info);
    }));
}

/// Re-applies ordering after the sort key or direction changes. In tree mode
/// the order is produced by the tree build, so a full rebuild is needed.
fn resort(sys: &System, app: &mut App) {
    if app.tree_mode && app.filter.is_empty() {
        rebuild_processes(sys, app);
    } else {
        app.sort_processes();
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let mut app = App::new(config);

    // Only sample what utop displays (skip disks/network/components/users),
    // twice with a real interval in between, so the first frame already
    // shows valid CPU usage. sysinfo discards samples closer together than
    // MINIMUM_CPU_UPDATE_INTERVAL (200 ms on Linux), so keep at least that
    // gap between the two startup samples.
    let mut sys = System::new();
    do_refresh(&mut sys, &mut app);
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL + Duration::from_millis(20));
    do_refresh(&mut sys, &mut app);

    let mut tick_rate = Duration::from_millis(config.delay_ms);
    let mut last_tick = Instant::now();
    let mut table_state = TableState::default();

    loop {
        if last_tick.elapsed() >= tick_rate {
            if !app.paused {
                do_refresh(&mut sys, &mut app);
            }
            if app.status_ttl > 0 {
                app.status_ttl -= 1;
                if app.status_ttl == 0 {
                    app.status.clear();
                }
            }
            last_tick = Instant::now();
        }

        table_state.select(if app.processes.is_empty() {
            None
        } else {
            Some(app.selected)
        });
        terminal.draw(|f| ui(f, &sys, &app, &mut table_state))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Mouse(mouse) => handle_mouse(&mut app, mouse),
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Release {
                        continue;
                    }
                    // In raw mode Ctrl+C arrives as a key event rather than SIGINT;
                    // honor it as an exit request from any input mode.
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        return Ok(());
                    }
                    let action = match app.mode {
                        InputMode::Normal => {
                            handle_normal_key(&mut app, &mut sys, key, &mut tick_rate)
                        }
                        InputMode::Searching => handle_search_key(&mut app, &sys, key),
                        InputMode::ConfirmKill => handle_kill_key(&mut app, &mut sys, key),
                    };
                    if action == Action::Quit {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }
}

/// Mouse wheel scrolling of the process list.
fn handle_mouse(app: &mut App, mouse: crossterm::event::MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollUp => app.move_up(SCROLL_STEP),
        MouseEventKind::ScrollDown => app.move_down(SCROLL_STEP),
        _ => {}
    }
}

/// Normal-mode input: navigation, view toggles, sort, filter entry, kill.
/// Returns `Action::Quit` if the user pressed `q`, `Action::Continue` otherwise.
fn handle_normal_key(
    app: &mut App,
    sys: &mut System,
    key: KeyEvent,
    tick_rate: &mut Duration,
) -> Action {
    match key.code {
        KeyCode::Char('q') => return Action::Quit,
        KeyCode::Up => app.move_up(1),
        KeyCode::Down => app.move_down(1),
        KeyCode::PageUp => app.move_up(10),
        KeyCode::PageDown => app.move_down(10),
        KeyCode::Home => app.select_first(),
        KeyCode::End => app.select_last(),
        KeyCode::Char('s') => {
            app.cycle_sort();
            resort(sys, app);
        }
        KeyCode::Char('r') => {
            app.desc = !app.desc;
            resort(sys, app);
        }
        KeyCode::Char('/') => app.enter_search(),
        KeyCode::Char('p') => app.toggle_paused(),
        KeyCode::Char('t') => {
            app.toggle_tree();
            rebuild_processes(sys, app);
        }
        KeyCode::Char(' ') => {
            if app.toggle_collapse_selected().is_some() {
                rebuild_processes(sys, app);
            }
        }
        KeyCode::F(5) => do_refresh(sys, app),
        KeyCode::Char('k') => {
            if let Some((pid, name)) = app.begin_kill_selected() {
                app.set_status(format!(
                    "Kill PID {} ({})? y = SIGTERM, K = SIGKILL, Esc = cancel",
                    pid, name
                ));
            }
        }
        KeyCode::Char('-') => {
            let ms = tick_rate.as_millis().saturating_sub(100) as u64;
            *tick_rate = Duration::from_millis(ms.clamp(100, 5000));
            app.set_status(format!("refresh interval: {}ms", tick_rate.as_millis()));
        }
        KeyCode::Char('+') | KeyCode::Char('=') => {
            let ms = (tick_rate.as_millis() as u64).saturating_add(100);
            *tick_rate = Duration::from_millis(ms.clamp(100, 5000));
            app.set_status(format!("refresh interval: {}ms", tick_rate.as_millis()));
        }
        KeyCode::Enter | KeyCode::Char('d') => app.toggle_details(),
        KeyCode::Esc => {
            if !app.filter.is_empty() {
                app.clear_filter();
                rebuild_processes(sys, app);
            }
        }
        _ => {}
    }
    Action::Continue
}

/// Search-mode input: append/delete filter chars, rebuild only on change.
fn handle_search_key(app: &mut App, sys: &System, key: KeyEvent) -> Action {
    let mut changed = true;
    match key.code {
        KeyCode::Enter => {
            app.enter_normal();
            changed = false;
        }
        KeyCode::Esc => {
            app.clear_filter();
            app.enter_normal();
        }
        KeyCode::Backspace => app.pop_filter(),
        KeyCode::Char(c) if !c.is_control() => app.push_filter_char(c),
        _ => changed = false,
    }
    if changed {
        rebuild_processes(sys, app);
    }
    Action::Continue
}

/// Kill-confirmation input: `y` = SIGTERM, `K` = SIGKILL, anything else cancels.
fn handle_kill_key(app: &mut App, sys: &mut System, key: KeyEvent) -> Action {
    let target = app.take_kill_target();
    let signal = match key.code {
        KeyCode::Char('y') => Some(Signal::Term),
        KeyCode::Char('K') => Some(Signal::Kill),
        _ => None,
    };
    match signal {
        Some(signal) => {
            if let Some(pid) = target
                && let Some(process) = sys.process(pid)
            {
                match process.kill_with(signal) {
                    Some(true) => {
                        app.set_status(format!("sent SIG{signal:?} to PID {pid}"));
                        do_refresh(sys, app);
                    }
                    Some(false) => {
                        app.set_status(format!("failed to kill PID {pid}"));
                    }
                    None => {
                        app.set_status(format!("SIG{signal:?} unsupported on this platform"));
                    }
                }
            } else if let Some(pid) = target {
                app.set_status(format!("PID {pid} no longer exists"));
            }
            app.enter_normal();
        }
        None => {
            app.enter_normal();
            app.set_status("kill cancelled".to_string());
        }
    }
    Action::Continue
}
