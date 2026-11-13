# Task model

The unit of work in coding-lanes. One task, one file, JSON, under the managed
project's own `coding-lanes/work/` folder. A task is groomed by a creator
agent with a human, executed by a runner agent, reviewed by a reviewer agent,
and read by a human.

| doc | what it owns |
|---|---|
| [roles/roles.md](../roles/roles.md) | actors, roles, who may write which section, session boundaries |
| [tasks/smaller-tasks.md](smaller-tasks.md) | the work that is not a task yet (`ITM01`) |
| [vocabulary.md](../vocabulary.md) | the names of record |
| [decisions/](../decisions/) | why each choice was made |

The code is `crates/core` (`task.rs`, `lifecycle.rs`, `actor.rs`, `store.rs`);
`crates/core/tests/task_model.rs` carries the gates.

## 1. Design constraints

Every rule below traces to a failure the model has to survive. This is the
checklist the domain must satisfy.

| # | Constraint | Why |
|--:|---|---|
| C1 | **One source of truth per fact.** Nothing derived is stored. | A stored value that can be derived is a value that can disagree with the thing it was derived from |
| C2 | **No index file.** The folder *is* the index. | An index drifts from the records it indexes; the folder cannot |
| C3 | **Every field has a writer.** No dead flags. | A flag nothing writes is a flag the reader must still interpret |
| C4 | **Gates and transitions are one step.** | A transition that can be performed without its guards is a transition with no guards |
| C5 | **Every state is reachable and producible.** | A state nothing produces is a state the reader must still render |
| C6 | **One closed enum per concept.** | Three answers to one question in three places is three answers |
| C7 | **Evidence, not assertion.** | A result that matched nothing is not a result |
| C8 | **Evidence is append-only.** | Overwriting evidence erases the record of what was actually run |
| C9 | **The domain is UI-free.** | The two surfaces share the model; neither shells out to the other |
| C10 | **One writer.** talita writes, raul reads. | Two writers to one file is a merge conflict with no referee |
| C11 | **Unknown values are errors.** | A reader that tolerates what it does not know reports a task that is not there |
| C12 | **Material edits after approval are visible.** | An approved task rewritten in place leaves no trail |

## 2. Anatomy

### 2.1 The sections

```
Task
├── identity      id · title · slug · parent · children · created · updated · kind · promoted_from
├── status        status · overlays · claim · planned_by
├── meta          priority · effort · tags
├── intent        outcome · problem
├── scope         in_scope[] · out_of_scope[]
├── criteria      [AcceptanceCriterion]
├── steps         [Step]
├── review        max_rounds · rounds[] · verdict · findings[]
├── questions     [Question]
├── notes         [Note]
├── depends_on[]
```

Every section has one owner (see
[roles/roles.md § Ownership by section](../roles/roles.md#ownership-by-section)).
A role may not write another role's section.

### 2.2 The file

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
  "kind": "task",
  "promoted_from": null,
  "status": "groomed",
  "overlays": { "blocked": null, "deferred": null, "cancelled": null },
  "claim": null,
  "planned_by": "s-creator-1",
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

### 2.3 Fields

| field | type | meaning |
|---|---|---|
| `schema_version` | `u32` | the shape of this file. Currently `1` |
| `id` | string | `TSK01`, child `TSK03.1` |
| `title` | string | what the task is |
| `slug` | string | the title, lowercased and hyphenated. Drives the file name |
| `parent` | id or null | the task this one refines |
| `children[]` | ids | the tasks this one spawned |
| `created`, `updated` | timestamp | RFC 3339. `updated` moves on every write |
| `kind` | `task` \| `bug` \| `feature` | `bug`/`feature` is a **small task**: the step rules do not apply to it. See [smaller-tasks.md](smaller-tasks.md) |
| `promoted_from` | id or null | the smaller task this task became |
| `status` | enum | one canonical status. Never a combined string |
| `overlays` | three slots | orthogonal to the status. Each slot holds `{reason, by, at}` or null |
| `claim` | object or null | `{by, session, at, lease_expires_at}` — who holds the task and in which session |
| `planned_by` | session or null | the creator session that wrote the task. Used by the review-independence gate |
| `meta.priority` | `urgent` \| `high` \| `normal` \| `low` | |
| `meta.effort` | `S` \| `M` \| `L` | |
| `meta.tags[]` | strings | free |
| `intent.outcome` | string | what must be true when the task is done |
| `intent.problem` | string | why it is worth doing |
| `scope.in_scope[]` | strings | the work |
| `scope.out_of_scope[]` | strings | the work that is explicitly **not** in this task. At least one is required to groom |
| `depends_on[]` | ids | tasks that must be `done` before this one starts |
| `questions[]` | `[Question]` | `{id, question, status, answer, raised_by, raised_at, resolved_by, resolved_at}` |
| `notes[]` | `[Note]` | `{id, author, body, at}` — what the execution learned |

### 2.4 Identifiers

| entity | form | scope |
|---|---|---|
| task | `TSK01`, child `TSK03.1` | project |
| smaller task | `ITM01` | project |
| acceptance criterion | `ACCEPT-01` | task |
| step | `STP-01` | task |
| question | `QST-01` | task |
| finding | `FIND-01` | task |
| comment | `CMT-01` | task |
| note | `NOTE-01` | task |
| review round | `n`, 1.. — the count is of **external** rounds | task |

Ids are allocated by **scanning what exists**, never from a count of files: the
highest plain `TSK` number in the folder plus one, zero-padded to two digits.
A deleted task must not hand its id to a new one. Child ids (`TSK03.1`) are
not plain numbers and are skipped by the scan.

### 2.5 Mutability

| Section | Mutability | Written by |
|---|---|---|
| `intent`, `scope`, `acceptance_criteria` | **Frozen at approval.** A change ⇒ a child task | creator, pre-approval |
| `acceptance_criteria[].evidence` | Append-only | runner |
| `acceptance_criteria[].status` | Derived from the latest evidence; the stored value mirrors it | runner (mirrors), domain (derives) |
| `steps` | Mutable, always. A different path is recorded, not hidden | runner |
| `claim`, `status` | Written by transitions only | domain |
| `review` | Append-only for rounds and findings; `verdict` moves | reviewer |
| `notes`, `questions` | Append-only | any role |
| `meta` | Mutable, pre-approval | creator |
| `overlays` | Mutable, each with a reason, a `by` and an `at` | any actor |

## 3. The lifecycle

### 3.1 States

Eight, one canonical status at a time. Overlays are orthogonal and are not
states.

| State | Meaning |
|---|---|
| `draft` | Being written. Intent may be incomplete |
| `groomed` | Outcome, criteria and out-of-scope written; open questions still allowed |
| `approved` | **Locked.** Every criterion covered, every step has a `done_when`. The frozen sections are frozen |
| `executing` | Claimed and being implemented. Steps may be revised |
| `code-review` | Work done, criteria proven or failed with a reason, evidence attached, self-review filed |
| `remediating` | External findings open; being fixed |
| `needs-human` | The autonomous review budget is spent and work is still outstanding. A human decides |
| `done` | Reviewed clean. Terminal until reopened |

`done` is the only terminal state, and it is not final: a creator may reopen it.

### 3.2 Transitions

```mermaid
stateDiagram-v2
    direction TB

    [*] --> draft : create

    draft --> groomed : groom<br/>intent.outcome_present · criteria.present · scope.out_of_scope_declared
    groomed --> approved : approve<br/>questions.resolved · criteria.covered · steps.done_when_present
    approved --> executing : start<br/>task.claimed · dependencies.satisfied
    executing --> code-review : finish<br/>steps.settled · evidence.attached · criteria.proven · self_round.filed
    code-review --> done : pass_review<br/>review.verdict_ok · review.no_open_findings · review.within_budget · review.independent
    code-review --> remediating : request_changes<br/>review.verdict_changes_needed · review.findings_exist · review.budget_left · review.independent
    remediating --> code-review : re_review<br/>review.findings_closed · remediation.session_distinct
    code-review --> needs-human : escalate<br/>review.verdict_changes_needed · review.budget_exhausted
    needs-human --> executing : resume_work<br/>human.instructed
    done --> executing : reopen

    note right of remediating
      The review loop.
      Findings open → remediate →
      findings closed → back to code-review.
    end note

    note right of needs-human
      The budget is spent.
      The task does not review
      itself into completion.
    end note
```

Each edge is labelled with the event and the gates that guard it. The gates are
defined once, in [§ 3.3](#33-gates).

| from → to | event | role | must hold |
|---|---|---|---|
| — → `draft` | create | creator | a title |
| `draft` → `groomed` | `groom` | creator | `intent.outcome` set, ≥1 acceptance criterion, ≥1 `out_of_scope` entry |
| `groomed` → `approved` | `approve` | creator | no open questions, every criterion covered by a step, every step has `done_when` — **the step rules do not apply to a small task** (`kind` `bug` or `feature`) |
| `approved` → `executing` | `start` | runner | task claimed, dependencies satisfied |
| `executing` → `code-review` | `finish` | runner | all steps settled, every criterion proven or failed with a reason, evidence attached, ≥1 `self` round filed — **a small task has no steps to settle** |
| `code-review` → `done` | `pass_review` | reviewer | verdict `ok`, no open findings, within the external-round budget, reviewer session is not the planning session |
| `code-review` → `remediating` | `request_changes` | reviewer | verdict `changes-needed`, ≥1 finding opened, budget not exhausted, reviewer session is not the planning session |
| `remediating` → `code-review` | `re_review` | runner | all open findings fixed, and the remediation session differs from the execution session |
| `code-review` → `needs-human` | `escalate` | reviewer | 5 external rounds exhausted and still `changes-needed` |
| `needs-human` → `executing` | `resume_work` | creator | a human instructed it |
| `done` → `executing` | `reopen` | creator | — |

**A task never completes itself out of repeated review.** After five autonomous
external rounds with work still outstanding, the loop stops and the task
escalates to `needs-human`. The alternative — a sixth round — is the task
grading its own homework.

**A cancelled task refuses every transition.** `overlays.cancelled` is terminal
by overlay: there is no un-cancel event, because a cancelled task is a record
of a decision, not a state to be walked back from.

### 3.3 Gates

One table, one definition each. The gates live in `crates/core/src/lifecycle.rs`;
the transitions run them internally, so a transition and its guards cannot be
applied apart.

| Gate | Checks | Applies to |
|---|---|---|
| `intent.outcome_present` | `intent.outcome` is non-empty | `groom` |
| `criteria.present` | ≥1 acceptance criterion | `groom` |
| `scope.out_of_scope_declared` | ≥1 `scope.out_of_scope` entry | `groom` |
| `questions.resolved` | no open question | `approve` |
| `criteria.covered` | every criterion has ≥1 covering step | `approve` |
| `steps.done_when_present` | every step has a non-empty `done_when` | `approve` |
| `task.claimed` | a claim exists and `claim.by` is the acting runner | `start` |
| `dependencies.satisfied` | every `depends_on` task is `done` | `start` |
| `steps.settled` | every step is `done` or `skipped` | `finish` |
| `evidence.attached` | every criterion carries ≥1 evidence entry | `finish` |
| `criteria.proven` | every criterion is `passed`, `failed` **with a `failure_reason`**, or `deferred` | `finish` |
| `self_round.filed` | ≥1 round of kind `self` | `finish` |
| `review.verdict_ok` | `review.verdict` is `ok` | `pass_review` |
| `review.verdict_changes_needed` | `review.verdict` is `changes-needed` | `request_changes`, `escalate` |
| `review.no_open_findings` | no open finding | `pass_review` |
| `review.findings_exist` | ≥1 open finding | `request_changes` |
| `review.findings_closed` | no open finding | `re_review` |
| `review.within_budget` | external rounds ≤ `max_rounds` | `pass_review` |
| `review.budget_left` | external rounds < `max_rounds` | `request_changes` |
| `review.budget_exhausted` | external rounds ≥ `max_rounds` | `escalate` |
| `review.independent` | the acting session is not `planned_by` | `pass_review`, `request_changes` |
| `remediation.session_distinct` | the acting session is not `claim.session` | `re_review` |
| `overlay.reason_present` | a non-empty reason | `block`, `defer`, `cancel` |

**The budget is asymmetric on purpose.** `pass_review` accepts exactly
`max_rounds`; `request_changes` needs one round left. So the fifth
`changes-needed` round cannot cycle: the only move left is `escalate`.

**Dependencies are checked, not assumed.** `dependencies.satisfied` needs the
store — a check that cannot be performed is a refusal, not a pass.

### 3.4 Overlays

Overlays ride alongside the status and are not edges on the lifecycle graph.
Any actor may set one; a reason is always required.

| Overlay | Fields | Notes |
|---|---|---|
| `blocked` | `reason`, `by`, `at` | Something outside the task's control stops it. A blocked task can be half-executed |
| `deferred` | `reason`, `by`, `at` | Parked on purpose. Re-openable |
| `cancelled` | `reason`, `by`, `at` | Terminal by overlay. Refuses every transition |

### 3.5 Session boundaries

Two rules, both enforced as gates:

| Rule | Gate |
|---|---|
| The session that **planned** a task never **reviews** it | `review.independent` |
| The session that **executed** a task never **remediates** its findings | `remediation.session_distinct` |

The task records the planning session in `planned_by`; the claim records the
execution session in `claim.session`. A reviewer is ideally a different
*model*, not only a different session — the diversity is what surfaces
uncorrelated bugs.

## 4. Review

### 4.1 Rounds

Two kinds, and they are not interchangeable.

| kind | who | counts against the budget |
|---|---|---|
| `self` | the runner, on its own work, before handing it over | no |
| `external` | an independent agent, not the executing session | yes |

A `self` round is a **pre-screen**, not the verdict: it catches the obvious
before a second agent spends a round on it. The runner may file a `self`
round, but only a `reviewer` may move the task out of `code-review` — the
`pass_review` and `request_changes` events are reviewer-only.

Each round carries a `focus`, so successive rounds look at the change through
different lenses instead of repeating the same check.

| field | values |
|---|---|
| `n` | the round number |
| `kind` | `self` \| `external` |
| `focus` | `spec-compliance` \| `edge-cases` \| `security` \| `empirical` \| `design` |
| `reviewer` | who reviewed |
| `verdict` | `ok` \| `changes-needed` |
| `at` | timestamp |
| `notes` | free text |
| `findings[]` | the finding ids opened in this round |

`review.verdict` is the **latest** round's verdict. It is what the gates read.

### 4.2 Findings

| field | values |
|---|---|
| `id` | `FIND-01` |
| `severity` | `low` \| `medium` \| `high` |
| `status` | `open` \| `fixed` \| `dismissed` |
| `description` | what is wrong |
| `created` | timestamp |
| `resolved` | timestamp |
| `resolution` | what was done, or why it was dismissed |
| `comments[]` | `{id, author, body, at}` — the discussion under the finding |

The reviewer opens findings; the runner resolves them. A finding that is
`fixed` records **what** was fixed; a finding that is `dismissed` records **why**
it is not worth fixing. The outcome is written once — the second writer is
refused, because a dismissal that overwrote a fix would erase the record of
the work that was done.

### 4.3 The round budget

`review.max_rounds` is `5` and counts **external** rounds. Self-review is a
pre-screen and does not consume it.

Five external rounds with work still outstanding and the task is `needs-human`.
The loop is bounded so that a task cannot be approved by the same kind of
agent indefinitely.

## 5. Evidence

Evidence is attached to a criterion and is the only thing that proves it.

```json
{ "command": "cargo test --workspace", "exit_code": 0, "summary": "10 passed; 0 failed",
  "at": "2026-11-10T12:00Z", "by": "runner-01", "session": "session-runner-01" }
```

| rule | detail |
|---|---|
| **Attached, never prose** | `{command, exit_code, summary, at, by, session}` |
| **Append-only** | Re-running appends. It never overwrites |
| **Exit code is the truth** | `exit 0` with a non-empty summary is a pass; any other exit code is a failure; no exit code at all is `unproven` |
| **Status is derived** | A criterion's status is the outcome of its **latest** evidence: `passed` \| `failed` \| `unproven`. No evidence ⇒ the stored `status` stands (`pending`) |
| **A failure needs a reason** | `failure_reason` is required when a criterion is `failed`; `criteria.proven` refuses a failure without one |
| **A manual override is evidence** | An agent may override a derived status, and the override is recorded as an evidence entry. A bypass is **explicit debt**, never a silent one |
| **Evidence is per criterion** | The command proves one thing; it does not prove the next criterion |

`exit 0` with an **empty** summary is a failure, not a pass: a command that ran
and said nothing has proved nothing.

## 6. Steps and criteria

**Acceptance criteria are the outcomes.** What must be true, and how it is
verified.

| field | meaning |
|---|---|
| `id` | `ACCEPT-01` |
| `description` | what must be true |
| `verification` | a command, or a reproducible procedure. **Not prose** |
| `status` | derived: `pending` \| `passed` \| `failed` \| `unproven` \| `deferred` |
| `evidence[]` | append-only |
| `deferral` | `{reason, follow_up}` — required when the criterion is deferred |
| `failure_reason` | why it failed, when it did |

**Steps are the actions that get there.**

| field | meaning |
|---|---|
| `id` | `STP-01` |
| `order` | position |
| `action` | what to do |
| `done_when` | **Observable** — how you will know it worked |
| `files[]` | where. A hint, not a contract |
| `status` | `pending` \| `in-progress` \| `done` \| `skipped` \| `failed` |
| `covers[]` | the acceptance criteria ids this step advances |
| `depends_on[]` | other step ids |
| `deviation` | set when the step was done differently than described, with why |

Steps are **guidance, not a blueprint**. The runner may add, split or drop
steps while executing; what changed goes on the step as `deviation`, where it
stays as part of the record.

The rule of thumb: a criterion is what must be true, a step is one way of
making it true, and `covers[]` is the line between them.

## 7. Scope-down visibility

Partial scope is legitimate. Hidden scope is not.

```json
"deferral": { "reason": "needs an upstream API change", "follow_up": "TSK03" }
```

A deferred criterion does not block completion — `criteria.proven` accepts it —
but it stays visible in raul and on the task. The work that was not done is
recorded, with a reason and a pointer to where it went.

## 8. Notes and questions

**Notes** (`NOTE-nn`) carry what the execution learned — the part that is not a
step and not a finding, but that the next task of the same shape should not
have to rediscover. Any role may write one.

**Questions** (`QST-nn`) carry what is still undecided. An open question blocks
approval (`questions.resolved`); a resolved one keeps its answer.

## 9. Immutability after approval

Once approved, `intent`, `scope` and the acceptance criteria are **frozen**.
The execution approach stays open: steps, evidence, findings, notes and rounds
keep changing.

**A refinement is not a silent edit.** It becomes a child task — `TSK03.1` —
with its own brief, criteria, evidence and review. The parent keeps the record
of the intent it was approved against, and the child carries the change.

### 9.1 Why there is no content fingerprint

An obvious alternative is to hash the frozen sections at approval and reject
any later change. We deliberately do not.

| | fingerprint | child task |
|---|---|---|
| What it catches | a silent edit to a frozen section | nothing on its own — it *prevents* the edit by giving it another home |
| What it costs | a digest to compute, store, compare and invalidate; a hash that must be re-derived by every reader, and a plan that a rename or a field reordering can invalidate | a child id in the parent, and a task file |
| What it can tell a reader | "this file was tampered with" | "this is what the task became" — the *refinement*, not the suspicion |
| Failure mode | a false accusation: a task nobody edited is refused because the digest moved | an extra task, which is visible and cheap |

The trail the fingerprint was meant to preserve is carried by the **child-task
path** instead: the parent's frozen sections stay frozen, the change is a
first-class artifact with its own lifecycle, and the link between them is a
pair of ids a human can read. A suspicion is not a trail.

The cost of the choice is stated plainly: an out-of-band edit to a frozen
section is **not detected by the tool**. It is caught by review, and the
child-task path is the sanctioned way to make the change.

## 10. Subtasks

`TSK03.1` is a **full task**: its own status, criteria, steps, evidence,
review and findings. It is not a checklist item on the parent.

| rule | |
|---|---|
| The parent lists the child in `children[]` | |
| The child names the parent in `parent` | |
| Progress is **derived** from the children's statuses | never stored, never aggregated automatically |
| The parent is not blocked by an open child | a refinement is not a debt of the original |

## 11. Storage

```
<project>/coding-lanes/work/
  TSK01-<slug>.json     # a task
  ITM01-<slug>.json     # a smaller task — see smaller-tasks.md
```

One folder holds every work unit. The id prefix says which is which
([ADR 0002](../decisions/0002-work-folder.md)). No central store: the data
lives in the project it describes, next to the code it is about.

| rule | |
|---|---|
| **One file per artifact** | one task per file |
| **The folder is the index** | there is no summary file and no manifest. Listing is reading the directory |
| **JSON, one object per file** | pretty-printed, one trailing newline |
| **Atomic writes** | the writer writes `<file>.json.tmp`, syncs it, and renames it over the target. A crash leaves the old file intact |
| **The file name is `<id>-<slug>.json`** | a rename writes a new file; the writer removes the old one, so one id never has two files |
| **Ids come from a scan** | the highest `TSK` number present plus one. Never `len()+1` |
| **The reader is tolerant of a broken file** | a file that does not parse is skipped, not fatal — one corrupt file does not hide the folder |

The store is **discovered**, not configured: from any path, walk up until a
`coding-lanes` folder appears.

## 12. Derived, never stored

| view | derived from |
|---|---|
| criterion status | the criterion's latest evidence |
| review state | `pending-review` · `open-findings` · `clean` — from the rounds and the findings |
| lanes | the human lane and the agent lane — from the status, the overlays and the claims (see [vocabulary.md](../vocabulary.md#the-lanes-are-derived)) |
| board views | the lanes, grouped and sorted |
| children progress | the children's own statuses |
| readiness to claim | would `start` be accepted right now, and if not, which gate refuses it |

A stored lane is a lane that can disagree with the status it was derived from.
These are views, and they are recomputed on read.

## 13. Worked examples

`mock-data/coding-lanes/work/` holds one task per lifecycle stage, plus the
edge cases. Every fixture satisfies the gates of the phase it is in.

| fixture | what it shows |
|---|---|
| `TSK01` | `draft`: title only |
| `TSK05` | a `blocked` overlay with a reason, a `by` and an `at` |
| `TSK07` | `approved`, and the parent of `TSK27` |
| `TSK10` | `executing`: claimed with a lease, evidence attached, a `deviation` on the step, a note |
| `TSK11` | a **deferred criterion**: `status` `deferred`, a `deferral` with a `follow_up` |
| `TSK13` | `code-review`: a `self` round and one open finding |
| `TSK16` | `remediating`: a `self` round (`ok`) then an `external` round (`changes-needed`) |
| `TSK19` | `needs-human`: five external rounds spent, work still outstanding |
| `TSK22` | `done`: verdict `ok`, no open findings |
| `TSK27` | a child task: `parent` set, still `draft` |

The dependency graph in the fixtures runs backwards only — a task depends on
tasks with lower ids — so the graph has no cycles and every `depends_on` points
at a task that exists. `TSK07` and `TSK08` wait on `TSK05`, which is blocked,
so they cannot start until it is unblocked.

`crates/core/tests/mock_data.rs` loads the set and checks that it covers every
phase and keeps every gate.

## 14. What this doc is not

| | |
|---|---|
| **Not talita's command surface** | the verbs, their flags and their output are not defined yet |
| **Not raul's screens** | the tabs, the columns and the key bindings are not defined yet |
| **Not the smaller-task model** | that is [smaller-tasks.md](smaller-tasks.md) |
| **Not the role model** | that is [roles/roles.md](../roles/roles.md) |
| **Not the journal** | `activity.json` and its event kinds are not defined yet |
| **Not the project config** | not defined yet |
| **Not autopilot** | the orchestrator role exists; what it drives does not |

## 15. Known gaps

**Not defined yet:**

- journal event kinds (`activity.json`) and its home in the work folder
- project config
- talita's operation surface
- raul's screens and views
- autopilot

**Specified here, not yet in `crates/core`:**

| gap | |
|---|---|
| `kind` and `promoted_from` on the task | the fields the small-task model needs |
| the small-task step exemption | `criteria.covered`, `steps.done_when_present` and `steps.settled` are applied to every task today |
| the claim lease | `lease_expires_at` is stored and never checked |
| the rename cleanup | `save` writes the new file name; the old file is not removed yet |
| the work folder path | fixed: the store reads `coding-lanes/work/`, as [ADR 0002](../../decisions/0002-work-folder.md) decides |
