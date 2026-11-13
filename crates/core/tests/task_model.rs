use coding_lanes_core::lifecycle::{Event, TransitionContext, TransitionError, transition};
use coding_lanes_core::task::{
    AcceptanceCriterion, Evidence, Finding, FindingStatus, Focus, Round, RoundKind, Verdict,
};
use coding_lanes_core::{Actor, CriterionStatus, Role, Step, StepStatus, Store, Task};

fn creator() -> Actor {
    Actor::agent("creator-1", Role::Creator).in_session("s-creator")
}

fn runner() -> Actor {
    Actor::agent("runner-1", Role::Runner).in_session("s-runner")
}

fn reviewer() -> Actor {
    Actor::agent("reviewer-1", Role::Reviewer).in_session("s-reviewer")
}

fn groomed_task() -> Task {
    let mut task = Task::new("TSK01", "Add lane filtering");
    task.intent.outcome = "raul filters lanes".to_string();
    task.scope.out_of_scope = vec!["no new keys".to_string()];
    task.acceptance_criteria = vec![AcceptanceCriterion {
        id: "ACCEPT-01".to_string(),
        description: "filtering works".to_string(),
        verification: "cargo test".to_string(),
        ..AcceptanceCriterion::default()
    }];
    task.steps = vec![Step {
        id: "STP-01".to_string(),
        order: 1,
        action: "read the filter module".to_string(),
        done_when: "the module is read".to_string(),
        covers: vec!["ACCEPT-01".to_string()],
        ..Step::default()
    }];
    task
}

#[test]
fn draft_needs_outcome_criteria_and_scope() {
    let mut task = Task::new("TSK01", "empty");
    assert!(
        transition(
            &mut task,
            Event::Groom,
            &creator(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.intent.outcome = "something".to_string();
    assert!(
        transition(
            &mut task,
            Event::Groom,
            &creator(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.acceptance_criteria.push(AcceptanceCriterion {
        id: "ACCEPT-01".to_string(),
        ..AcceptanceCriterion::default()
    });
    assert!(
        transition(
            &mut task,
            Event::Groom,
            &creator(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.scope.out_of_scope.push("no more".to_string());
    transition(
        &mut task,
        Event::Groom,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();
    assert_eq!(task.status, coding_lanes_core::Status::Groomed);
}

#[test]
fn only_the_creator_grooms() {
    let mut task = groomed_task();
    let error = transition(
        &mut task,
        Event::Groom,
        &runner(),
        &TransitionContext::default(),
    )
    .expect_err("runner cannot groom");
    assert!(matches!(error, TransitionError::WrongRole { .. }));
}

#[test]
fn approval_needs_coverage_and_done_when() {
    let mut task = groomed_task();
    transition(
        &mut task,
        Event::Groom,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();
    task.steps[0].done_when = String::new();
    assert!(
        transition(
            &mut task,
            Event::Approve,
            &creator(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.steps[0].done_when = "read".to_string();
    transition(
        &mut task,
        Event::Approve,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();
    assert_eq!(task.status, coding_lanes_core::Status::Approved);
}

#[test]
fn start_needs_a_claim() {
    let mut task = groomed_task();
    transition(
        &mut task,
        Event::Groom,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();
    transition(
        &mut task,
        Event::Approve,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();

    assert!(
        transition(
            &mut task,
            Event::Start,
            &runner(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.claim = Some(coding_lanes_core::Claim {
        by: "runner-1".to_string(),
        session: Some("s-runner".to_string()),
        at: "now".to_string(),
        lease_expires_at: String::new(),
    });
    transition(
        &mut task,
        Event::Start,
        &runner(),
        &TransitionContext::default(),
    )
    .unwrap();
    assert_eq!(task.status, coding_lanes_core::Status::Executing);
}

#[test]
fn finish_needs_evidence_and_a_self_round() {
    let mut task = groomed_task();
    transition(
        &mut task,
        Event::Groom,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();
    transition(
        &mut task,
        Event::Approve,
        &creator(),
        &TransitionContext::default(),
    )
    .unwrap();
    task.claim = Some(coding_lanes_core::Claim {
        by: "runner-1".to_string(),
        session: Some("s-runner".to_string()),
        at: "now".to_string(),
        lease_expires_at: String::new(),
    });
    transition(
        &mut task,
        Event::Start,
        &runner(),
        &TransitionContext::default(),
    )
    .unwrap();

    task.steps[0].status = StepStatus::Done;
    assert!(
        transition(
            &mut task,
            Event::Finish,
            &runner(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.acceptance_criteria[0].evidence.push(Evidence {
        command: "cargo test".to_string(),
        exit_code: Some(0),
        summary: "3 passed".to_string(),
        at: "now".to_string(),
        by: "runner-1".to_string(),
        session: Some("s-runner".to_string()),
    });
    assert!(
        transition(
            &mut task,
            Event::Finish,
            &runner(),
            &TransitionContext::default()
        )
        .is_err()
    );

    task.review.rounds.push(Round {
        n: 1,
        kind: RoundKind::SelfReview,
        focus: Focus::SpecCompliance,
        reviewer: "runner-1".to_string(),
        verdict: Verdict::Ok,
        at: "now".to_string(),
        notes: String::new(),
        findings: Vec::new(),
    });
    transition(
        &mut task,
        Event::Finish,
        &runner(),
        &TransitionContext::default(),
    )
    .unwrap();
    assert_eq!(task.status, coding_lanes_core::Status::CodeReview);
}

#[test]
fn criterion_status_comes_from_evidence() {
    let criterion = AcceptanceCriterion {
        id: "ACCEPT-01".to_string(),
        evidence: vec![Evidence {
            command: "cargo test".to_string(),
            exit_code: Some(1),
            summary: "1 failed".to_string(),
            at: "now".to_string(),
            by: "runner-1".to_string(),
            session: None,
        }],
        ..AcceptanceCriterion::default()
    };
    assert_eq!(criterion.effective_status(), CriterionStatus::Failed);
}

#[test]
fn five_external_rounds_escalate_to_a_human() {
    let mut task = groomed_task();
    task.status = coding_lanes_core::Status::CodeReview;
    task.review.verdict = Some(Verdict::ChangesNeeded);
    task.review.findings.push(Finding {
        id: "FIND-01".to_string(),
        status: FindingStatus::Open,
        ..Finding::default()
    });
    for n in 1..=5 {
        task.review.rounds.push(Round {
            n,
            kind: RoundKind::External,
            focus: Focus::EdgeCases,
            reviewer: "reviewer-1".to_string(),
            verdict: Verdict::ChangesNeeded,
            at: "now".to_string(),
            notes: String::new(),
            findings: vec!["FIND-01".to_string()],
        });
    }

    transition(
        &mut task,
        Event::Escalate,
        &reviewer(),
        &TransitionContext::default(),
    )
    .unwrap();
    assert_eq!(task.status, coding_lanes_core::Status::NeedsHuman);
}

#[test]
fn store_round_trips_tasks() {
    let dir = std::env::temp_dir().join("coding-lanes-store-test");
    let _ = std::fs::remove_dir_all(&dir);
    let store = Store::new(&dir);

    let task = store.create("Add lane filtering").unwrap();
    assert_eq!(task.id, "TSK01");
    assert_eq!(task.status, coding_lanes_core::Status::Draft);

    let second = store.create("Second task").unwrap();
    assert_eq!(second.id, "TSK02");

    assert_eq!(store.list().unwrap().len(), 2);
    assert_eq!(store.load("TSK02").unwrap().title, "Second task");

    let _ = std::fs::remove_dir_all(&dir);
}
