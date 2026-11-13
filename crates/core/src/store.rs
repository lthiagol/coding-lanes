use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::item::Item;
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
    #[error("{0}")]
    Item(String),
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
            if !file_starts_with(&path, "TSK") {
                continue;
            }
            match load_task_path(&path) {
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
                && file_starts_with(&path, "TSK")
            {
                return load_task_path(&path);
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

    /// All smaller tasks, ordered by id.
    pub fn list_items(&self) -> Result<Vec<Item>, StoreError> {
        let dir = self.root.clone();
        if !dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut items: Vec<Item> = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if !file_starts_with(&path, "ITM") {
                continue;
            }
            match load_item_path(&path) {
                Ok(item) => items.push(item),
                Err(_) => continue,
            }
        }
        items.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(items)
    }

    /// One smaller task by id.
    pub fn load_item(&self, id: &str) -> Result<Item, StoreError> {
        let dir = self.root.clone();
        if !dir.is_dir() {
            return Err(StoreError::NotFound(id.to_string()));
        }
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if file_starts_with(&path, "ITM")
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
                && name.starts_with(id)
            {
                return load_item_path(&path);
            }
        }
        Err(StoreError::NotFound(id.to_string()))
    }

    /// Write a smaller task file, atomically.
    pub fn save_item(&self, item: &Item) -> Result<(), StoreError> {
        fs::create_dir_all(&self.root)?;
        let path = self.root.join(format!("{}-{}.json", item.id, item.slug));
        let json = serde_json::to_string_pretty(item)?;
        let tmp = path.with_extension("json.tmp");
        {
            let mut file = fs::File::create(&tmp)?;
            file.write_all(json.as_bytes())?;
            file.write_all(b"\n")?;
            file.sync_all()?;
        }
        fs::rename(&tmp, &path)?;
        Ok(())
    }

    /// Create a smaller task. The kind starts as an idea.
    pub fn create_item(&self, title: &str) -> Result<Item, StoreError> {
        let item = Item::new(self.next_item_id()?, title);
        self.save_item(&item)?;
        Ok(item)
    }

    /// Next free smaller-task id, scanning what exists.
    pub fn next_item_id(&self) -> Result<String, StoreError> {
        let mut highest: u32 = 0;
        for item in self.list_items()? {
            if let Some(number) = item.id.strip_prefix("ITM")
                && let Ok(number) = number.parse::<u32>()
            {
                highest = highest.max(number);
            }
        }
        Ok(format!("ITM{:02}", highest + 1))
    }

    /// Promote a smaller task into a task.
    ///
    /// The item is written first, so a crash cannot produce two tasks from
    /// one item.
    pub fn promote_item(&self, id: &str) -> Result<Task, StoreError> {
        let mut item = self.load_item(id)?;
        let task_id = self.next_id()?;
        let task = item
            .promote(task_id)
            .map_err(|error| StoreError::Item(error.to_string()))?;
        self.save_item(&item)?;
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

fn load_task_path(path: &Path) -> Result<Task, StoreError> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

fn load_item_path(path: &Path) -> Result<Item, StoreError> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Whether a file's name starts with an id prefix.
fn file_starts_with(path: &Path, prefix: &str) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| name.starts_with(prefix))
}

/// The file name a task would get.
pub fn file_name(task: &Task) -> String {
    format!("{}-{}.json", task.id, slugify(&task.title))
}
