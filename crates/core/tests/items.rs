use coding_lanes_core::{Item, ItemError, ItemKind, ItemStatus, Store, TaskKind};
use std::path::PathBuf;

fn temp_store(name: &str) -> (Store, PathBuf) {
    let dir = std::env::temp_dir().join(format!("coding-lanes-items-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    (Store::new(&dir), dir)
}

#[test]
fn an_idea_may_not_be_promoted() {
    let mut item = Item::new("ITM01", "Maybe a lorem thing");
    assert_eq!(item.kind, ItemKind::Idea);
    assert!(!item.is_small_task());

    let error = item
        .promote("TSK01")
        .expect_err("an idea is not promotable");
    assert!(matches!(error, ItemError::NotPromotable(ItemKind::Idea)));
    assert!(
        error
            .to_string()
            .contains("shape it into a small task first")
    );
    assert_eq!(item.status, ItemStatus::Open, "a refusal changes nothing");
}

#[test]
fn shaping_keeps_the_id() {
    let mut item = Item::new("ITM01", "Maybe a lorem thing");
    item.shape(ItemKind::Bug, vec!["the lorem pane".to_string()])
        .expect("shaping an idea");

    assert_eq!(item.id, "ITM01");
    assert_eq!(item.kind, ItemKind::Bug);
    assert!(item.is_small_task());
    assert_eq!(item.scope, vec!["the lorem pane".to_string()]);
}

#[test]
fn promotion_makes_a_small_task_with_one_criterion() {
    let mut item = Item::new("ITM01", "Fix the lorem pane");
    item.shape(ItemKind::Bug, vec!["the lorem pane".to_string()])
        .unwrap();

    let task = item.promote("TSK01").expect("a small task is promotable");

    assert_eq!(task.id, "TSK01");
    assert_eq!(task.kind, TaskKind::Bug);
    assert_eq!(task.promoted_from.as_deref(), Some("ITM01"));
    assert_eq!(task.status, coding_lanes_core::Status::Draft);
    assert_eq!(task.acceptance_criteria.len(), 1);
    assert_eq!(
        task.acceptance_criteria[0].description,
        "Fix the lorem pane"
    );
    assert_eq!(task.scope.in_scope, vec!["the lorem pane".to_string()]);
    assert!(task.is_small_task());

    assert_eq!(item.status, ItemStatus::Promoted);
    assert_eq!(item.promoted_to.as_deref(), Some("TSK01"));
}

#[test]
fn a_settled_item_is_settled() {
    let mut item = Item::new("ITM01", "Fix the lorem pane");
    item.shape(ItemKind::Feature, vec![]).unwrap();
    item.dismiss("not worth it").unwrap();

    assert_eq!(item.status, ItemStatus::Dismissed);
    assert_eq!(item.dismissed_reason, "not worth it");
    assert!(item.promote("TSK01").is_err());
    assert!(item.dismiss("again").is_err());
}

#[test]
fn the_store_keeps_items_and_tasks_apart() {
    let (store, dir) = temp_store("store");

    store.create("A task").unwrap();
    store.create_item("An item").unwrap();

    assert_eq!(store.list().unwrap().len(), 1);
    assert_eq!(store.list_items().unwrap().len(), 1);
    assert_eq!(store.next_id().unwrap(), "TSK02");
    assert_eq!(store.next_item_id().unwrap(), "ITM02");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn promotion_through_the_store_records_the_trail() {
    let (store, dir) = temp_store("promote");

    let mut item = store.create_item("Fix the lorem pane").unwrap();
    item.shape(ItemKind::Bug, vec![]).unwrap();
    store.save_item(&item).unwrap();

    let task = store.promote_item("ITM01").expect("promotion");

    assert_eq!(task.id, "TSK01");
    assert_eq!(task.promoted_from.as_deref(), Some("ITM01"));

    let stored = store.load_item("ITM01").unwrap();
    assert_eq!(stored.status, ItemStatus::Promoted);
    assert_eq!(stored.promoted_to.as_deref(), Some("TSK01"));

    let _ = std::fs::remove_dir_all(&dir);
}
