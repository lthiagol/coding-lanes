# Task model

The unit of work in coding-lanes. A task is created by an agent during a
grooming session, executed by an agent, reviewed by an agent, and read by a
human.

## Storage

```
coding-lanes/
  tasks/TSK01-<slug>.json     # one task per file
  activity.json               # append-only event journal
```

Task files live in the managed project's own `coding-lanes/` folder, next to
its code. No central store. talita writes; raul reads.

File naming: `TSK<NN>-<slug>.json`, slug derived from the title.

## Task file

```json
{
  "schema_version": 1,
  "id": "TSK01",
  "title": "Add lane filtering to raul",
  "slug": "add-lane-filtering-to-raul",
  "parent": null,
  "children": [],
  "created": "2026-11-13T10:00:00Z",
  "updated": "2026-11-13T12:30:00Z",

  "status": "groomed",
  "overlays": { "blocked": null, "deferred": null, "cancelled": null },
  "claim": null,
  "planned_by": null,
  "meta": { "priority": "normal", "effort": "M", "tags": [] },

  "intent": { "outcome": "", "problem": "" },
  "scope": { "in_scope": [], "out_of_scope": [] },

  "acceptance_criteria": [
    { "id": "ACCEPT-01", "description": "", "verification": "",
      "status": "pending", "evidence": [], "deferral": null,
      "failure_reason": "" }
  ],
  "steps": [
    { "id": "STP-01", "order": 1, "action": "", "done_when": "", "files": [],
      "status": "pending", "covers": [], "depends_on": [], "deviation": "" }
  ],

  "review": {
    "max_rounds": 5,
    "rounds": [],
    "verdict": null,
    "findings": []
  },

  "questions": [],
  "notes": [],
  "depends_on": []
}
```

## Identifiers

| entity | form | scope |
|---|---|---|
| task | `TSK01`, `TSK03.1` | project |
| acceptance criterion | `ACCEPT-01` | task |
| step | `STP-01` | task |
| question | `QST-01` | task |
| finding | `FIND-01` | task |
| comment | `CMT-01` | task |
| note | `NOTE-01` | task |
| review round | `1..5` (external rounds) | task |

## Vocabulary

| field | values |
|---|---|
| `status` | `draft` `groomed` `approved` `executing` `code-review` `remediating` `needs-human` `done` |
| `overlays` | `blocked` `deferred` `cancelled`, each `{reason, by, at}` |
| `step.status` | `pending` `in-progress` `done` `skipped` `failed` |
| `acceptance_criteria[].status` | `pending` `passed` `failed` `unproven` `deferred` (derived) |
| `finding.status` | `open` `fixed` `dismissed` |
| `finding.severity` | `low` `medium` `high` |
| `review.verdict` / round verdict | `ok` `changes-needed` |
| `review.rounds[].kind` | `self` `external` |
| `review.rounds[].focus` | `spec-compliance` `edge-cases` `security` `empirical` `design` |
| `questions[].status` | `open` `resolved` |
| `meta.priority` | `urgent` `high` `normal` `low` |
| `meta.effort` | `S` `M` `L` |

One canonical status plus orthogonal overlays. Never combined status strings.

## Actors and roles

Actors, roles, ownership, and session boundaries are defined in
[roles/roles.md](../roles/roles.md). Humans carry no role.

## Sessions

The creator's session is recorded in `planned_by`; the runner's in
`claim.session`. The gates use them: the session that planned a task never
reviews it, and the session that executed it never remediates its findings.

## Transitions

```mermaid
stateDiagram-v2
    [*] --> draft : creator
    draft --> groomed : creator
    groomed --> approved : creator
    approved --> executing : runner
    executing --> code-review : runner
    code-review --> done : reviewer / verdict ok
    code-review --> remediating : reviewer / changes-needed
    remediating --> code-review : runner / findings fixed
    code-review --> needs-human : reviewer / 5 rounds exhausted
    needs-human --> executing : creator / human instructed
    done --> executing : creator / reopen
```

Overlays (`blocked`, `deferred`, `cancelled`) apply to any phase and are not
shown in the graph.

| from → to | actor | must hold |
|---|---|---|
| — → `draft` | creator | title only |
| `draft` → `groomed` | creator | `intent.outcome` set, ≥1 acceptance criterion, ≥1 `out_of_scope` entry |
| `groomed` → `approved` | creator | no open questions, every acceptance criterion covered by a step, every step has `done_when` |
| `approved` → `executing` | runner | task claimed, dependencies satisfied |
| `executing` → `code-review` | runner | all steps settled, every criterion proven or failed with a reason, evidence attached, ≥1 `self` round filed |
| `code-review` → `done` | reviewer | verdict `ok`, no open findings, ≤5 external rounds |
| `code-review` → `remediating` | reviewer | verdict `changes-needed`, <5 external rounds, findings opened |
| `remediating` → `code-review` | runner | all open findings fixed, remediation session differs from the execution session |
| `code-review` → `needs-human` | reviewer | 5 external rounds exhausted and still `changes-needed` |
| `needs-human` → `executing` | creator | a human instructed it |
| `done` → `executing` | creator | reopen |
| any → blocked / deferred / cancelled | any | reason required |

A task never completes itself out of repeated review. After 5 autonomous
external review rounds with work still outstanding, it stops cycling and
escalates to `needs-human`.

## Review

Two kinds of round, in order:

- `self` — the runner reviews its own work before handing off. It catches
  obvious issues but is biased; it is a pre-screen, not the verdict.
- `external` — an independent agent reviews the implementation. It must not
  be the session that wrote the code, and ideally runs a different model.

Each round carries a `focus`, so successive rounds look at the change through
different lenses instead of repeating the same check.

```json
"review": {
  "max_rounds": 5,
  "rounds": [
    { "n": 1, "kind": "self", "focus": "spec-compliance",
      "reviewer": "runner-1", "verdict": "changes-needed",
      "at": "", "notes": "", "findings": ["FIND-01"] },
    { "n": 2, "kind": "external", "focus": "edge-cases",
      "reviewer": "reviewer-1", "verdict": "changes-needed",
      "at": "", "notes": "", "findings": ["FIND-01"] }
  ],
  "verdict": null,
  "findings": [
    { "id": "FIND-01", "severity": "medium", "status": "open",
      "description": "", "created": "", "resolved": "", "resolution": "",
      "comments": [ { "id": "CMT-01", "author": "", "body": "", "at": "" } ] }
  ]
}
```

The 5-round cap counts external rounds. Self-review is a pre-screen and does
not consume the budget.

## Evidence

Evidence is appended to an acceptance criterion and its status is derived from
the latest entry.

```json
{ "command": "cargo test", "exit_code": 0, "summary": "3 passed",
  "at": "2026-11-13T12:00:00Z", "by": "runner-1", "session": "s-42" }
```

`exit_code` 0 with a non-empty summary means `passed`; a non-zero exit means
`failed`. A criterion with no evidence is `pending`. An agent may override the
derived status, but the override is recorded as evidence — a bypass is
explicit debt, never a silent one.

## Steps and criteria

- Acceptance criteria are falsifiable outcomes: what must be true, and how it
  is verified.
- Steps are the actions that get there: `action`, `done_when`, `files`,
  `covers` (acceptance criteria ids), `depends_on` (other step ids).
- Steps are required before approval, but the runner may add or split steps
  while executing.
- Steps are guidance, not a blueprint. When the runner takes a different
  path, the note goes on the step: `deviation`. It stays on the task as part
  of the audit trail and feeds future task crafting.

## Scope-down visibility

Partial scope is legitimate; hidden scope is not. A criterion that will not be
delivered records why, and where the work went instead:

```json
"deferral": { "reason": "needs upstream API change", "follow_up": "TSK07" }
```

A deferred criterion does not block completion, but it stays visible in raul.

## Notes

The task carries what the execution learned, so it remains the single place
to look for where the work stands:

```json
"notes": [ { "id": "NOTE-01", "author": "runner-1", "body": "", "at": "" } ]
```

## Immutability after approval

Once approved, `intent`, `scope`, and the acceptance criteria are frozen. The
execution approach stays open: steps, evidence, findings, and notes keep
changing.

A refinement is not a silent edit. Small adjustments become child tasks
(`TSK03.1`) — each with its own brief, criteria, evidence, and review — so
the original task stays the record of the original intent.

## Subtasks

`TSK03.1` is a full task with its own status. The parent lists its children
and shows a derived progress indicator. No automatic aggregation.

## Derived, never stored

- acceptance criterion status (from evidence)
- lanes and board views
- review state (`pending-review`, `open-findings`, `clean`)
- children progress
- readiness to claim

## Worked examples

`mock-data/coding-lanes/tasks/` holds one task per lifecycle stage, plus
edge cases (blocked overlay, deferred criterion, child task). Every fixture
satisfies the gates of the phase it is in. See [mock-data/README.md](../../mock-data/README.md).

## Not defined yet

- journal event kinds
- project config
- talita's operation surface
- raul screens and views
- autopilot
