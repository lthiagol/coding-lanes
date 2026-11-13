use serde::{Deserialize, Serialize};

use crate::actor::Actor;
use crate::lifecycle::{Overlays, Status};

/// Schema version written into every task file.
pub const SCHEMA_VERSION: u32 = 1;

/// A unit of work. See `docs/tasks/task-model.md`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub overlays: Overlays,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<Claim>,
    /// Session of the creator that planned the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_by: Option<String>,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub intent: Intent,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default)]
    pub review: Review,
    #[serde(default)]
    pub questions: Vec<Question>,
    #[serde(default)]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

fn default_schema_version() -> u32 {
    SCHEMA_VERSION
}

impl Task {
    /// A new task in `draft`.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            schema_version: SCHEMA_VERSION,
            slug: slugify(&title),
            id: id.into(),
            title,
            parent: None,
            children: Vec::new(),
            created: crate::now_rfc3339(),
            updated: crate::now_rfc3339(),
            status: Status::Draft,
            overlays: Overlays::default(),
            claim: None,
            planned_by: None,
            meta: Meta::default(),
            intent: Intent::default(),
            scope: Scope::default(),
            acceptance_criteria: Vec::new(),
            steps: Vec::new(),
            review: Review::default(),
            questions: Vec::new(),
            notes: Vec::new(),
            depends_on: Vec::new(),
        }
    }

    pub fn planned_by(mut self, actor: &Actor) -> Self {
        self.planned_by = actor.session.clone();
        self
    }

    pub fn criterion(&self, id: &str) -> Option<&AcceptanceCriterion> {
        self.acceptance_criteria.iter().find(|c| c.id == id)
    }

    pub fn criterion_mut(&mut self, id: &str) -> Option<&mut AcceptanceCriterion> {
        self.acceptance_criteria.iter_mut().find(|c| c.id == id)
    }

    pub fn step(&self, id: &str) -> Option<&Step> {
        self.steps.iter().find(|s| s.id == id)
    }

    pub fn step_mut(&mut self, id: &str) -> Option<&mut Step> {
        self.steps.iter_mut().find(|s| s.id == id)
    }

    pub fn finding_mut(&mut self, id: &str) -> Option<&mut Finding> {
        self.review.findings.iter_mut().find(|f| f.id == id)
    }

    pub fn open_questions(&self) -> usize {
        self.questions
            .iter()
            .filter(|q| q.status == QuestionStatus::Open)
            .count()
    }

    pub fn open_findings(&self) -> usize {
        self.review
            .findings
            .iter()
            .filter(|f| f.status == FindingStatus::Open)
            .count()
    }

    pub fn self_rounds(&self) -> usize {
        self.review
            .rounds
            .iter()
            .filter(|r| r.kind == RoundKind::SelfReview)
            .count()
    }

    pub fn external_rounds(&self) -> usize {
        self.review
            .rounds
            .iter()
            .filter(|r| r.kind == RoundKind::External)
            .count()
    }

    /// True when every acceptance criterion is covered by at least one step.
    pub fn criteria_covered(&self) -> bool {
        self.acceptance_criteria
            .iter()
            .all(|c| self.steps.iter().any(|s| s.covers.contains(&c.id)))
    }

    /// True when every step carries a `done_when`.
    pub fn steps_have_done_when(&self) -> bool {
        self.steps.iter().all(|s| !s.done_when.trim().is_empty())
    }
}

/// Lowercase, hyphenated form of a title.
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut pending_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    out
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    pub at: String,
    #[serde(default)]
    pub lease_expires_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub priority: Priority,
    #[serde(default)]
    pub effort: Effort,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            priority: Priority::Normal,
            effort: Effort::Medium,
            tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Priority {
    Urgent,
    High,
    #[default]
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Effort {
    #[serde(rename = "S")]
    Small,
    #[serde(rename = "M")]
    #[default]
    Medium,
    #[serde(rename = "L")]
    Large,
}

impl Effort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Effort::Small => "S",
            Effort::Medium => "M",
            Effort::Large => "L",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intent {
    #[serde(default)]
    pub outcome: String,
    #[serde(default)]
    pub problem: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    #[serde(default)]
    pub in_scope: Vec<String>,
    #[serde(default)]
    pub out_of_scope: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceptanceCriterion {
    pub id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub verification: String,
    #[serde(default)]
    pub status: CriterionStatus,
    #[serde(default)]
    pub evidence: Vec<Evidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferral: Option<Deferral>,
    /// Why the criterion failed, when it did.
    #[serde(default)]
    pub failure_reason: String,
}

impl AcceptanceCriterion {
    /// Status from the latest evidence, or the stored status when there is
    /// none. A manual override is recorded as evidence.
    pub fn effective_status(&self) -> CriterionStatus {
        match self.evidence.last() {
            Some(evidence) => evidence.outcome(),
            None => self.status,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum CriterionStatus {
    #[default]
    Pending,
    Passed,
    Failed,
    Unproven,
    Deferred,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    #[serde(default)]
    pub command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub at: String,
    #[serde(default)]
    pub by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

impl Evidence {
    pub fn outcome(&self) -> CriterionStatus {
        match self.exit_code {
            Some(0) if !self.summary.trim().is_empty() => CriterionStatus::Passed,
            Some(_) => CriterionStatus::Failed,
            None => CriterionStatus::Unproven,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deferral {
    pub reason: String,
    #[serde(default)]
    pub follow_up: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub done_when: String,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub status: StepStatus,
    #[serde(default)]
    pub covers: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// Note when the runner took a different path than the step described.
    #[serde(default)]
    pub deviation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum StepStatus {
    #[default]
    Pending,
    InProgress,
    Done,
    Skipped,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Review {
    #[serde(default = "default_max_rounds")]
    pub max_rounds: u32,
    #[serde(default)]
    pub rounds: Vec<Round>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verdict: Option<Verdict>,
    #[serde(default)]
    pub findings: Vec<Finding>,
}

fn default_max_rounds() -> u32 {
    5
}

impl Default for Review {
    fn default() -> Self {
        Self {
            max_rounds: default_max_rounds(),
            rounds: Vec::new(),
            verdict: None,
            findings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Round {
    pub n: u32,
    pub kind: RoundKind,
    pub focus: Focus,
    #[serde(default)]
    pub reviewer: String,
    pub verdict: Verdict,
    #[serde(default)]
    pub at: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum RoundKind {
    #[default]
    #[serde(rename = "self")]
    SelfReview,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Focus {
    #[default]
    SpecCompliance,
    EdgeCases,
    Security,
    Empirical,
    Design,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    #[default]
    Ok,
    ChangesNeeded,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    #[serde(default)]
    pub severity: Severity,
    #[serde(default)]
    pub status: FindingStatus,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub resolved: String,
    #[serde(default)]
    pub resolution: String,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum FindingStatus {
    #[default]
    Open,
    Fixed,
    Dismissed,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub at: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    #[serde(default)]
    pub question: String,
    #[serde(default)]
    pub status: QuestionStatus,
    #[serde(default)]
    pub answer: String,
    #[serde(default)]
    pub raised_by: String,
    #[serde(default)]
    pub raised_at: String,
    #[serde(default)]
    pub resolved_by: String,
    #[serde(default)]
    pub resolved_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum QuestionStatus {
    #[default]
    Open,
    Resolved,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub at: String,
}
