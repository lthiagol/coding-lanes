# 0008. Session boundaries are structural

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

Two separation rules sit under the whole workflow:

- the session that **executed** a task never **remediates** that task's findings
- the session that **planned** a task never **reviews** it

Both exist for the same reason. A session that wrote the code has the mindset of
the code; asking it to judge its own work means it reads the implementation and
answers for the implementation. A session that planned a task has the mindset of
the plan; asking it to review means it checks the implementation against a brief
it wrote, and finds the brief consistent.

The role split already separates these jobs in name — the runner and the reviewer
are different roles ([ADR 0005](0005-roles.md)). But **roles are not sessions**. An
agent can be a runner in one session and pick up the same task in another; two
review rounds can be filed by the same session wearing different role names; a
runner can fix its own findings by continuing.

If the rule is written as an instruction — *the runner must not fix its own
findings* — then the outcome depends on the agent's compliance and on whether the
human notices. The alternative is to make the boundary a property of the record:
the task carries the identity of the session that planned it, and the claim carries
the identity of the session that executed it, so a transition that would cross a
boundary **cannot be expressed**.

## Decision

**The boundaries are gates, not guidance.**

| Boundary | Rule |
|---|---|
| Execution and remediation | the session that executed a task never remediates that task's findings |
| Planning and review | the session that planned a task never reviews it |

### The record carries the identity

| Field | Records |
|---|---|
| `planned_by` | the session of the creator that wrote and approved the task |
| `claim.session` | the session of the runner that claimed and executed the task |
| `review.rounds[].reviewer` | the session that filed the round |

A transition that would cross a boundary fails, and says which boundary it crossed.

### A review session is distinct from the runner's

The review runs in a session distinct from the runner's, ideally on a different
model: the diversity is what surfaces uncorrelated bugs. The session identity is
what makes that checkable — the model is a preference, the session is a fact.

### The gates, in the model

| Transition | Must hold |
|---|---|
| `remediating → code-review` | the remediation session differs from the execution session |
| `code-review → done` / `→ remediating` / `→ needs-human` | the review session differs from the planning session |

### The alternatives, and why they lost

| Option | Lost because |
|---|---|
| Write the rule as guidance — *the runner must not fix its own findings* | compliance is not enforcement. An agent that has the task open is the agent most likely to continue on it, and nothing in the transition would stop it |
| Rely on the role split alone | roles are what an actor claims to be; sessions are what actually happened. The same session can file a round as `reviewer` |
| Rely on model diversity alone | the model is not recorded in the task and not checked by a gate. It is a preference, and preferences are not fences |
| Record the session on the finding only | a finding does not know who executed the task. The execution session has to live on the task's claim, which is where the work was done |
| Let a human override a boundary | the human already has a route: `needs-human`. A human that needs the work done in this session reopens it, and the record shows it |

## Consequences

### Positive

- The separation is checkable, and the check is a gate that fails a transition.
- The audit trail shows which session did what, without asking anyone.
- The gates are cheap: two comparisons between recorded session ids.

### Negative

- **A blocked task can wait for a new session.** The right session may not be
  available, and the work waits. That is the intended cost, but it is a real
  delay.
- **Session identity has to be reliable.** If sessions are not recorded faithfully,
  the gate is theatre. Recording a session id must be part of acting, not an
  optional field.
- **A human must sometimes break the pattern.** The remedy is `needs-human` and a
  deliberate reopen, not a session that quietly does both jobs.

### Follow-up

- The actor and session shape is in [roles/roles.md](../roles/roles.md); the
  transitions that check it are in
  [tasks/task-model.md](../tasks/task-model.md#transitions).
- How a session identity is minted and carried across talita invocations is part
  of talita's operation surface, which is not decided yet.
