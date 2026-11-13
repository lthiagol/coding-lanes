use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use coding_lanes_core::Store;
use std::path::{Path, PathBuf};

/// Manage coding lanes and the tasks inside them.
#[derive(Debug, Parser)]
#[command(name = "talita", version, about)]
struct Talita {
    /// Path of the task store (defaults to the XDG data directory).
    #[arg(short, long, global = true)]
    store: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Add a task to the current lane.
    Add {
        /// Title of the task.
        title: String,
    },
    /// List all tasks.
    List,
    /// Mark a task as done.
    Done {
        /// Identifier of the task.
        id: u64,
    },
}

fn main() -> Result<()> {
    let talita = Talita::parse();
    let path = talita.store.unwrap_or_else(Store::default_path);

    let mut store = Store::load(&path).context("loading task store")?;

    match talita.command {
        Command::Add { title } => {
            let task = store.add(title);
            persist(&store, &path)?;
            println!("added task #{}: {}", task.id, task.title);
        }
        Command::List => {
            if store.tasks.is_empty() {
                println!("no tasks yet");
            }
            for task in &store.tasks {
                let marker = if task.done { "x" } else { " " };
                println!("[{marker}] #{:<4} {}", task.id, task.title);
            }
        }
        Command::Done { id } => {
            if store.set_done(id) {
                persist(&store, &path)?;
                println!("task #{id} done");
            } else {
                println!("no task with id {id}");
            }
        }
    }

    Ok(())
}

fn persist(store: &Store, path: &Path) -> Result<()> {
    store
        .save(path)
        .map_err(anyhow::Error::from)
        .with_context(|| format!("saving task store to {}", path.display()))
}
