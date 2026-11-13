use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use coding_lanes_core::{Store, Task};
use std::path::{Path, PathBuf};

/// Task Access and Lane Interaction Terminal App — the CLI tool of coding-lanes.
#[derive(Debug, Parser)]
#[command(name = "talita", version, about)]
struct Talita {
    /// Project root. Defaults to the nearest parent holding a coding-lanes folder.
    #[arg(short, long, global = true)]
    project: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a task in draft.
    Create {
        /// Title of the task.
        title: String,
    },
    /// List tasks.
    List,
    /// Show one task.
    Show {
        /// Task id, e.g. TSK01.
        id: String,
    },
}

fn main() -> Result<()> {
    let talita = Talita::parse();
    match talita.command {
        Command::Create { title } => {
            let store = store_for(talita.project.as_deref(), true)?;
            let task = store.create(&title)?;
            print_task(&task)?;
        }
        Command::List => {
            let store = store_for(talita.project.as_deref(), false)?;
            for task in store.list()? {
                println!("{}  {:<10}  {}", task.id, task.status.as_str(), task.title);
            }
        }
        Command::Show { id } => {
            let store = store_for(talita.project.as_deref(), false)?;
            let task = store.load(&id)?;
            print_task(&task)?;
        }
    }

    Ok(())
}

/// The store of the project. With `create`, a missing folder is created in the
/// current directory; without it, the folder must exist.
fn store_for(project: Option<&Path>, create: bool) -> Result<Store> {
    if let Some(dir) = project {
        return Ok(Store::for_project(dir));
    }
    let cwd = std::env::current_dir().context("reading the current directory")?;
    match Store::discover(&cwd) {
        Ok(store) => Ok(store),
        Err(error) if create => {
            let _ = error;
            Ok(Store::for_project(&cwd))
        }
        Err(error) => Err(error).context("finding the coding-lanes folder"),
    }
}

fn print_task(task: &Task) -> Result<()> {
    let json = serde_json::to_string_pretty(task)?;
    println!("{json}");
    Ok(())
}
