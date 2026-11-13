# Vocabulary — the decision of record

coding-lanes has two interfaces, two artifact kinds, four agent roles and one set
of lifecycle states. This doc fixes the words. The reasoning behind each choice
is in [decisions/](decisions/); where a doc and an ADR differ, this doc wins.

## The rule

**The lane names the place and the views over it. Everything else — the
interfaces, the machine's records, the agents' jobs — takes plain words.**

| Layer | Vocabulary | Examples |
|---|---|---|
| The place and its views | lane | the **lane**, the human lane, the agent lane, the backlog tab, the board |
| The interfaces | plain words behind a coined name | **talita**, **raul** |
| The records | plain | **task**, **smaller task**, acceptance criterion, step, **evidence**, finding, comment, note, question |
| The jobs | plain | **creator**, **runner**, **reviewer**, **orchestrator** |
| The states | plain | `draft` `groomed` `approved` `executing` `code-review` `remediating` `needs-human` `done`; overlays `blocked` `deferred` `cancelled` |

Two consequences that are rules, not accidents:

- A row shows the state word exactly as it is stored (`code-review`). There is no
  second word for a state and no translation between what talita prints and what
  raul shows.
- **Lane** is a place and a grouping, never a state. A task sits in the human
  lane or the agent lane; it is not "in the lane" the way it is "executing".

**Evidence** is commands and exit codes, not prose. A criterion's status is
derived from its latest evidence.

## The names

| Thing | Name | Note |
|---|---|---|
| The project | **coding-lanes** | *An agentic todo list with superpowers* |
| The CLI tool | **talita** | agent-facing; writes and manages tasks. **T**ask **A**ccess and **L**ane **I**nteraction **T**erminal **A**pp |
| The TUI | **raul** | human-facing; shows tasks. **R**eview, **A**pprove, **U**nderstand **L**ane |
| The CLI binary | `talita` | package `coding-lanes-talita`, crate `crates/talita` |
| The TUI binary | `raul` | package `coding-lanes-raul`, crate `crates/raul` |
| The work folder | `coding-lanes/work/` | inside each managed project, next to its code. No central store |
| A task | **task**, id `TSK01` | the unit of work. Children `TSK03.1` |
| A smaller task | **smaller task**, id `ITM01` | an idea, a bug or a small feature that is not a task yet |
| A criterion | acceptance criterion, id `ACCEPT-01` | |
| A step | step, id `STP-01` | |
| A question | question, id `QST-01` | |
| A finding | finding, id `FIND-01` | |
| A comment | comment, id `CMT-01` | |
| A note | note, id `NOTE-01` | |

The two interfaces share one project name and take their own. `talita` names
what the agent reaches for; `raul` names what the human reads.

## The backlog: one artifact, three kinds

The work that is not a task yet is **one artifact** with a `kind`.

| | |
|---|---|
| The artifact | a **smaller task**, id `ITM01`, in `coding-lanes/work/` |
| Its kinds | `idea` · `feature` · `bug` |
| Its groups in raul | **Ideas** (needs shaping) and **Small tasks** (ready to work) — **derived** from the kind, never stored |
| Its fields | title, body, scope, kind, tags, status, created, updated, promoted_to, dismissed_reason |
| Shaping | an **update**: `idea → feature\|bug`, plus scope. The id does not change |
| Promotion | **one-way**. The item records the task it became in `promoted_to` |

An **idea** is not promoted: it carries no acceptance criteria, and a task
without them can never be signed off. The refusal names the remedy — shape it
into a small task first.

One artifact because two nouns for one list is where drift comes from. The
groups the human sees need two file types to exist, and they do not.

### How the loose end becomes work

```
idea ──shaped into──▶ a small task  (kind: idea → feature|bug, plus scope)
small task ──promoted──▶ a task with ONE criterion
```

The last line matters: **a smaller task is too small to be a full task, not too
small to be guarded.** It becomes a task with a single criterion, so every gate
still applies — see [decisions/0004-lighter-gates.md](decisions/0004-lighter-gates.md).

## The lanes are derived

Lanes are **views**, never stored. A lane groups tasks by **who owns the next
action**:

| Lane | Next action belongs to |
|---|---|
| The human lane | the human — approve, decide, unblock |
| The agent lane | an agent — claim, execute, review, remediate |

A task's lane is computed from its status, its overlays and its claims. It is
never a field, because a stored lane is a lane that can disagree with the
status it was derived from.

## What is strings only

| | |
|---|---|
| raul's tab labels | the backlog tab, the lane headers — **Human lane**, **Agent lane** |
| raul's group headings | **Ideas**, **Small tasks** — derived from the kind |
| The window title | *coding-lanes* |
| talita's help | `talita — the coding-lanes CLI` |
| The state words | shown as stored: `code-review`, `needs-human`, `done` |
| The id prefixes | shown as stored: `TSK01`, `ITM01`, `ACCEPT-01` |

Display strings are not persisted. State words and ids are persisted **and**
shown unchanged, so there is one word per state and one spelling per id.

## The alternatives, and why they lost

| Option | Lost because |
|---|---|
| Calling the CLI "the CLI tool" and the TUI "the TUI" | two surfaces with no names of their own; the human has no word for the tool they read, and the agent has none for the tool it writes. Both are named now: **talita** and **raul** |
| Naming the CLI after the lane alone, and the TUI after the role | `talita` and `raul` are already that: *Lane Interaction* and *Understand Lane*. Naming them by role again would put a job where a surface belongs |
| Splitting tasks and smaller tasks into two folders | raul shows them in one backlog tab; a split on disk is a split a reader has to learn for no gain |
| Giving a smaller task a light path with no review gate | work that closes with no proof — the hole this project exists to avoid |
| Folding the kind into the task instead of a separate artifact | an idea, a bug and a task are not the same shape; folding them means either a task with empty fields or a second artifact wearing the task's name |
| Storing the lanes | a stored lane can disagree with the status it was derived from. The lanes are views |
| Giving the human a role | humans carry no role. The roles are the jobs agents do; a human is an actor of kind `human` |
