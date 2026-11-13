use serde::{Deserialize, Serialize};

/// Who acts on a task. Humans carry no role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub kind: ActorKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

impl Actor {
    pub fn agent(id: impl Into<String>, role: Role) -> Self {
        Self {
            id: id.into(),
            kind: ActorKind::Agent,
            role: Some(role),
            session: None,
        }
    }

    pub fn human(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: ActorKind::Human,
            role: None,
            session: None,
        }
    }

    pub fn in_session(mut self, session: impl Into<String>) -> Self {
        self.session = Some(session.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ActorKind {
    Human,
    Agent,
}

/// The agent roles. `orchestrator` drives autopilot flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    Creator,
    Runner,
    Reviewer,
    Orchestrator,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Creator => "creator",
            Role::Runner => "runner",
            Role::Reviewer => "reviewer",
            Role::Orchestrator => "orchestrator",
        }
    }
}
