# 0009. JSON for agents, markdown for humans

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

A task is one record with two audiences. **Agents** read and write it, and
they need it to be exact: fields, types, ids, statuses — something a program can
check and a gate can enforce. **Humans** read it in raul, and they need it to be
readable: headings, prose, tables, the commands that were run.

The two pull in opposite directions. JSON serves the first and reads like a
dump to the second. Markdown serves the second and is a poor record for the
first — prose hides the field names, and "the tests pass" is not a status.

The markdown renderer that already exists in this ecosystem is
[leaf](https://github.com/rivolink/leaf): MIT, Rust, ratatui 0.30 and crossterm
0.29 — the same versions raul uses — with tables, callouts, footnotes,
syntax highlighting, LaTeX and mermaid. It is a **binary**: it has no lib
target, so it cannot be depended on as a crate.

## Decision

**The record stays JSON. The display is markdown.**

| layer | what it is | where |
|---|---|---|
| record | the task file, JSON — exact, typed, gated | `coding-lanes/work/TSK*.json` |
| projection | the task → a markdown document: headings per section, tables for the criteria and their evidence, lists for the steps and findings, with the free-text fields embedded as markdown in their own right | `crates/core` — pure `task → markdown`, no terminal involved |
| rendering | markdown → ratatui lines, via leaf's engine | `crates/engine` — leaf's engine, vendored |
| surface | the document in the detail pane | `crates/raul` |

**leaf's engine is vendored into `crates/engine`**, with the upstream commit
recorded and a `VENDORED.md` listing every deviation — the shape
[Frond](https://github.com/lthiagol/Frond) already uses for the same engine.

The projection lives in `core` rather than in raul because it is pure: it can be
tested without a terminal, and it is the piece other surfaces will need.

## Consequences

### Positive

- One record, no second copy. The markdown is a view of the JSON, never a file
  that can drift from it.
- The agents keep a machine-readable surface with fields and gates.
- The human gets a document: prose where prose belongs, tables where counts
  belong, the verification commands as readable code.
- The engine arrives already proven: the same one, already vendored, in the
  user's own other project.

### Negative

- The projection is a second piece of software that must agree with the model.
  A field that changes shape has to change the projection too.
- Vendoring carries the maintenance cost: the upstream copy has to be
  re-synced, and the deviations have to be kept honest.
- A task's free text is rendered as markdown, so a stray `#` or a fence in a
  title changes the rendering. Fields that must not be interpreted are escaped.

### Follow-up

- The markdown view is a unit of its own, after the shell and the Overview.
- `crates/engine` needs a sync script and a `VENDORED.md`, the way
  `.agents/skills/` does for its vendored skills.
- The projection needs a decision on escaping: which fields are rendered as
  markdown, and which are escaped.
