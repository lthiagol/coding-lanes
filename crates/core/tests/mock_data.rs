use coding_lanes_core::{Status, Store};
use std::path::PathBuf;

fn mock_store() -> Store {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("mock-data/coding-lanes");
    Store::new(root)
}

#[test]
fn every_fixture_loads() {
    let store = mock_store();
    let tasks = store.list().expect("listing the mock tasks");
    assert_eq!(tasks.len(), 12);

    let mut ids: Vec<&str> = tasks.iter().map(|t| t.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 12, "task ids must be unique");
}

#[test]
fn every_stage_is_covered() {
    let store = mock_store();
    let tasks = store.list().expect("listing the mock tasks");

    let covered: Vec<Status> = tasks.iter().map(|t| t.status).collect();
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
            covered.contains(&status),
            "no fixture for status {status:?}"
        );
    }
}

#[test]
fn fixtures_keep_their_gates() {
    let store = mock_store();

    let approved = store.load("TSK03").unwrap();
    assert!(approved.criteria_covered());
    assert!(approved.steps_have_done_when());

    let code_review = store.load("TSK05").unwrap();
    assert!(
        code_review.self_rounds() > 0,
        "code-review needs a self round"
    );

    let done = store.load("TSK08").unwrap();
    assert_eq!(done.open_findings(), 0);
    assert!(done.external_rounds() as u32 <= done.review.max_rounds);

    let needs_human = store.load("TSK07").unwrap();
    assert_eq!(
        needs_human.external_rounds() as u32,
        needs_human.review.max_rounds
    );
    assert_eq!(needs_human.open_findings(), 5);
}

#[test]
fn child_task_points_at_its_parent() {
    let store = mock_store();
    let child = store.load("TSK11").unwrap();
    assert_eq!(child.parent.as_deref(), Some("TSK03"));

    let parent = store.load("TSK03").unwrap();
    assert!(parent.children.contains(&child.id));
}

#[test]
fn deferred_criterion_names_its_follow_up() {
    let store = mock_store();
    let task = store.load("TSK10").unwrap();
    let criterion = task.criterion("ACCEPT-02").unwrap();
    let deferral = criterion.deferral.as_ref().unwrap();
    assert_eq!(deferral.follow_up, "TSK12");
    assert!(store.load("TSK12").is_ok());
}
