use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::task::{Task, slugify};

/// Name of the data folder inside a managed project.
pub const DATA_DIR: &str = "coding-lanes";

/// Name of the work folder inside the data folder.
pub const WORK_DIR: &str = "work";

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("failed to read: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse task file: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("task {0} not found")]
    NotFound(String),
    #[error("no coding-lanes folder found")]
    NoDataFolder,
}

/// The work files of one project.
///
/// Layout:
/// ```text
/// <project>/coding-lanes/work/TSK01-<slug>.json
/// ```
///
/// One folder holds tasks (`TSK*`) and smaller tasks (`ITM*`).
#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    /// Store rooted at a work folder.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Store of a project, by its root.
    pub fn for_project(project: &Path) -> Self {
        Self::new(project.join(DATA_DIR).join(WORK_DIR))
    }

    /// Find the store of the project containing `start`, walking up.
    pub fn discover(start: &Path) -> Result<Self, StoreError> {
        let mut dir = start.to_path_buf();
        loop {
            let candidate = dir.join(DATA_DIR);
            if candidate.is_dir() {
                return Ok(Self::new(candidate.join(WORK_DIR)));
            }
            if !dir.pop() {
                return Err(StoreError::NoDataFolder);
            }
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Path of a task file.
    pub fn task_path(&self, task: &Task) -> PathBuf {
        self.root.join(format!("{}-{}.json", task.id, task.slug))
    }

    /// All tasks, ordered by id.
    pub fn list(&self) -> Result<Vec<Task>, StoreError> {
        let dir = self.root.clone();
        if !dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut tasks: Vec<Task> = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            match load_path(&path) {
                Ok(task) => tasks.push(task),
                // A file that does not parse is skipped, not fatal.
                Err(_) => continue,
            }
        }
        tasks.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(tasks)
    }

    /// One task by id.
    pub fn load(&self, id: &str) -> Result<Task, StoreError> {
        let dir = self.root.clone();
        if !dir.is_dir() {
            return Err(StoreError::NotFound(id.to_string()));
        }
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Some(name) = path.file_name().and_then(|n| n.to_str())
                && name.starts_with(id)
            {
                return load_path(&path);
            }
        }
        Err(StoreError::NotFound(id.to_string()))
    }

    /// Write a task file, atomically. A file the same task had under another
    /// name is removed, so one task is one file.
    pub fn save(&self, task: &Task) -> Result<(), StoreError> {
        fs::create_dir_all(&self.root)?;
        let path = self.task_path(task);
        let json = serde_json::to_string_pretty(task)?;
        let tmp = path.with_extension("json.tmp");
        {
            let mut file = fs::File::create(&tmp)?;
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
            file.sync_all()?;
        }
        fs::rename(&tmp, &path)?;
        self.remove_other_copies(task, &path)?;
        Ok(())
    }

    /// Remove files of the same task left under an older name.
    fn remove_other_copies(&self, task: &Task, keep: &Path) -> Result<(), StoreError> {
        for entry in fs::read_dir(&self.root)? {
            let path = entry?.path();
            if path == keep {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.starts_with(&task.id) && name.ends_with(".json") {
                fs::remove_file(&path)?;
            }
        }
        Ok(())
    }

    /// Create a task in `draft`.
    pub fn create(&self, title: &str) -> Result<Task, StoreError> {
        let task = Task::new(self.next_id()?, title);
        self.save(&task)?;
        Ok(task)
    }

    /// Next free task id, scanning what exists.
    pub fn next_id(&self) -> Result<String, StoreError> {
        let mut highest: u32 = 0;
        for task in self.list()? {
            if let Some(number) = task.id.strip_prefix("TSK")
                && let Ok(number) = number.parse::<u32>()
            {
                highest = highest.max(number);
            }
        }
        Ok(format!("TSK{:02}", highest + 1))
    }
}

fn load_path(path: &Path) -> Result<Task, StoreError> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

/// The file name a task would get.
pub fn file_name(task: &Task) -> String {
    format!("{}-{}.json", task.id, slugify(&task.title))
}
