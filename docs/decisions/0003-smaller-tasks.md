# 0003. Smaller tasks — one artifact, three kinds

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

Projects collect work that is not yet work: an idea, a defect someone reported, a
small feature that is not worth a task file yet. raul shows these in a backlog tab,
next to nothing else — one list for everything that is not a task.

The obvious build is one artifact per kind: an *idea* file, a *bug* file, a *feature*
file. Each gets its own id prefix, its own file shape, its own verbs. The obvious
cost is that a hunch becomes a bug report becomes a small feature, and each of those
transitions is a **migration between file types** — a new file, a new id, a broken
trail back to the thought it came from.

The opposite build is to fold all of it into the task itself: one artifact, with a
field saying what it is. That makes a task file carry fields that are meaningless
for a task (a bug's reproduction steps) and fields that are missing for a hunch (the
acceptance criteria).

The failure to avoid is the one this project exists to hold: **two nouns for one
list**, where the records, the CLI verbs and the display drift apart.

## Decision

**One artifact with a `kind`.**

| | |
|---|---|
| The artifact | a **smaller task**, id `ITM01`, file `ITM01-<slug>.json` |
| Its kinds | `idea` · `feature` · `bug` |
| Its fields | title, body, scope, kind, tags, status, created, updated, promoted_to, dismissed_reason |
| Its groups in raul | **Ideas** (needs shaping) and **Small tasks** (ready to work) — **derived** from the kind, never stored |

### The id is kind-neutral

`ITM01` stays `ITM01` when an idea is shaped into a small task. The prefix says
*this is not a task yet*; it says nothing about what the item is. A kind-specific
prefix (`IDEA01`, `BUG01`) would force an id change at exactly the moment the
trail matters most, and the old id would have to be carried in a `supersedes` field
to stay reachable.

### Shaping is an update

`idea → feature|bug`, plus scope. The id does not change, no file is rewritten
under a new name, and the item's history stays in one place.

### Promotion is one-way and records its target

A promoted item becomes a task in `draft`, carrying its title, body, scope, kind
and a `promoted_from` pointer back to the item. The item records the task it became
in `promoted_to`.

| Direction | |
|---|---|
| smaller task → task | yes, once |
| task → smaller task | no |

The item is written before the task is created, so a crash cannot produce two tasks
from one item. A task is a record of work that was approved; shrinking it back into
an item would delete an approval.

### An idea may not be promoted

An idea carries no acceptance criteria, and a task without them can never be
signed off. The refusal names the remedy: **shape it into a small task first**.

### Groups are derived

The groups raul shows — Ideas, Small tasks — come from the kind. They are not
fields, and they are not folders. A group that is stored is a group that can
disagree with the kind it was derived from.

### The alternatives, and why they lost

| Option | Lost because |
|---|---|
| One file type per kind — idea, bug, feature | three nouns for one list; three file shapes to keep in step; an id change on every shaping |
| Kind-specific id prefixes | the id is the trail. Changing it when the kind changes breaks the link back to the thought that became work |
| Folding the kind into the task | a task with fields that mean nothing for a hunch, and a hunch with fields that are missing for a task |
| Shaping as a new file plus a pointer | two records to keep consistent, and a crash between them loses the link |
| Promotion in both directions | a task holds an approval; a reversible promotion means a task that was never approved |
| Storing the groups | derived, or stale |
| Letting an idea be promoted straight to a task | a task with no acceptance criteria can never be signed off. The refusal names the remedy, so the hole is closed rather than tolerated |

## Consequences

### Positive

- One artifact, one file shape, one set of verbs for everything that is not a task
  yet.
- The id survives shaping, so the trail from thought to work is unbroken.
- The groups the human sees are derived, so they cannot drift from the kind.
- Promotion is auditable: the item points at the task, and the task points back.

### Negative

- **One artifact carries fields that only some kinds need.** A bug's reproduction
  steps and an idea's scope are both "body" or "scope" in free text, not typed
  fields. We accept loose fields rather than one shape per kind.
- **The kind is a string that must be understood by every reader.** A tool that
  does not know `feature` must still round-trip the item.
- **Promotion is not undoable.** An item promoted by mistake is dismissed, not
  un-promoted.

### Follow-up

- A promoted item's task carries exactly one acceptance criterion, with an empty
  verification. What the gates then require of it is decided in
  [ADR 0004](0004-lighter-gates.md).
