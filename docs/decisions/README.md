# Decision records

One file per decision, `NNNN-short-slug.md`, numbered from `0001`.

## Should this be an ADR?

Four questions. All four must be yes.

| # | Question |
|--:|---|
| 1 | **Hard to reverse?** Would changing it later cost meaningfully? |
| 2 | **Surprising without context?** Will a reader look at the code and wonder *why on earth*? |
| 3 | **A real trade-off?** Were there genuine alternatives, picked for specific reasons? |
| 4 | **Does a reader need the reasoning, or just the decision?** |

Question 4 is the one that separates an ADR from a reference doc. If the decision is
obvious once stated, it belongs in a reference document — a glossary entry, a README
section — not here.

**Fails 1?** It is cheap to reverse: skip the ADR and just reverse it.
**Fails 2?** Nobody will wonder, so there is nothing to record.
**Fails 3?** There was no real alternative, so there is nothing to say beyond "we did the
obvious thing".

## Format

```markdown
# NNNN. <Title>

- **Status:** proposed | accepted | superseded by NNNN
- **Date:** YYYY-MM-DD
- **Deciders:** <who>

## Context
<the forces at play — constraints, evidence>

## Decision
<what we are doing>

## Consequences
### Positive
### Negative
### Follow-up
```

Length is not a virtue — a decision that needs three sections gets three sections. But an
ADR is a *record*, not a breadcrumb: it carries the reasoning and the evidence, because
the reasoning is what survives.

**Say it once.** If a fact belongs to a glossary, a reference doc or the code, the ADR
points at it rather than repeating it. The vocabulary of record is
[docs/vocabulary.md](../vocabulary.md).

## Index

| # | Title | Status |
|---|---|---|
| [0001](0001-naming.md) | Naming — the project, the two interfaces, the binaries | ✅ accepted |
| [0002](0002-work-folder.md) | One work folder | ✅ accepted |
| [0003](0003-smaller-tasks.md) | Smaller tasks — one artifact, three kinds | ✅ accepted |
| [0004](0004-lighter-gates.md) | Lighter gates for small tasks | ✅ accepted |
| [0005](0005-roles.md) | Roles — agent roles only | ✅ accepted |
| [0006](0006-evidence.md) | Evidence — commands, exit codes, and derived status | ✅ accepted |
| [0007](0007-one-writer.md) | One writer — talita writes, raul reads | ✅ accepted |
| [0008](0008-session-boundaries.md) | Session boundaries are structural | ✅ accepted |
| [0009](0009-markdown-projection.md) | JSON for agents, markdown for humans — the record and the display | ✅ accepted |
