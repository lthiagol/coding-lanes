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
its code. No central store. The CLI tool writes; raul reads.

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
  "meta": { "priority": "normal", "effort": "M", "tags": [] },

  "intent": { "outcome": "", "problem": "" },
  "scope": { "in_scope": [], "out_of_scope": [] },

  "acceptance_criteria": [
    { "id": "ACCEPT-01", "description": "", "verification": "",
      "status": "pending", "evidence": [] }
  ],
  "steps": [
    { "id": "STP-01", "order": 1, "action": "", "done_when": "", "files": [],
      "status": "pending", "covers": [], "depends_on": [] }
  ],

  "review": {
    "max_rounds": 5,
    "rounds": [],
    "verdict": null,
    "findings": []
  },

  "questions": [],
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
| review round | `1..5` | task |

## Vocabulary

| field | values |
|---|---|
| `status` | `draft` `groomed` `approved` `executing` `code-review` `remediating` `needs-human` `done` |
| `overlays` | `blocked` `deferred` `cancelled`, each `{reason, by, at}` |
| `step.status` | `pending` `in-progress` `done` `skipped` `failed` |
| `acceptance_criteria[].status` | `pending` `passed` `failed` `unproven` (derived) |
| `finding.status` | `open` `fixed` `dismissed` |
| `finding.severity` | `low` `medium` `high` |
| `review.verdict` / round verdict | `ok` `changes-needed` |
| `questions[].status` | `open` `resolved` |
| `meta.priority` | `urgent` `high` `normal` `low` |
| `meta.effort` | `S` `M` `L` |

One canonical status plus orthogonal overlays. Never combined status strings.

## Actors and roles

An actor is `{id, kind: human|agent, role?}`. Humans carry no role.

| role | owns | autonomy |
|---|---|---|
| `creator` | creates and edits tasks: intent, scope, acceptance criteria, steps, questions, meta; approves `groomed → approved` | human-dependent — its content comes from an interactive session with a human |
| `runner` | claims approved tasks, writes steps, executes, attaches evidence, applies fixes | autonomous |
| `reviewer` | reviews code, opens findings, gives verdicts; never writes code | autonomous |
| `orchestrator` | autopilot flows | autonomous — later, out of scope for now |

## Transitions

| from → to | actor | must hold |
|---|---|---|
| — → `draft` | creator | title only |
| `draft` → `groomed` | creator | `intent.outcome` set, ≥1 acceptance criterion, ≥1 `out_of_scope` entry |
| `groomed` → `approved` | creator | no open questions, every acceptance criterion covered by a step, every step has `done_when` |
| `approved` → `executing` | runner | task claimed, dependencies satisfied |
| `executing` → `code-review` | runner | all steps settled, every criterion proven or failed with a reason, evidence attached |
| `code-review` → `done` | reviewer | verdict `ok`, no open findings, rounds ≤ 5 |
| `code-review` → `remediating` | reviewer | verdict `changes-needed`, rounds < 5, findings opened |
| `remediating` → `code-review` | runner | all open findings fixed |
| `code-review` → `needs-human` | reviewer | 5 rounds exhausted and still `changes-needed` |
| `needs-human` → `executing` | creator | a human instructed it |
| `done` → `executing` | creator | reopen |
| any → blocked / deferred / cancelled | any | reason required |

A task never completes itself out of repeated review. After 5 autonomous review
rounds with work still outstanding, it stops cycling and escalates to
`needs-human`.

## Review

```json
"review": {
  "max_rounds": 5,
  "rounds": [
    { "n": 1, "reviewer": "reviewer-1", "verdict": "changes-needed",
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

## Evidence

Evidence is appended to an acceptance criterion and its status is derived from
the latest entry.

```json
{ "command": "cargo test", "exit_code": 0, "summary": "3 passed",
  "at": "2026-11-13T12:00:00Z", "by": "runner-1" }
```

`exit_code` 0 with a non-empty summary means `passed`; a non-zero exit means
`failed`. A criterion with no evidence is `pending`. An agent may override the
derived status, but the override is recorded as evidence.

## Steps and criteria

- Acceptance criteria are falsifiable outcomes: what must be true, and how it
  is verified.
- Steps are the actions that get there: `action`, `done_when`, `files`,
  `covers` (acceptance criteria ids), `depends_on` (other step ids).
- Steps are required before approval, but the runner may add or split steps
  while executing.

## Subtasks

`TSK03.1` is a full task with its own status. The parent lists its children
and shows a derived progress indicator. No automatic aggregation.

## Derived, never stored

- acceptance criterion status (from evidence)
- lanes and board views
- review state (`pending-review`, `open-findings`, `clean`)
- children progress
- readiness to claim

## Not defined yet

- journal event kinds
- schema validation
- the CLI tool's operation surface
- raul screens and views
- autopilot
