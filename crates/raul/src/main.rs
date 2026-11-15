//! raul — the TUI of coding-lanes.
//!
//! raul reads a work folder and shows it. It writes nothing.

mod app;
mod data;
mod overview;
mod path;
mod tabs;
mod theme;

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
use theme::{Preset, Theme};

const USAGE: &str = "usage: raul [--theme dark|terminal] [work-folder]";

fn main() -> Result<()> {
    let options = parse_args(std::env::args().skip(1))?;

    let mut terminal = setup()?;
    let result = run(&mut terminal, options);
    restore(&mut terminal)?;
    result
}

/// What the command line asks for.
#[derive(Debug)]
struct Options {
    /// The work folder to read.
    root: PathBuf,
    /// The colours of the window.
    theme: Theme,
}

/// The work folder to read and the colours to paint it with.
///
/// The folder is given on the command line, or discovered from the current
/// directory: the store is discovered, not configured. The theme is chosen
/// with a flag.
fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Options> {
    let mut root: Option<PathBuf> = None;
    let mut preset = Preset::Dark;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            "--theme" => {
                let Some(name) = args.next() else {
                    bail!("--theme needs a name\n{USAGE}");
                };
                preset = preset_of(&name)?;
            }
            other if other.starts_with("--theme=") => {
                preset = preset_of(&other["--theme=".len()..])?;
            }
            other if other.starts_with('-') => bail!("unknown option {other}\n{USAGE}"),
            other => root = Some(PathBuf::from(other)),
        }
    }
    Ok(Options {
        root: root.unwrap_or_else(discover),
        theme: Theme::preset(preset),
    })
}

/// The preset of a name.
fn preset_of(name: &str) -> Result<Preset> {
    Preset::parse(name).ok_or_else(|| {
        let names: Vec<&str> = Preset::ALL.iter().map(|preset| preset.name()).collect();
        anyhow::anyhow!("unknown theme {name}: the themes are {}", names.join(", "))
    })
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
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, options: Options) -> Result<()> {
    let mut app = App::new(options.root, options.theme);
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

    #[test]
    fn the_theme_is_the_dark_one_by_default() {
        let options = parse_args(std::iter::empty()).expect("the options");
        assert_eq!(options.theme, Theme::preset(Preset::Dark));
    }

    #[test]
    fn the_theme_is_chosen_with_a_flag() {
        for args in [
            vec!["--theme".to_string(), "terminal".to_string()],
            vec!["--theme=terminal".to_string()],
        ] {
            let options = parse_args(args.into_iter()).expect("the options");
            assert_eq!(options.theme, Theme::preset(Preset::Terminal));
        }
    }

    #[test]
    fn an_unknown_theme_is_refused() {
        let error = parse_args(["--theme".to_string(), "light".to_string()].into_iter())
            .expect_err("light is not a theme yet");
        assert!(error.to_string().contains("light"), "{error}");
        let error =
            parse_args(["--theme".to_string()].into_iter()).expect_err("a theme needs a name");
        assert!(error.to_string().contains("--theme"), "{error}");
    }
}
