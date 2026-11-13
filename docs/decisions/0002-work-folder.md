# 0002. One work folder

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

Every managed project carries its own task data. There is no central store: the
tasks live in the project they belong to, next to the code they are about, so that
a task can be read, diffed and committed with the work it describes.

That leaves a question with two plausible answers. The work in a project is not one
kind of thing. There are **tasks** (`TSK01-<slug>.json`) and **smaller tasks**
(`ITM01-<slug>.json`) — see [ADR 0003](0003-smaller-tasks.md) — and raul shows them
together in one backlog tab.

Two layouts follow from that:

```
coding-lanes/work/tasks/TSK01-x.json       coding-lanes/work/TSK01-x.json
coding-lanes/work/items/ITM01-y.json       coding-lanes/work/ITM01-y.json
```

The split is the layout a directory-tree instinct produces: one folder per type,
so a glob of one kind is a glob of one folder.

The counter-pressure is that raul shows the two kinds in one view, and that view is
the thing the human uses to decide what to do next. A split on disk puts a
distinction in front of the reader that the product has deliberately removed.

## Decision

**One folder holds every work unit.**

```
coding-lanes/work/
  TSK01-<slug>.json     # a task
  ITM01-<slug>.json     # a smaller task
```

| Rule | |
|---|---|
| One folder | `coding-lanes/work/`, inside the managed project |
| One file per artifact | one task or one smaller task per file |
| The id prefix says which | `TSK` is a task, `ITM` is a smaller task |
| No central store | each project carries its own data |
| No per-kind subfolders | the folder is flat |

The id prefix carries the distinction that a subfolder would otherwise carry, and
it carries it better: a file's kind is visible in any listing, in a commit, in a
diff and in a shell glob, without the path being parsed.

### The alternatives, and why they lost

| Option | Lost because |
|---|---|
| `work/tasks/` and `work/items/` | a split raul does not show. The human learns a disk distinction to read one list, and every tool has to join two roots to serve that list |
| `work/small/` and `work/tasks/`, keeping the word "small" on disk | "small" is a judgement about size that changes when the work is shaped. The prefix is neutral; the folder would not be |
| One folder per kind, but with subfolders per kind *and* per status | the tree grows a level per dimension, and status changes would move files. Files are records; they do not migrate because a field changed |
| A central store, one database per project | the data would leave the project it describes. A task is read, diffed and committed with the code; a central store breaks all three |
| A subfolder per project inside a central folder | a central folder is a central store with extra steps, and it gives the project a second home |

## Consequences

### Positive

- The reader has one place to look, and one convention to learn.
- raul's backlog tab reads one folder, and so does every other reader.
- The id prefix does the kind's work, so the kind is never ambiguous in a path, a
  log line or a diff.
- A project is portable: the folder moves with it.

### Negative

- **The folder grows mixed.** A long-lived project holds tasks and smaller tasks
  side by side, and a reader listing it sees both. The prefix is the only signal.
- **A glob for one kind is now a glob with a filter** (`ITM*`), not a path. Any
  tooling that wants one kind must filter on the name.
- **No natural place for other artifacts** — journal, project config — without
  adding to the same folder or adding a sibling. We will decide that separately
  when those artifacts exist.

### Follow-up

- The activity journal, project config and any other non-task artifact will need
  a home. This ADR does not decide it; the folder stays flat until then.
