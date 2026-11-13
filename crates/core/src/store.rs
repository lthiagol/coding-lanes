use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur while reading or writing the task store.
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("failed to read store: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse store: {0}")]
    Parse(#[from] serde_json::Error),
}

/// In-memory collection of tasks, persisted as JSON on disk.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    #[serde(default)]
    next_id: u64,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

impl Store {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    /// Adds a task and returns a copy of it.
    pub fn add(&mut self, title: impl Into<String>) -> Task {
        let task = Task::new(self.next_id, title);
        self.next_id += 1;
        self.tasks.push(task.clone());
        task
    }

    /// Marks a task as done. Returns `false` if no task has that id.
    pub fn set_done(&mut self, id: u64) -> bool {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.done = true;
                true
            }
            None => false,
        }
    }

    /// Loads the store from `path`, returning an empty store if it is missing.
    pub fn load(path: &Path) -> Result<Self, StoreError> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let contents = fs::read_to_string(path)?;
        let mut store: Self = serde_json::from_str(&contents)?;
        if store.next_id == 0 {
            store.next_id = store.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
        }
        Ok(store)
    }

    /// Writes the store to `path`, creating parent directories as needed.
    pub fn save(&self, path: &Path) -> Result<(), StoreError> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Default location of the store: `$XDG_DATA_HOME/coding-lanes/tasks.json`.
    pub fn default_path() -> PathBuf {
        let base = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("coding-lanes").join("tasks.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_tasks_with_increasing_ids() {
        let mut store = Store::new();
        let first = store.add("write parser");
        let second = store.add("write renderer");
        assert_eq!(first.id, 1);
        assert_eq!(second.id, 2);
        assert_eq!(store.tasks.len(), 2);
    }

    #[test]
    fn marks_tasks_as_done() {
        let mut store = Store::new();
        let task = store.add("ship it");
        assert!(store.set_done(task.id));
        assert!(store.tasks[0].done);
        assert!(!store.set_done(999));
    }

    #[test]
    fn round_trips_through_json() {
        let dir = std::env::temp_dir().join("coding-lanes-test-roundtrip");
        let path = dir.join("tasks.json");
        let mut store = Store::new();
        store.add("one");
        store.save(&path).expect("save store");
        let loaded = Store::load(&path).expect("load store");
        assert_eq!(loaded.tasks.len(), 1);
        let _ = fs::remove_dir_all(&dir);
    }
}
