use coding_lanes_core::{Status, Store, Task};
use std::collections::HashSet;
use std::path::PathBuf;

fn mock_store() -> Store {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("mock-data/coding-lanes/work");
    Store::new(root)
}

fn tasks() -> Vec<Task> {
    mock_store().list().expect("listing the mock tasks")
}

fn by_status(status: Status) -> Vec<Task> {
    tasks().into_iter().filter(|t| t.status == status).collect()
}

#[test]
fn every_fixture_loads() {
    let tasks = tasks();
    assert_eq!(tasks.len(), 28);

    let ids: HashSet<&str> = tasks.iter().map(|t| t.id.as_str()).collect();
    assert_eq!(ids.len(), tasks.len(), "task ids must be unique");
}

#[test]
fn every_phase_is_covered() {
    for status in [
        Status::Draft,
        Status::Groomed,
        Status::Approved,
        Status::Executing,
        Status::CodeReview,
        Status::Remediating,
        Status::NeedsHuman,
        Status::Done,
    ] {
        assert!(
            !by_status(status).is_empty(),
            "no fixture for status {status:?}"
        );
    }
}

#[test]
fn phase_counts_are_three_and_five_done() {
    assert_eq!(by_status(Status::Groomed).len(), 3);
    assert_eq!(by_status(Status::Approved).len(), 3);
    assert_eq!(by_status(Status::Executing).len(), 3);
    assert_eq!(by_status(Status::CodeReview).len(), 3);
    assert_eq!(by_status(Status::Remediating).len(), 3);
    assert_eq!(by_status(Status::NeedsHuman).len(), 3);
    assert_eq!(by_status(Status::Done).len(), 5);
    assert_eq!(
        by_status(Status::Draft).len(),
        5,
        "three drafts, one child, one promoted bug"
    );
}

#[test]
fn fixtures_keep_their_gates() {
    for task in by_status(Status::Approved) {
        assert!(
            task.criteria_covered(),
            "{} has uncovered criteria",
            task.id
        );
        assert!(
            task.steps_have_done_when(),
            "{} has a step without done_when",
            task.id
        );
    }

    for task in by_status(Status::Executing) {
        assert!(task.claim.is_some(), "{} is not claimed", task.id);
        assert!(
            task.acceptance_criteria
                .iter()
                .any(|c| !c.evidence.is_empty()),
            "{} has no evidence",
            task.id
        );
    }

    for task in by_status(Status::CodeReview) {
        assert!(task.self_rounds() > 0, "{} has no self round", task.id);
        assert!(
            task.steps
                .iter()
                .all(|s| s.status == coding_lanes_core::StepStatus::Done),
            "{} has unsettled steps",
            task.id
        );
    }

    for task in by_status(Status::Remediating) {
        assert!(task.open_findings() > 0, "{} has no open finding", task.id);
        assert_eq!(
            task.review.verdict,
            Some(coding_lanes_core::Verdict::ChangesNeeded)
        );
    }

    for task in by_status(Status::NeedsHuman) {
        assert_eq!(
            task.external_rounds() as u32,
            task.review.max_rounds,
            "{} has not exhausted the round budget",
            task.id
        );
    }

    for task in by_status(Status::Done) {
        assert_eq!(task.open_findings(), 0, "{} has open findings", task.id);
        assert_eq!(task.review.verdict, Some(coding_lanes_core::Verdict::Ok));
    }
}

#[test]
fn dependencies_point_to_existing_tasks_and_never_forward() {
    let tasks = tasks();
    let ids: HashSet<&str> = tasks.iter().map(|t| t.id.as_str()).collect();

    for task in &tasks {
        for dependency in &task.depends_on {
            assert!(
                ids.contains(dependency.as_str()),
                "{} depends on unknown task {dependency}",
                task.id
            );
            assert!(
                dependency < &task.id,
                "{} depends on a later task {dependency}",
                task.id
            );
        }
    }
}

#[test]
fn every_phase_after_the_first_depends_on_something() {
    for status in [
        Status::Approved,
        Status::Executing,
        Status::CodeReview,
        Status::Remediating,
        Status::NeedsHuman,
        Status::Done,
    ] {
        for task in by_status(status) {
            assert!(!task.depends_on.is_empty(), "{} has no dependency", task.id);
        }
    }
}

#[test]
fn child_task_points_at_its_parent() {
    let tasks = tasks();
    let child = tasks
        .iter()
        .find(|t| t.parent.is_some())
        .expect("a child task");
    let parent = tasks
        .iter()
        .find(|t| t.id == child.parent.clone().unwrap())
        .expect("the parent task");
    assert!(parent.children.contains(&child.id));
}

#[test]
fn deferred_criterion_names_its_follow_up() {
    let tasks = tasks();
    let deferred: Vec<&Task> = tasks
        .iter()
        .filter(|t| t.acceptance_criteria.iter().any(|c| c.deferral.is_some()))
        .collect();
    assert!(!deferred.is_empty(), "no fixture with a deferred criterion");

    for task in deferred {
        for criterion in &task.acceptance_criteria {
            if let Some(deferral) = &criterion.deferral {
                assert!(
                    tasks.iter().any(|t| t.id == deferral.follow_up),
                    "{} defers to unknown task {}",
                    task.id,
                    deferral.follow_up
                );
            }
        }
    }
}

#[test]
fn dependencies_fan_in_and_chain() {
    let tasks = tasks();

    let fan_in = tasks.iter().filter(|t| t.depends_on.len() >= 3).count();
    assert!(fan_in > 0, "no task waits on three others");

    let same_phase_chain = tasks.iter().any(|task| {
        task.depends_on.iter().any(|dependency| {
            tasks
                .iter()
                .any(|other| &other.id == dependency && other.status == task.status)
        })
    });
    assert!(same_phase_chain, "no task waits on a task in its own phase");
}

#[test]
fn a_task_waits_on_a_blocked_task() {
    let tasks = tasks();
    let blocked: Vec<&str> = tasks
        .iter()
        .filter(|t| t.overlays.blocked.is_some())
        .map(|t| t.id.as_str())
        .collect();
    assert_eq!(blocked.len(), 1);

    let waiting = tasks.iter().any(|task| {
        task.depends_on
            .iter()
            .any(|dependency| blocked.contains(&dependency.as_str()))
    });
    assert!(waiting, "no task waits on the blocked one");
}

#[test]
fn one_fixture_is_blocked() {
    let blocked = tasks()
        .into_iter()
        .filter(|t| t.overlays.blocked.is_some())
        .count();
    assert_eq!(blocked, 1);
}

#[test]
fn smaller_tasks_are_fixtures_too() {
    let store = mock_store();
    let items = store.list_items().expect("listing the mock items");
    assert_eq!(items.len(), 5);

    let kinds: HashSet<&str> = items.iter().map(|i| i.kind.as_str()).collect();
    for kind in ["idea", "feature", "bug"] {
        assert!(kinds.contains(kind), "no item of kind {kind}");
    }

    let promoted = items
        .iter()
        .find(|i| i.status == coding_lanes_core::ItemStatus::Promoted);
    let promoted = promoted.expect("a promoted item");
    let task_id = promoted.promoted_to.clone().expect("the task it became");
    let task = store.load(&task_id).expect("the promoted task");
    assert_eq!(task.promoted_from.as_deref(), Some(promoted.id.as_str()));
    assert!(task.is_small_task());

    let dismissed = items
        .iter()
        .find(|i| i.status == coding_lanes_core::ItemStatus::Dismissed);
    let dismissed = dismissed.expect("a dismissed item");
    assert!(!dismissed.dismissed_reason.is_empty());
}
