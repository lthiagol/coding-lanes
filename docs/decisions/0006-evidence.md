# 0006. Evidence — commands, exit codes, and derived status

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

An acceptance criterion says what must be true and how it is verified. The
verification is the part that can go wrong: a criterion can be *passed* on a
sentence that describes a test run, and the sentence can be wrong, or describe
nothing, or describe a run that matched zero tests.

The failure mode to avoid is **manufactured confidence** — work that closes with
no proof, or worse, with a proof that was never checked.

Two questions follow from that:

1. What is evidence? A thing a reader can check, or a thing an agent says?
2. Where does a criterion's status come from — the agent that set it, or the
   evidence attached to it?

If the status is a field the agent writes, then the status is the agent's claim
and the evidence is decoration. If the status is derived, the agent cannot pass a
criterion by asserting it, and a reader can recompute the status from the record.

There is also a case for overriding a derived status: a criterion may be true for
a reason no command captures. That case is real, and it is exactly the case where
a silent override does the most damage.

## Decision

### Evidence is commands and exit codes

```json
{ "command": "cargo test", "exit_code": 0, "summary": "3 passed",
  "at": "2026-11-13T12:00:00Z", "by": "runner-1", "session": "s-42" }
```

| Rule | |
|---|---|
| What counts | a command, its exit code, and enough summary to read it |
| What does not | prose about what was tested |
| Who runs it | the runner, in its session, before the task advances |
| Who reads it | the reviewer, and the human in raul |

### The criterion's status is derived from the latest evidence

| Evidence | Status |
|---|---|
| none | `pending` |
| exit code 0, non-empty summary | `passed` |
| exit code non-zero | `failed` |

The status is **never written**. It is computed from the criterion's evidence
whenever it is read, so a reader and the tool cannot disagree about it.

### A manual override is recorded as evidence

An agent may override the derived status. The override is itself an evidence
entry, with the same shape, marked as a manual override and attributed to the
actor and session that made it.

**A bypass is explicit debt.** The criterion reads as passed, but the record shows
that it was passed by hand and not by a command. It is visible to the reviewer and
to the human, and it is not a silent hole in the gate.

### Evidence is append-only

Re-running appends a new entry. It never overwrites. The latest entry decides the
status; the earlier ones stay as the trail.

## Consequences

### Positive

- A criterion cannot be passed by asserting it. The status follows from a record.
- The history is trustworthy: an evidence entry is never rewritten.
- An override is visible. The debt is on the record, not in a conversation.
- The evidence is machine-checkable in the way that matters — an exit code.

### Negative

- **Not everything is a command.** A criterion whose truth is a judgement or a
  design decision has no exit code. It can only be overridden by hand, which makes
  explicit debt the normal case for that kind of criterion. Accept it: such a
  criterion should be rare, and its override is visible.
- **The derived status is computed on read.** Every reader of a task recomputes
  it, and a reader that reimplements the rule can get it wrong. The rule is one
  line and must live in one place.
- **Append-only evidence grows.** A task run ten times carries ten entries. The
  summary exists to keep that readable.

### Follow-up

- The evidence's shape and the derived statuses are listed in
  [tasks/task-model.md](../tasks/task-model.md#evidence).
- What talita does with a verification command — whether it runs it or accepts the
  runner's report of a run — is part of the operation surface, which is not
  decided yet.
