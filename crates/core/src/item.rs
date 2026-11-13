use serde::{Deserialize, Serialize};

use crate::task::{AcceptanceCriterion, Scope, Task, TaskKind, slugify};

/// Schema version written into every item file.
pub const ITEM_SCHEMA_VERSION: u32 = 1;

/// The work that is not a task yet: an idea, a bug or a small feature.
///
/// See `docs/tasks/smaller-tasks.md`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    #[serde(default = "default_item_schema_version")]
    pub schema_version: u32,
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub slug: String,
    /// What the item is. The kind is the only truth about it.
    #[serde(default)]
    pub kind: ItemKind,
    /// Whether the item is live, dismissed, or became a task.
    #[serde(default)]
    pub status: ItemStatus,
    /// Why the item is worth doing.
    #[serde(default)]
    pub body: String,
    /// The work, when it is known.
    #[serde(default)]
    pub scope: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
    /// The task this item became. Promotion is one-way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_to: Option<String>,
    #[serde(default)]
    pub dismissed_reason: String,
}

fn default_item_schema_version() -> u32 {
    ITEM_SCHEMA_VERSION
}

impl Item {
    /// A new item. The kind defaults to `idea`.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            schema_version: ITEM_SCHEMA_VERSION,
            slug: slugify(&title),
            id: id.into(),
            title,
            kind: ItemKind::Idea,
            status: ItemStatus::Open,
            body: String::new(),
            scope: Vec::new(),
            tags: Vec::new(),
            created: crate::now_rfc3339(),
            updated: crate::now_rfc3339(),
            promoted_to: None,
            dismissed_reason: String::new(),
        }
    }

    /// A small task: the work is known and it is ready to run.
    pub fn is_small_task(&self) -> bool {
        self.kind.is_small_task()
    }

    /// Shape the item into a small task. The id does not change.
    pub fn shape(&mut self, kind: ItemKind, scope: Vec<String>) -> Result<(), ItemError> {
        if self.status != ItemStatus::Open {
            return Err(ItemError::Settled(self.id.clone()));
        }
        if kind == ItemKind::Idea {
            return Err(ItemError::UnknownKind);
        }
        self.kind = kind;
        if !scope.is_empty() {
            self.scope = scope;
        }
        self.updated = crate::now_rfc3339();
        Ok(())
    }

    /// Dismiss the item, keeping the reason.
    pub fn dismiss(&mut self, reason: impl Into<String>) -> Result<(), ItemError> {
        if self.status != ItemStatus::Open {
            return Err(ItemError::Settled(self.id.clone()));
        }
        self.status = ItemStatus::Dismissed;
        self.dismissed_reason = reason.into();
        self.updated = crate::now_rfc3339();
        Ok(())
    }

    /// Promote the item into a task.
    ///
    /// An idea may not be promoted: it carries no acceptance criteria, and a
    /// task without them can never be signed off. The refusal names the
    /// remedy.
    pub fn promote(&mut self, task_id: impl Into<String>) -> Result<Task, ItemError> {
        if self.status != ItemStatus::Open {
            return Err(ItemError::Settled(self.id.clone()));
        }
        if !self.is_small_task() {
            return Err(ItemError::NotPromotable(self.kind));
        }

        let task_id = task_id.into();
        let mut task = Task::new(&task_id, &self.title);
        task.kind = match self.kind {
            ItemKind::Feature => TaskKind::Feature,
            ItemKind::Bug => TaskKind::Bug,
            ItemKind::Idea => TaskKind::Task,
        };
        task.promoted_from = Some(self.id.clone());
        task.intent.outcome = self.title.clone();
        task.intent.problem = self.body.clone();
        task.scope = Scope {
            in_scope: self.scope.clone(),
            out_of_scope: Vec::new(),
        };
        task.acceptance_criteria = vec![AcceptanceCriterion {
            id: "ACCEPT-01".to_string(),
            description: self.title.clone(),
            ..AcceptanceCriterion::default()
        }];

        // The item is written first by the store, so the trail survives a crash.
        self.status = ItemStatus::Promoted;
        self.promoted_to = Some(task_id);
        self.updated = crate::now_rfc3339();
        Ok(task)
    }
}

/// What an item is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ItemKind {
    /// A hunch — worth knowing, not yet work.
    #[default]
    Idea,
    /// A small task: the work is known and it is ready to run.
    Feature,
    /// A small task that is a defect.
    Bug,
}

impl ItemKind {
    /// `feature` and `bug` are small tasks.
    pub fn is_small_task(&self) -> bool {
        matches!(self, ItemKind::Feature | ItemKind::Bug)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ItemKind::Idea => "idea",
            ItemKind::Feature => "feature",
            ItemKind::Bug => "bug",
        }
    }
}

/// Whether an item is live, dismissed, or became a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ItemStatus {
    #[default]
    Open,
    Dismissed,
    Promoted,
}

impl ItemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemStatus::Open => "open",
            ItemStatus::Dismissed => "dismissed",
            ItemStatus::Promoted => "promoted",
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ItemError {
    #[error("item {0} is settled")]
    Settled(String),
    #[error("unknown kind")]
    UnknownKind,
    #[error(
        "{0:?} carries no acceptance criteria, and a task without them can never be signed off — shape it into a small task first"
    )]
    NotPromotable(ItemKind),
}
