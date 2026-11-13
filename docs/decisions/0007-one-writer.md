# 0007. One writer — talita writes, raul reads

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

Two surfaces act on the same records. talita is the agent-facing CLI; raul is the
human-facing TUI. The obvious symmetry is that both may write: raul lets the human
approve, reopen, defer or cancel a task from the screen.

The cost of that symmetry is that **the record's shape is then enforced by two
implementations**. talita enforces the file format, the id rules, the status
vocabulary and the transition gates because the agent using it is fast and literal.
raul would have to enforce the same rules for the human, or the human's edits would
produce files the CLI refuses to read — and a file the CLI refuses to read is a
silent loss of work.

The second cost is a **trust boundary**. A task file is written by an agent that
was told to write it. If the human's editor is also a writer, then the file may
carry content nobody gated. The gates that matter — approval, evidence, findings —
are gates on a record, and a record with two unconnected writers has no single
path by which a gate was applied.

The third cost is the one the interface split is for. talita's job is to be right
about format and protocol, because agents drive it. raul's job is to be right about
**reading** — showing what is waiting, on whom, and what the human is being asked
to decide. A read-only raul cannot drift from the model, because it has no model of
its own.

## Decision

**talita writes. raul reads.**

| | talita | raul |
|---|---|---|
| Writes task data | yes | **no** |
| Reads task data | yes | yes |
| Enforces format, ids, statuses, gates | yes | no — it renders what talita wrote |
| The human's route to a change | run talita | see what raul shows, then run talita |

### The task file is the channel

Agents and the human do not message each other. The task file is the blackboard:
the runner writes the claim and the evidence, the reviewer writes the rounds and
the findings, the next agent reads the file and sees what changed. A human reads
the same file through raul.

This closes the gap the human has in a CLI-only tool. If raul were read-only with
no way to act, the human would fall back to editing JSON in an editor — which is
the second writer we just ruled out, arriving through the back door. So raul must
offer the human a **route to talita**: the human's actions are performed by talita,
with talita's rules applied.

### Direct file edits are discouraged

The file format is a protocol between talita and the files. Hand-edits are allowed
to be read — a human may open a task file to see the whole record — but they are
discouraged as a way to make a change, because an edit that bypasses talita's
gates produces a record the domain did not sanction.

## Consequences

### Positive

- One implementation enforces the format, the ids, the statuses and the gates.
- raul cannot drift from the model: it renders, it does not decide.
- Every change to a task passes through one path, so the audit trail is one trail.
- talita stays the place where protocol lives, which is what an agent needs from
  it.

### Negative

- **The human is slower.** Every action is a talita invocation, including the
  approvals that feel like they belong on the screen. raul must therefore make
  those invocations cheap to issue and obvious to issue.
- **A read-only raul cannot offer inline editing.** An editor inside raul would be
  a second writer. We accept that raul shows and talita acts.
- **Hand-edited files are still possible.** Nothing stops an editor. The
  consequence is that talita must refuse or repair what it cannot vouch for,
  rather than trust it.

### Follow-up

- The operation surface that raul calls into is not decided yet. When it is, it
  will be talita's operations, not a second implementation of them.
- Journal event kinds — how talita records what it did — are still open.
