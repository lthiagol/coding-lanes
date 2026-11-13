use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::{Actor, ActorKind, Role};
use crate::task::{CriterionStatus, StepStatus, Task, Verdict};

/// The canonical status of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    #[default]
    Draft,
    Groomed,
    Approved,
    Executing,
    CodeReview,
    Remediating,
    NeedsHuman,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Groomed => "groomed",
            Status::Approved => "approved",
            Status::Executing => "executing",
            Status::CodeReview => "code-review",
            Status::Remediating => "remediating",
            Status::NeedsHuman => "needs-human",
            Status::Done => "done",
        }
    }
}

/// An overlay: orthogonal to the status, applies to any phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overlay {
    pub reason: String,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub at: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overlays {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<Overlay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred: Option<Overlay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<Overlay>,
}

/// The events that move a task forward or hold it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// draft → groomed
    Groom,
    /// groomed → approved
    Approve,
    /// approved → executing
    Start,
    /// executing → code-review
    Finish,
    /// code-review → done
    PassReview,
    /// code-review → remediating
    RequestChanges,
    /// remediating → code-review
    ReReview,
    /// code-review → needs-human
    Escalate,
    /// needs-human → executing
    ResumeWork,
    /// done → executing
    Reopen,
    /// hold the task
    Block,
    Unblock,
    Defer,
    Undefer,
    Cancel,
}

impl Event {
    /// The role allowed to perform this event.
    pub fn required_role(&self) -> Option<Role> {
        match self {
            Event::Groom | Event::Approve | Event::ResumeWork | Event::Reopen => {
                Some(Role::Creator)
            }
            Event::Start | Event::Finish | Event::ReReview => Some(Role::Runner),
            Event::PassReview | Event::RequestChanges | Event::Escalate => Some(Role::Reviewer),
            Event::Block | Event::Unblock | Event::Defer | Event::Undefer | Event::Cancel => None,
        }
    }
}

/// What the transition needs to know about the world outside the task.
#[derive(Clone, Copy, Default)]
pub struct TransitionContext<'a> {
    /// Whether a dependency task is done. Absent means "cannot check".
    pub dependency_done: Option<&'a dyn Fn(&str) -> bool>,
    /// Reason for an overlay event.
    pub reason: Option<&'a str>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransitionError {
    #[error("{event:?} needs the {role:?} role")]
    WrongRole { event: Event, role: Role },
    #[error("only agents act on tasks")]
    NotAnAgent,
    #[error("cannot {event:?} from status {status:?}")]
    WrongStatus { event: Event, status: Status },
    #[error("gate failed: {0}")]
    Gate(String),
}

/// Apply an event to a task, enforcing the gates.
pub fn transition(
    task: &mut Task,
    event: Event,
    actor: &Actor,
    ctx: &TransitionContext<'_>,
) -> Result<(), TransitionError> {
    if let Some(role) = event.required_role() {
        if actor.kind != ActorKind::Agent {
            return Err(TransitionError::NotAnAgent);
        }
        if actor.role != Some(role) {
            return Err(TransitionError::WrongRole { event, role });
        }
    }

    match event {
        Event::Block => return set_overlay(task, actor, ctx, OverlayKind::Blocked),
        Event::Unblock => return clear_overlay(task, OverlayKind::Blocked),
        Event::Defer => return set_overlay(task, actor, ctx, OverlayKind::Deferred),
        Event::Undefer => return clear_overlay(task, OverlayKind::Deferred),
        Event::Cancel => return set_overlay(task, actor, ctx, OverlayKind::Cancelled),
        _ => {}
    }

    if task.overlays.cancelled.is_some() {
        return Err(TransitionError::Gate("task is cancelled".into()));
    }

    match event {
        Event::Groom => {
            expect_status(task, Status::Draft, event)?;
            gate(
                !task.intent.outcome.trim().is_empty(),
                "intent.outcome is required",
            )?;
            gate(
                !task.acceptance_criteria.is_empty(),
                "at least one acceptance criterion is required",
            )?;
            gate(
                !task.scope.out_of_scope.is_empty(),
                "at least one out_of_scope entry is required",
            )?;
            task.status = Status::Groomed;
        }
        Event::Approve => {
            expect_status(task, Status::Groomed, event)?;
            gate(
                task.open_questions() == 0,
                "open questions must be resolved",
            )?;
            gate(
                task.criteria_covered(),
                "every criterion needs a covering step",
            )?;
            gate(task.steps_have_done_when(), "every step needs a done_when")?;
            task.status = Status::Approved;
        }
        Event::Start => {
            expect_status(task, Status::Approved, event)?;
            let claim = task
                .claim
                .as_ref()
                .ok_or_else(|| TransitionError::Gate("task is not claimed".into()))?;
            gate(claim.by == actor.id, "the task is claimed by someone else")?;
            check_dependencies(task, ctx)?;
            task.status = Status::Executing;
        }
        Event::Finish => {
            expect_status(task, Status::Executing, event)?;
            gate(
                task.steps
                    .iter()
                    .all(|s| matches!(s.status, StepStatus::Done | StepStatus::Skipped)),
                "every step must be settled",
            )?;
            for criterion in &task.acceptance_criteria {
                gate(
                    !criterion.evidence.is_empty(),
                    &format!("{} has no evidence", criterion.id),
                )?;
                match criterion.effective_status() {
                    CriterionStatus::Passed => {}
                    CriterionStatus::Failed => gate(
                        !criterion.failure_reason.trim().is_empty(),
                        &format!("{} failed without a reason", criterion.id),
                    )?,
                    CriterionStatus::Deferred => {}
                    _ => {
                        return Err(TransitionError::Gate(format!(
                            "{} is {}",
                            criterion.id,
                            criterion.effective_status().as_str()
                        )));
                    }
                }
            }
            gate(task.self_rounds() > 0, "a self-review round is required")?;
            task.status = Status::CodeReview;
        }
        Event::PassReview => {
            expect_status(task, Status::CodeReview, event)?;
            gate(
                task.review.verdict == Some(Verdict::Ok),
                "the review verdict is not ok",
            )?;
            gate(task.open_findings() == 0, "open findings remain")?;
            gate(
                task.external_rounds() as u32 <= task.review.max_rounds,
                "the review round budget is exhausted",
            )?;
            check_review_independence(task, actor)?;
            task.status = Status::Done;
        }
        Event::RequestChanges => {
            expect_status(task, Status::CodeReview, event)?;
            gate(
                task.review.verdict == Some(Verdict::ChangesNeeded),
                "the review verdict is not changes-needed",
            )?;
            gate(task.open_findings() > 0, "no findings were opened")?;
            gate(
                (task.external_rounds() as u32) < task.review.max_rounds,
                "the review round budget is exhausted",
            )?;
            check_review_independence(task, actor)?;
            task.status = Status::Remediating;
        }
        Event::ReReview => {
            expect_status(task, Status::Remediating, event)?;
            gate(task.open_findings() == 0, "open findings remain")?;
            if let Some(claim) = &task.claim
                && let Some(session) = &actor.session
            {
                gate(
                    claim.session.as_deref() != Some(session.as_str()),
                    "the session that executed the task cannot remediate it",
                )?;
            }
            task.status = Status::CodeReview;
        }
        Event::Escalate => {
            expect_status(task, Status::CodeReview, event)?;
            gate(
                task.review.verdict == Some(Verdict::ChangesNeeded),
                "the review verdict is not changes-needed",
            )?;
            gate(
                task.external_rounds() as u32 >= task.review.max_rounds,
                "the review round budget is not exhausted",
            )?;
            task.status = Status::NeedsHuman;
        }
        Event::ResumeWork => {
            expect_status(task, Status::NeedsHuman, event)?;
            task.status = Status::Executing;
        }
        Event::Reopen => {
            expect_status(task, Status::Done, event)?;
            task.status = Status::Executing;
        }
        Event::Block | Event::Unblock | Event::Defer | Event::Undefer | Event::Cancel => {
            unreachable!("handled above")
        }
    }

    task.updated = crate::now_rfc3339();
    Ok(())
}

#[derive(Clone, Copy)]
enum OverlayKind {
    Blocked,
    Deferred,
    Cancelled,
}

fn set_overlay(
    task: &mut Task,
    actor: &Actor,
    ctx: &TransitionContext<'_>,
    kind: OverlayKind,
) -> Result<(), TransitionError> {
    let reason = ctx.reason.unwrap_or_default();
    gate(!reason.trim().is_empty(), "a reason is required")?;

    let overlay = Overlay {
        reason: reason.to_string(),
        by: actor.id.clone(),
        at: crate::now_rfc3339(),
    };
    match kind {
        OverlayKind::Blocked => task.overlays.blocked = Some(overlay),
        OverlayKind::Deferred => task.overlays.deferred = Some(overlay),
        OverlayKind::Cancelled => task.overlays.cancelled = Some(overlay),
    }
    task.updated = crate::now_rfc3339();
    Ok(())
}

fn clear_overlay(task: &mut Task, kind: OverlayKind) -> Result<(), TransitionError> {
    match kind {
        OverlayKind::Blocked => task.overlays.blocked = None,
        OverlayKind::Deferred => task.overlays.deferred = None,
        OverlayKind::Cancelled => task.overlays.cancelled = None,
    }
    task.updated = crate::now_rfc3339();
    Ok(())
}

fn check_dependencies(task: &Task, ctx: &TransitionContext<'_>) -> Result<(), TransitionError> {
    if task.depends_on.is_empty() {
        return Ok(());
    }
    let check = ctx.dependency_done.ok_or_else(|| {
        TransitionError::Gate("dependencies cannot be checked without the plan".into())
    })?;
    for dependency in &task.depends_on {
        gate(
            check(dependency),
            &format!("dependency {dependency} is not done"),
        )?;
    }
    Ok(())
}

/// The session that planned a task never reviews it.
fn check_review_independence(task: &Task, actor: &Actor) -> Result<(), TransitionError> {
    if let (Some(planned), Some(session)) = (&task.planned_by, &actor.session) {
        gate(
            planned != session,
            "the session that planned the task cannot review it",
        )?;
    }
    Ok(())
}

fn expect_status(task: &Task, expected: Status, event: Event) -> Result<(), TransitionError> {
    if task.status == expected {
        Ok(())
    } else {
        Err(TransitionError::WrongStatus {
            event,
            status: task.status,
        })
    }
}

fn gate(condition: bool, message: &str) -> Result<(), TransitionError> {
    if condition {
        Ok(())
    } else {
        Err(TransitionError::Gate(message.to_string()))
    }
}

impl CriterionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CriterionStatus::Pending => "pending",
            CriterionStatus::Passed => "passed",
            CriterionStatus::Failed => "failed",
            CriterionStatus::Unproven => "unproven",
            CriterionStatus::Deferred => "deferred",
        }
    }
}
