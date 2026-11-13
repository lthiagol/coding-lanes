//! Reading the work folder. raul reads; it writes nothing.
//!
//! Every view is derived from the files on disk, so the folder is read again
//! on every redraw.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use coding_lanes_core::{Item, ItemKind, Status, Task};

/// A file that could not be read. The rest of the folder still renders.
#[derive(Debug, Clone)]
pub struct ReadError {
    pub file: String,
    pub error: String,
}

/// Why a task waits on a human.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    /// The task is groomed and waits for approval.
    Approve,
    /// The task is in review.
    Review,
    /// The review budget is spent and a human has to decide.
    Escalated,
}

impl Reason {
    pub fn label(&self) -> &'static str {
        match self {
            Reason::Approve => "approve",
            Reason::Review => "review",
            Reason::Escalated => "escalated",
        }
    }
}

/// The work folder, as it is on disk.
#[derive(Debug, Default)]
pub struct Work {
    pub root: PathBuf,
    pub tasks: Vec<Task>,
    pub items: Vec<Item>,
    pub errors: Vec<ReadError>,
    /// The folder is not there.
    pub missing: bool,
}

impl Work {
    /// Read the folder. A file that does not parse is reported, not fatal.
    pub fn load(root: &Path) -> Self {
        let mut work = Work {
            root: root.to_path_buf(),
            ..Work::default()
        };
        if !root.is_dir() {
            work.missing = true;
            return work;
        }
        let Ok(entries) = fs::read_dir(root) else {
            work.missing = true;
            return work;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let contents = match fs::read_to_string(&path) {
                Ok(contents) => contents,
                Err(error) => {
                    work.errors.push(ReadError {
                        file: name.to_string(),
                        error: error.to_string(),
                    });
                    continue;
                }
            };
            if name.starts_with("TSK") {
                match serde_json::from_str::<Task>(&contents) {
                    Ok(task) => work.tasks.push(task),
                    Err(error) => work.errors.push(ReadError {
                        file: name.to_string(),
                        error: error.to_string(),
                    }),
                }
            } else if name.starts_with("ITM") {
                match serde_json::from_str::<Item>(&contents) {
                    Ok(item) => work.items.push(item),
                    Err(error) => work.errors.push(ReadError {
                        file: name.to_string(),
                        error: error.to_string(),
                    }),
                }
            }
        }
        work.tasks.sort_by(|a, b| a.id.cmp(&b.id));
        work.items.sort_by(|a, b| a.id.cmp(&b.id));
        work
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty() && self.items.is_empty()
    }

    /// One task by id.
    pub fn task(&self, id: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    /// Tasks per status, in the order of the lifecycle.
    pub fn status_counts(&self) -> Vec<(Status, usize)> {
        STATUSES
            .iter()
            .map(|status| {
                let count = self
                    .tasks
                    .iter()
                    .filter(|task| task.status == *status)
                    .count();
                (*status, count)
            })
            .collect()
    }

    /// Smaller tasks per kind.
    pub fn kind_counts(&self) -> Vec<(ItemKind, usize)> {
        KINDS
            .iter()
            .map(|kind| {
                let count = self.items.iter().filter(|item| item.kind == *kind).count();
                (*kind, count)
            })
            .collect()
    }

    /// The tasks whose next action is the human's.
    ///
    /// The human lane, minus the tasks carrying an overlay: a task that is
    /// blocked or deferred is not waiting on a human.
    pub fn needs_you(&self) -> Vec<(&Task, Reason)> {
        self.tasks
            .iter()
            .filter(|task| !has_overlay(task))
            .filter_map(|task| {
                let reason = match task.status {
                    Status::Groomed => Reason::Approve,
                    Status::CodeReview => Reason::Review,
                    Status::NeedsHuman => Reason::Escalated,
                    _ => return None,
                };
                Some((task, reason))
            })
            .collect()
    }

    /// How many tasks wait on a human, per reason.
    pub fn needs_you_counts(&self) -> (usize, usize, usize) {
        let mut counts = (0, 0, 0);
        for (_, reason) in self.needs_you() {
            match reason {
                Reason::Approve => counts.0 += 1,
                Reason::Review => counts.1 += 1,
                Reason::Escalated => counts.2 += 1,
            }
        }
        counts
    }

    /// The tasks an agent could claim right now.
    pub fn claimable(&self) -> Vec<&Task> {
        let now = coding_lanes_core::now_rfc3339();
        let mut claimable: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.status == Status::Approved)
            .filter(|task| !has_overlay(task))
            .filter(|task| !live_claim(task, &now))
            .filter(|task| self.dependencies_done(task))
            .collect();
        // Dependency order first: the deepest dependency comes first.
        let mut depths: HashMap<String, u32> = HashMap::new();
        claimable.sort_by_key(|task| (self.depth(task, &mut depths), task.id.clone()));
        claimable
    }

    /// The first claimable task — what happens next.
    pub fn next(&self) -> Option<&Task> {
        self.claimable().into_iter().next()
    }

    /// Whether every dependency of the task is done.
    fn dependencies_done(&self, task: &Task) -> bool {
        task.depends_on.iter().all(|dependency| {
            self.task(dependency)
                .is_some_and(|dependency| dependency.status == Status::Done)
        })
    }

    /// How deep a task sits in the dependency graph.
    fn depth(&self, task: &Task, depths: &mut HashMap<String, u32>) -> u32 {
        let mut visiting = Vec::new();
        self.depth_of(task, depths, &mut visiting)
    }

    fn depth_of(
        &self,
        task: &Task,
        depths: &mut HashMap<String, u32>,
        visiting: &mut Vec<String>,
    ) -> u32 {
        if let Some(depth) = depths.get(&task.id) {
            return *depth;
        }
        // A dependency cycle would loop forever; the graph is acyclic by
        // design, but the reader does not have to trust that.
        if visiting.contains(&task.id) {
            return 0;
        }
        visiting.push(task.id.clone());
        let deepest = task
            .depends_on
            .iter()
            .filter_map(|dependency| self.task(dependency))
            .map(|dependency| self.depth_of(dependency, depths, visiting))
            .max()
            .unwrap_or(0);
        visiting.pop();
        let depth = deepest + 1;
        depths.insert(task.id.clone(), depth);
        depth
    }
}

/// The statuses of the lifecycle, in order.
pub const STATUSES: [Status; 8] = [
    Status::Draft,
    Status::Groomed,
    Status::Approved,
    Status::Executing,
    Status::CodeReview,
    Status::Remediating,
    Status::NeedsHuman,
    Status::Done,
];

/// The kinds of a smaller task.
pub const KINDS: [ItemKind; 3] = [ItemKind::Idea, ItemKind::Feature, ItemKind::Bug];

/// Whether the task carries any overlay.
pub fn has_overlay(task: &Task) -> bool {
    task.overlays.blocked.is_some()
        || task.overlays.deferred.is_some()
        || task.overlays.cancelled.is_some()
}

/// Whether a claim is live — held, and not expired.
pub fn live_claim(task: &Task, now: &str) -> bool {
    task.claim
        .as_ref()
        .is_some_and(|claim| !claim.lease_expired(now))
}

#[cfg(test)]
mod tests {
    use super::*;
    use coding_lanes_core::{Claim, Overlay};

    fn mock_work() -> Work {
        Work::load(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../mock-data/coding-lanes/work"),
        )
    }

    fn work_of(tasks: Vec<Task>) -> Work {
        Work {
            root: PathBuf::new(),
            tasks,
            items: Vec::new(),
            errors: Vec::new(),
            missing: false,
        }
    }

    fn approved(id: &str, depends_on: &[&str]) -> Task {
        let mut task = Task::new(id, format!("task {id}"));
        task.status = Status::Approved;
        task.depends_on = depends_on.iter().map(|id| id.to_string()).collect();
        task
    }

    fn count_of<T: PartialEq>(counts: &[(T, usize)], key: T) -> usize {
        counts
            .iter()
            .find(|(counted, _)| *counted == key)
            .map(|(_, count)| *count)
            .unwrap_or(0)
    }

    #[test]
    fn reads_the_mock_folder() {
        let work = mock_work();
        assert!(!work.missing);
        assert_eq!(work.tasks.len(), 28);
        assert_eq!(work.items.len(), 5);
        assert!(work.errors.is_empty());
    }

    #[test]
    fn counts_the_tasks_by_status() {
        let work = mock_work();
        let counts = work.status_counts();
        assert_eq!(count_of(&counts, Status::Draft), 5);
        assert_eq!(count_of(&counts, Status::Groomed), 3);
        assert_eq!(count_of(&counts, Status::Approved), 3);
        assert_eq!(count_of(&counts, Status::Executing), 3);
        assert_eq!(count_of(&counts, Status::CodeReview), 3);
        assert_eq!(count_of(&counts, Status::Remediating), 3);
        assert_eq!(count_of(&counts, Status::NeedsHuman), 3);
        assert_eq!(count_of(&counts, Status::Done), 5);
    }

    #[test]
    fn counts_the_smaller_tasks_by_kind() {
        let work = mock_work();
        let counts = work.kind_counts();
        assert_eq!(count_of(&counts, ItemKind::Idea), 1);
        assert_eq!(count_of(&counts, ItemKind::Feature), 2);
        assert_eq!(count_of(&counts, ItemKind::Bug), 2);
    }

    #[test]
    fn counts_what_waits_on_a_human() {
        let work = mock_work();
        // Two groomed tasks wait for approval, one of them blocked and so
        // out of the human lane; three wait for review, three are escalated.
        assert_eq!(work.needs_you_counts(), (2, 3, 3));
    }

    #[test]
    fn a_blocked_task_does_not_wait_on_a_human() {
        let mut task = Task::new("TSK01", "blocked");
        task.status = Status::Groomed;
        task.overlays.blocked = Some(Overlay {
            reason: "waiting".into(),
            by: "creator-01".into(),
            at: "2026-11-13T10:00Z".into(),
        });
        let work = work_of(vec![task]);
        assert_eq!(work.needs_you_counts(), (0, 0, 0));
    }

    #[test]
    fn a_claimable_task_is_approved_and_free() {
        let work = work_of(vec![approved("TSK01", &[])]);
        assert_eq!(work.claimable().len(), 1);
        assert_eq!(work.next().map(|task| task.id.as_str()), Some("TSK01"));
    }

    #[test]
    fn a_task_waits_for_its_dependencies() {
        let mut dependency = approved("TSK01", &[]);
        dependency.status = Status::Done;
        let work = work_of(vec![dependency, approved("TSK02", &["TSK01"])]);
        assert_eq!(work.claimable().len(), 1);
        assert_eq!(work.next().map(|task| task.id.as_str()), Some("TSK02"));
    }

    #[test]
    fn an_unfinished_dependency_blocks_the_claim() {
        let dependency = approved("TSK01", &[]);
        let work = work_of(vec![dependency, approved("TSK02", &["TSK01"])]);
        let claimable: Vec<&str> = work
            .claimable()
            .iter()
            .map(|task| task.id.as_str())
            .collect();
        assert_eq!(claimable, vec!["TSK01"]);
        assert_eq!(work.next().map(|task| task.id.as_str()), Some("TSK01"));
    }

    #[test]
    fn a_claim_holds_the_task() {
        let mut task = approved("TSK01", &[]);
        task.claim = Some(Claim {
            by: "runner-01".into(),
            session: Some("session-runner-01".into()),
            at: "2026-11-13T10:00Z".into(),
            lease_expires_at: "2099-01-01T00:00:00Z".into(),
        });
        let work = work_of(vec![task]);
        assert!(work.claimable().is_empty());
    }

    #[test]
    fn an_overlay_holds_the_task() {
        let mut task = approved("TSK01", &[]);
        task.overlays.deferred = Some(Overlay {
            reason: "later".into(),
            by: "creator-01".into(),
            at: "2026-11-13T10:00Z".into(),
        });
        let work = work_of(vec![task]);
        assert!(work.claimable().is_empty());
    }

    #[test]
    fn the_next_task_comes_first_in_dependency_order() {
        let mut first = approved("TSK01", &[]);
        first.status = Status::Done;
        let second = approved("TSK02", &["TSK01"]);
        let third = approved("TSK03", &["TSK01"]);
        let work = work_of(vec![first, second, third]);
        let next = work.next().expect("a claimable task");
        assert_eq!(next.id, "TSK02");
    }

    #[test]
    fn the_mock_data_has_no_claimable_task() {
        // Every approved task of the mock data waits on a task that is not
        // done, so nothing is claimable there.
        let work = mock_work();
        assert!(work.claimable().is_empty());
        assert!(work.next().is_none());
    }

    #[test]
    fn a_missing_folder_is_not_an_error() {
        let work = Work::load(Path::new("/nowhere/coding-lanes/work"));
        assert!(work.missing);
        assert!(work.is_empty());
    }
}
