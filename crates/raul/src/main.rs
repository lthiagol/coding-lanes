//! raul — the TUI of coding-lanes.
//!
//! raul reads a work folder and shows it. It writes nothing.

mod app;
mod data;
mod overview;
mod tabs;

use std::io::Stdout;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::{Action, App};
use coding_lanes_core::store::{DATA_DIR, WORK_DIR};
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
///
/// The store is discovered, not configured: the nearest ancestor that holds
/// a `coding-lanes/work` folder is the one. A `coding-lanes` folder without
/// a `work` folder is not a store, so the search walks on. When there is no
/// store at all, the folder that was looked for is returned — raul then says
/// so instead of reading the wrong place.
fn discover() -> PathBuf {
    let Ok(current) = std::env::current_dir() else {
        return PathBuf::new();
    };
    discover_from(&current).unwrap_or_else(|| current.join(DATA_DIR).join(WORK_DIR))
}

/// The nearest ancestor of `start` that holds a work folder.
fn discover_from(start: &Path) -> Option<PathBuf> {
    let mut dir = Some(start.to_path_buf());
    while let Some(current) = dir {
        let candidate = current.join(DATA_DIR).join(WORK_DIR);
        if candidate.is_dir() {
            return Some(candidate);
        }
        dir = current.parent().map(Path::to_path_buf);
    }
    None
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("raul-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("a scratch folder");
        dir
    }

    #[test]
    fn the_discovery_needs_a_work_folder() {
        let dir = scratch_dir("discovery");
        // A coding-lanes folder without a work folder is not a store.
        fs::create_dir_all(dir.join(DATA_DIR)).expect("a data folder");
        assert_eq!(discover_from(&dir), None);
        fs::create_dir_all(dir.join(DATA_DIR).join(WORK_DIR)).expect("a work folder");
        assert_eq!(discover_from(&dir), Some(dir.join(DATA_DIR).join(WORK_DIR)));
    }

    #[test]
    fn the_discovery_walks_up() {
        let dir = scratch_dir("discovery-up");
        fs::create_dir_all(dir.join(DATA_DIR).join(WORK_DIR)).expect("a work folder");
        let deep = dir.join("crates").join("raul");
        fs::create_dir_all(&deep).expect("a nested folder");
        assert_eq!(
            discover_from(&deep),
            Some(dir.join(DATA_DIR).join(WORK_DIR))
        );
    }

    #[test]
    fn without_a_store_the_folder_that_was_looked_for_is_used() {
        let dir = scratch_dir("discovery-none");
        assert_eq!(discover_from(&dir), None, "there is no work folder here");
    }
}
