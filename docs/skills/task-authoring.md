---
name: task-authoring
description: Author a task in coding-lanes through talita, the CLI tool. Use when the user asks to write down a task, groom or approve a task, break work into tasks, or turn an idea into a task.
---

# Task authoring

You are the **creator**. You write the task; you never execute it and never
review it. The output of a session is one task file, written by talita.

## What a task is

One unit of work. One JSON file. One lifecycle.

```
<project>/coding-lanes/work/TSK01-<slug>.json
```

| rule | |
|---|---|
| The folder is the index | there is no summary file and no manifest. Listing is reading the directory |
| The store is discovered, not configured | from any path, talita walks up until a `coding-lanes` folder appears |
| Writes are atomic | talita writes `<file>.json.tmp`, syncs, renames it over the target |
| One id, one file | the file name is `<id>-<slug>.json`; a rename writes a new file and removes the old |

The file holds one JSON object per task. Its sections:

| section | holds |
|---|---|
| `intent` | `outcome` — what must be true when the task is done; `problem` — why it is worth doing |
| `scope` | `in_scope[]`, `out_of_scope[]` |
| `acceptance_criteria[]` | what must be true and how it is verified |
| `steps[]` | the actions that get there |
| `review` | rounds, verdict, findings |
| `questions[]`, `notes[]` | what is undecided, what was learned |
| `meta` | `priority`, `effort`, `tags[]` |

## Identifiers

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

Ids are allocated by scanning what exists — the highest plain `TSK` number in the
folder plus one, zero-padded to two digits. Never reuse a deleted task's id.
Child ids (`TSK03.1`) are not plain numbers and are skipped by the scan.

## Statuses and overlays

Eight statuses, one at a time. Overlays are orthogonal and are **not** states.

| status | meaning |
|---|---|
| `draft` | being written. Intent may be incomplete |
| `groomed` | outcome, criteria and out-of-scope written; open questions still allowed |
| `approved` | locked. Every criterion covered, every step has a `done_when` |
| `executing` | claimed and being implemented |
| `code-review` | work done, criteria proven or failed with a reason, evidence attached, self-review filed |
| `remediating` | external findings open; being fixed |
| `needs-human` | the review budget is spent and work is outstanding. A human decides |
| `done` | reviewed clean. Terminal until reopened |

| overlay | fields | notes |
|---|---|---|
| `blocked` | `reason`, `by`, `at` | something outside the task's control stops it |
| `deferred` | `reason`, `by`, `at` | parked on purpose. Re-openable |
| `cancelled` | `reason`, `by`, `at` | terminal by overlay. Refuses every transition |

Any actor may set an overlay; a reason is always required.

## The order of writing

Write in this order. Each step feeds the next, and the gates refuse anything
skipped.

| # | step | what it writes | gate it must pass |
|--:|---|---|---|
| 1 | **intent** | `intent.outcome`, `intent.problem` | `intent.outcome_present` — outcome is non-empty |
| 2 | **scope** | `scope.in_scope[]`, `scope.out_of_scope[]` | `scope.out_of_scope_declared` — ≥1 out-of-scope entry |
| 3 | **acceptance criteria** | `acceptance_criteria[]`, each with `description` and `verification` | `criteria.present` — ≥1 criterion |
| 4 | **steps** | `steps[]`, each with `action`, `done_when`, and `covers[]` naming the criteria it advances | `criteria.covered` — every criterion has ≥1 covering step; `steps.done_when_present` — every step has a `done_when` |
| 5 | **approval** | the status moves to `approved` | `questions.resolved` — no open question |

Grooming moves `draft → groomed` (steps 1–3). Approval moves `groomed →
approved` (steps 4–5).

| rule | |
|---|---|
| A criterion is what must be true | a step is one way of making it true. `covers[]` is the line between them |
| `verification` is a command or a reproducible procedure | **not prose**. The runner attaches the command and its exit code as evidence |
| `done_when` is observable | how you will know it worked |
| An open question blocks approval | raise it as a `QST-nn`, answer it, keep the answer |
| `out_of_scope` is mandatory | it is how a reader knows the boundary of the task |

## The gates

A transition that can happen without its guards has no guards. The gates live
in `crates/core/src/lifecycle.rs`; the transitions run them internally, so a
transition and its guards cannot be applied apart.

| gate | checks | applies to |
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

`review.independent` is why `planned_by` is recorded on the task: the session
that planned a task never reviews it.

## Never hand-edit a task file

**talita is the only writer.** Not you, not an editor, not a script that patches
JSON.

| why | |
|---|---|
| One writer | talita writes, raul reads. Two writers to one file is a merge conflict with no referee |
| Gates travel with the write | a transition and its guards cannot be applied apart when the edit bypasses talita |
| Frozen sections stay frozen | approval freezes `intent`, `scope` and the criteria. A refinement becomes a child task, not an in-place edit |

A material change after approval is a **child task** — `TSK03.1` — with its
own brief, criteria, evidence and review. The parent keeps the record of the
intent it was approved against; the child carries the change.

## Evidence

Evidence is **commands and exit codes**, not prose. It is the only thing that
proves a criterion.

```json
{ "command": "cargo test --workspace", "exit_code": 0, "summary": "10 passed; 0 failed",
  "at": "2026-11-10T12:00Z", "by": "runner-01", "session": "session-runner-01" }
```

| rule | detail |
|---|---|
| Attached, never prose | `{command, exit_code, summary, at, by, session}` |
| Append-only | re-running appends. It never overwrites |
| Exit code is the truth | `exit 0` with a non-empty summary is a pass; any other exit code is a failure; no exit code at all is `unproven` |
| Status is derived | a criterion's status is the outcome of its **latest** evidence. No evidence ⇒ `pending` |
| A failure needs a reason | `failure_reason` is required when a criterion is `failed` |
| A manual override is evidence | an agent may override a derived status, and the override is recorded as an evidence entry. A bypass is explicit debt, never a silent one |
| Evidence is per criterion | the command proves one thing; it does not prove the next criterion |

`exit 0` with an **empty** summary is a failure: a command that ran and said
nothing has proved nothing.

**Write criteria so the runner can prove them.** A criterion whose
`verification` is a sentence is a criterion nobody can evidence, and the task
stops at `code-review`.

## The review budget

`review.max_rounds` is `5` and counts **external** rounds. Self-review is a
pre-screen and does not consume it.

Five external rounds with work still outstanding and the task is `needs-human`.
The task does not review itself into completion.

For the creator this means: a brief that leaves the runner guessing burns the
budget. Say what must be true, how it is verified, and what is out of scope —
in the first two minutes of the task's life, not the fifth review round.

## Small tasks

A task whose `kind` is `bug` or `feature` is small. It keeps the full
lifecycle and every gate, **except the step rules**:

- no acceptance criterion needs a covering step
- no step needs a `done_when`
- no step needs to be settled before code review

The review stays. A small task still needs a self-review round before it goes
to code review, and a verdict before it is done.

Work that is not a task yet is a **smaller task** — `ITM01` — with a `kind` of
`idea`, `feature` or `bug`, and a status of `open`, `dismissed` or `promoted`.

| rule | |
|---|---|
| The kind is the only truth | the groups the human sees are derived from it, never stored |
| The id is kind-neutral | `ITM01` stays `ITM01` when an idea is shaped into a small task |
| Shaping is an update | `idea → feature\|bug`, plus a scope. The id does not change |
| An idea may not be promoted | it carries no acceptance criteria, and a task without them can never be signed off. Shape it into a small task first |
| Promotion is one-way | the item records the task it became in `promoted_to` |
| A promoted item becomes a task in `draft` | with exactly one criterion, `promoted_from` set, and no gate weakened |

## The CLI surface

**Only three verbs exist today.** talita's full operation surface is not
defined yet (`docs/tasks/task-model.md` §15). The verbs below marked *not
implemented yet* are the ones we will use; the marked ones are not in the
binary and cannot be run.

| verb | does | implemented |
|---|---|---|
| `talita create "<title>"` | create a task in `draft`; prints the task JSON | yes |
| `talita list` | list tasks as `id  status  title` | yes |
| `talita show <id>` | print one task as JSON | yes |
| `talita groom <id> --outcome ... --problem ... --out-of-scope ... --criterion ...` | write `intent`, `scope` and the criteria, then `draft → groomed` | not implemented yet |
| `talita add-step <id> --action ... --done-when ... --covers <ACCEPT-nn>` | add a step; `--covers` names the criteria it advances | not implemented yet |
| `talita approve <id>` | `groomed → approved` | not implemented yet |
| `talita ask <id> "<question>"` / `talita answer <id> <QST-nn> "<answer>"` | raise and resolve a question | not implemented yet |
| `talita set-meta <id> --priority ... --effort ... --tags ...` | write `meta` | not implemented yet |
| `talita reopen <id>` | `done → executing` | not implemented yet |
| `talita resume <id>` | `needs-human → executing`, when a human instructed it | not implemented yet |
| `talita block <id> --reason ...` / `defer` / `cancel` / `unblock` / `undefer` | set and clear an overlay; a reason is always required | not implemented yet |

Until the missing verbs land, do not write task files by hand. Say what is
missing, and let the human decide how to proceed.

## Writing a task, end to end

1. **Talk to the human.** The creator is human-dependent: the content of the
   task comes out of an interactive session. Ask what must be true, what it
   costs, and what is deliberately not in scope.
2. **Draft the intent.** One sentence for `outcome`, one for `problem`.
3. **Write the scope.** At least one `out_of_scope` entry. If you cannot name
   one, the task is not understood yet.
4. **Write the criteria.** One per observable outcome. `verification` is a
   command or a reproducible procedure.
5. **Write the steps.** One way of making each criterion true. `covers[]` ties
   each step to its criteria; `done_when` is observable.
6. **Close the questions.** Resolve or delete every `QST-nn`. An open question
   refuses approval.
7. **Approve.** After this, `intent`, `scope` and the criteria are frozen.

## Not yours

| | |
|---|---|
| Executing a task | that is the `runner` |
| Reviewing a task | that is the `reviewer` — and never in the session that planned it |
| Writing the runner's steps | the runner may add, split or drop steps while executing, and records the difference as a `deviation` |
