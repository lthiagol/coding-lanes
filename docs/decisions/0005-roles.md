# 0005. Roles — agent roles only

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

A task moves through states, and the states imply jobs: someone writes the task,
someone executes it, someone reviews it. The model needs to name those jobs, because
the gates must be able to ask *who is allowed to make this transition*.

The temptation is to model the human as one of the roles — a `human` actor with the
role `approver` or `owner` — so that "the human approves" can be written as a role
check. The trouble is that the human is not an agent with a job in the workflow; the
human is the party the whole workflow serves. The human approves, instructs and
decides, but the human does not execute a task or review code, and nothing in the
gates depends on the human having a role.

A second temptation is to let a role be attached per task instance — a runner on
this task, a reviewer on that one. That is already covered by the actor: the actor
carries the role it acts with, and the session it acts in.

A third is to build the orchestrator now. Autopilot is wanted, but an orchestrator
role that has no flow to orchestrate is a role with no gates, and a role with no
gates is a name.

## Decision

**The roles are agent roles. Humans carry no role.**

An actor is:

```json
{ "id": "runner-1", "kind": "agent", "role": "runner", "session": "s-42" }
```

| field | meaning |
|---|---|
| `id` | who acts |
| `kind` | `human` or `agent` |
| `role` | `creator`, `runner`, `reviewer`, `orchestrator` — absent for humans |
| `session` | the working session, so separation between planning, execution, remediation and review is structural |

### The four roles

| role | owns | never |
|---|---|---|
| `creator` | creates and edits the task: intent, scope, acceptance criteria, questions, meta. **Approves** `groomed → approved` | executes or reviews code |
| `runner` | claims approved tasks, writes steps, executes, attaches evidence, resolves findings | treats its own self-review as the verdict; approves a task |
| `reviewer` | reviews code, files rounds, opens findings, gives the verdict | **writes code** |
| `orchestrator` | autopilot flows — starts, sequences and escalates work | anything a human must do |

### The creator approves

Approval is the creator's, not the runner's and not the reviewer's. The creator is
the role whose content comes from an interactive session with a human, so approval
is where the human's intent is recorded as a gate the domain can check.

### The reviewer never codes

The reviewer's value is independence. A reviewer who edits the code is no longer
reviewing it — it has become the runner with an extra step. The rule is
structural: the reviewer has no write access to the code.

### The orchestrator is autopilot-only, and later

The orchestrator sequences and escalates; it does not execute, review or approve.
It is not built in this pass, and no gate refers to it yet.

### The alternatives, and why they lost

| Option | Lost because |
|---|---|
| Give the human a role (`approver`, `owner`) | the human is not an agent doing a job in the workflow; making the human a role would have every gate ask a question about a party that has no job |
| Make the human a `reviewer` | the human reads raul and decides; the reviewer files rounds, findings and verdicts against a diff. Different acts, and the human's decision is not a verdict |
| Roles as fields on the task | a task's role set is fixed at creation, but the actors change hands as it moves. The role belongs to the actor, per action |
| Build the orchestrator now | it would be a role with no flow to run. Autopilot is a later pass; the role is named so the vocabulary does not have to change when it arrives |
| Let the runner approve its own task | approval would be the runner's own judgement of its own brief. The gate loses its point |

## Consequences

### Positive

- Every gate can ask the same question: is this actor's role allowed to make this
  transition?
- Humans stay outside the role system, so the model's vocabulary never implies that
  a human is doing an agent's job.
- The reviewer–runner separation is one rule, and it is checkable.

### Negative

- **A human's actions are untyped.** A human who approves is recorded as an actor
  of kind `human`, and the domain checks the creator's role, not the human's
  identity. The audit trail must therefore carry enough for a reader to see who
  acted.
- **The orchestrator is named but inert.** Until autopilot exists, the role has no
  gates and no flow.
- **A role says what an agent may do, not what it is good at.** A reviewer and a
  runner both hold sessions; the difference is the job.

### Follow-up

- The orchestrator's flow and its gates are decided when autopilot is designed, not
  now.
- The roles, the ownership of each task section and the session boundaries live in
  [roles/roles.md](../roles/roles.md); the ADR is why they are split this way.
