# 0004. Lighter gates for small tasks

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

A task that is a bug or a small feature carries the same anatomy as a task that is
an epic: criteria, steps, gates, a review. For a one-line fix, that anatomy is
mostly ceremony, and ceremony is where work goes to die.

But the gates are not decoration. Each one exists because a specific way of getting
it wrong was common enough to name:

| Gate | The mistake it prevents |
|---|---|
| Every criterion covered by a step | work done that nobody checked against what was asked |
| Every step has a `done_when` | a step that nobody can tell is finished |
| Every step settled before review | code going to review while half of it is still in flight |
| A self-review round before review | obviously wrong work reaching a reviewer |
| A verdict before done | work declared done without anyone looking |

The question is which of these are load-bearing for a small task, and which are
cost without benefit.

Two positions are available:

- **Weaken nothing.** A small task goes through exactly the same gates as any other
  task. The cost of a small task is then the same as the cost of a task, which is
  the cost people avoid tasks over.
- **A light path.** Small tasks get a separate, cheaper track with fewer gates.
  The cost is a second lifecycle to design, a second set of gates to enforce, and
  a second way for work to close — and work that closes without proof is exactly
  the hole this project exists to avoid.

## Decision

**A small task keeps the full lifecycle and every gate, except the step rules.**

| Rule | Applies to a small task |
|---|---|
| Criterion needs a covering step | **no** |
| Step needs a `done_when` | **no** |
| Step must be settled before code review | **no** |
| Acceptance criteria required | yes |
| Out-of-scope entries required | yes |
| Self-review round before code review | yes |
| External review round and verdict | yes |
| Evidence attached | yes |
| No open findings before done | yes |
| Session boundaries between execution, remediation and review | yes |

The step rules are the ones waived, and only those three:

- no acceptance criterion needs a covering step
- no step needs a `done_when`
- no step needs to be settled before code review

Steps remain available — a runner may still write them — they are simply not
required, and a step that exists is still honoured.

### Why the step rules and not the others

The step rules are about **how the work is organised**, not about **whether the work
is checked**. A small task can be executed in one move; requiring the runner to
first write a plan for one move, and then to prove the plan was followed, adds no
confidence that the outcome is correct.

Every other gate is about whether the outcome is **known**:

- Criteria and out-of-scope entries say what was asked and what was not, so the
  reviewer can tell whether the change did the right thing.
- Evidence says the change works, in commands and exit codes.
- The self-review catches the obvious before an independent reviewer spends a round
  on it.
- The external verdict is an independent judgement, and findings must be closed
  before the task is done.

**The review stays.** A small task still gets a reviewer, and the reviewer still
never codes ([ADR 0005](0005-roles.md)).

### The alternatives, and why they lost

| Option | Lost because |
|---|---|
| **master-plan's track-like light path** — small work takes a different, cheaper track | a second lifecycle, a second gate set and a second close path. The light path is also where "it was small" quietly becomes "it was not checked". A single task with lighter step rules gets the cost reduction without the second anatomy |
| **maitre's "no gate is weakened at all"** | a one-line bug fix would carry the same ceremony as an epic, and the incentive to avoid tasks entirely grows with the ceremony. The step rules cost time and buy no confidence for small work |
| Waive the review for small tasks | a review is the only independent judgement in the model. Waiving it leaves work that closes with no proof |
| Waive evidence for small tasks | "the fix works" as prose is not evidence; commands and exit codes are ([ADR 0006](0006-evidence.md)) |
| Waive the criteria for small tasks | a task with no acceptance criteria can never be signed off. This is the same hole that blocks an idea's promotion ([ADR 0003](0003-smaller-tasks.md)) |
| Waive the self-review | the self-review is cheap and catches the obvious before an independent reviewer spends a round on it |
| Two gate sets, chosen by the creator at write time | the choice becomes a way to skip gates, and every reader has to check which set applies |

## Consequences

### Positive

- Small work costs less without being less guarded.
- One lifecycle. One set of states, one set of gates, one close path.
- The waiver is three rules long, and they are about the plan, not the outcome.

### Negative

- **A small task can reach review with no steps at all**, so there is no written
  record of how the work was done. The evidence and the diff carry that instead.
- **The waiver is a condition every gate must implement.** Each rule needs to know
  whether the task is small, and that check lives in one place so it cannot be
  applied unevenly.
- **"Small" is a fact of the kind, not a judgement of the reviewer.** A task
  declared `feature` or `bug` gets the lighter path whatever its size turns out to
  be; the reviewer's job is to find that out.

### Follow-up

- The gates that do apply are listed in
  [tasks/task-model.md](../tasks/task-model.md#transitions), which marks each
  transition with the step rules as not applying to a small task.
- If small tasks turn out to be under-specified in practice, the remedy is more
  criteria or a better reviewer, not fewer gates.
