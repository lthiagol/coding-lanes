use serde::{Deserialize, Serialize};

/// A single unit of work tracked in a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

impl Task {
    /// Creates a pending task with the given identifier.
    pub fn new(id: u64, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            done: false,
        }
    }
}
