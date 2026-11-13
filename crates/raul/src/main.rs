//! raul — the TUI of coding-lanes.
//!
//! raul reads a work folder and shows it. It writes nothing.

mod app;
mod data;
mod overview;
mod tabs;

use std::io::Stdout;
use std::path::PathBuf;

use anyhow::{Result, bail};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::{Action, App};
use coding_lanes_core::Store;
use data::Work;

const USAGE: &str = "usage: raul [work-folder]";

fn main() -> Result<()> {
    let root = parse_args(std::env::args().skip(1))?;

    let mut terminal = setup()?;
    let result = run(&mut terminal, root);
    restore(&mut terminal)?;
    result
}

/// The work folder to read.
///
/// Given on the command line, or discovered from the current directory: the
/// store is discovered, not configured.
fn parse_args(args: impl Iterator<Item = String>) -> Result<PathBuf> {
    let mut root: Option<PathBuf> = None;
    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            other if other.starts_with('-') => bail!("unknown option {other}\n{USAGE}"),
            other => root = Some(PathBuf::from(other)),
        }
    }
    Ok(root.unwrap_or_else(discover))
}

/// The work folder of the project in the current directory.
fn discover() -> PathBuf {
    let Ok(current) = std::env::current_dir() else {
        return PathBuf::new();
    };
    match Store::discover(&current) {
        Ok(store) => store.root().to_path_buf(),
        Err(_) => current,
    }
}

/// The window loop. The folder is read again on every redraw.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, root: PathBuf) -> Result<()> {
    let mut app = App::new(root);
    loop {
        let work = Work::load(&app.root);
        terminal.draw(|frame| app.draw(frame, &work))?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && app.on_key(key) == Action::Quit
        {
            break;
        }
    }
    Ok(())
}

fn setup() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
