# raul — screens

The TUI of coding-lanes. It reads the work folder and shows it. It writes
nothing.

The domain is in [tasks/task-model.md](../tasks/task-model.md); the names are
in [vocabulary.md](../vocabulary.md); the roles are in
[roles/roles.md](../roles/roles.md).

## The window

```
┌────────────────────────────────────────────────────────────────┐
│ Overview   Needs you   Path   Backlog   Autopilot   Settings   │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  content pane — rows of the active tab                         │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│ q quit   tab next   enter open   ? help                         │
└────────────────────────────────────────────────────────────────┘
```

| part | behaviour |
|---|---|
| tab bar | the six tabs; `1`–`6`, `←`/`→`, `h`/`l`, `tab`/`shift+tab` |
| content pane | the active tab's rows; `↑`/`↓`, `j`/`k`, `pgup`/`pgdn`, `g`/`G` |
| detail pane | `enter` on a row opens the task; split or full screen; `esc` returns |
| footer | contextual keys for the active tab — never a key that does nothing |

The detail pane is not a tab: it is the same task seen close up, and it opens
from any tab that has a row.

## The tabs

| tab | question it answers | rows |
|---|---|---|
| Overview | what is the state of the project | counts, attention, what agents can claim, recent activity |
| Needs you | what waits on a human | tasks whose next action is the human's |
| Path | what happens next, in order | claimable, in flight, blocked — with the reason |
| Backlog | what is not a task yet | smaller tasks and draft tasks |
| Autopilot | what the automation is doing | the queue, the live agents, their output |
| Settings | how the tool behaves | keybinds, sort keys, preferences |

### Overview

Not a list — a summary the human reads in one screen.

| block | shows |
|---|---|
| counts | tasks per status; smaller tasks per kind |
| needs you | the same count *Needs you* shows, with the three reasons: to approve, to review, escalated |
| claimable | how many tasks an agent could take right now |
| activity | the last few events, when the journal exists |
| next | the first task of *Path* — what happens next |

### Needs you

The human lane: **approve**, **review**, **escalate**.

| column | shows |
|---|---|
| id | `TSK01` |
| why | `approve` · `review` · `escalated` |
| title | the task title |
| age | how long it has waited |

Rows: `groomed` → approve, `code-review` → review, `needs-human` → escalated.
A task with an overlay (`blocked`, `deferred`) drops out of this tab — it is
not waiting on a human.

### Path

Master-plan's Path tab, reproduced: a **spine** of what can run now, with
**branches** for what is waiting, and the blocked branch forked under what
blocks it.

```
 Path Tree 
 EXECUTION  ready · approved · deps met
 │
 ● TSK29   Lorem ipsum dolor sit amet
 │
 ├─ TSK09   Tempor incididunt ut labore et dolore
 │
  ╵
 ├─ AWAITING APPROVAL · 2
 │   └─ TSK04   Consectetur adipiscing elit
 ├─ BLOCKED · 3
 │   └─ blocked-by TSK04
 │       └─ TSK07   Tempor incididunt ut labore
 ├─ RUNNING · 6
 │   └─ TSK10   Tempor incididunt ut labore
 ├─ REVIEW · 3
 │   └─ TSK13   Praesentium voluptatum deleniti
 └─ ESCALATED · 3
     └─ TSK19   Quaerat voluptatem consequatur
```

| part | rule |
|---|---|
| spine | the claimable tasks: `approved`, every dependency `done`, no overlay, no live claim — in dependency order. The first is marked `●`: it is what happens next |
| AWAITING APPROVAL | `groomed` — waiting on the human to approve |
| BLOCKED | `approved` but not claimable. Forked under **the first of its dependencies that is not done**, and the row names the rest |
| RUNNING | `executing` and `remediating` |
| REVIEW | `code-review` |
| ESCALATED | `needs-human` — the review budget ran out |
| order | spine, then the branches in the order above |

Rules:

- The tree holds the **live work**: the tasks that are neither `draft` nor
  `done`. Drafts are the Backlog tab's, and done tasks are history.
- An empty branch is skipped.
- The status is not in the branch — it is the **colour** of the row, from the
  theme's phase mapping. The branch says who the next action belongs to.
- A row names its dependencies: `waits on TSK05, TSK06`.
- The tree is drawn as styled lines with a spine, and it scrolls.

This is master-plan's shape with our statuses: its lanes were *awaiting
approval*, *blocked*, *grooming* and *review*, and it grouped the blocked
items under their first dependency too.

### Backlog

Everything that is not a task yet, in one list — smaller tasks and draft tasks
together, so the human sees one backlog.

| column | shows |
|---|---|
| id | `ITM01` or `TSK01` |
| kind | `idea` · `bug` · `feature` · `draft` |
| title | the title |
| scope | the scope, when it is known |
| state | `open` · `dismissed` · `promoted` |

Smaller tasks are grouped by kind — ideas first, then the small tasks that
could be worked. The grouping is derived from the kind; it is never stored.

### Autopilot

Master-plan's *Watch* and maitre's *Sous-Chef*, in one place: what the
automation is doing.

| block | shows |
|---|---|
| queue | the units of work the autopilot has taken or is about to take |
| agents | the live agents, their labels and their state |
| output | the tail of what an agent printed |
| controls | attach, stop, detach |

This tab is a placeholder until the autopilot is designed. It shows nothing
until then, and says so.

### Settings

| row | shows |
|---|---|
| keybinds | the key for each action, rebindable |
| sort | the sort key per tab |
| theme | the colours |

## The detail pane

The task, close up — as a **document**. The record is JSON; what the human
reads is markdown, projected from that record and rendered through leaf's
engine ([ADR 0009](../decisions/0009-markdown-projection.md)).

The document carries the task's sections, in reading order:

| section | shows |
|---|---|
| header | id, title, status, overlays, claim, priority, effort, tags |
| intent | outcome and problem |
| scope | in and out |
| acceptance criteria | each: description, verification, status, evidence, deferral |
| steps | each: action, done_when, status, deviation |
| review | the rounds — kind, focus, verdict, findings — and the findings' threads |
| questions | open and resolved |
| notes | what the execution learned |
| dependencies | what it waits on, what waits on it |
| children | child tasks and their progress |

The projection is the shape, not the wording: headings per section, tables for
the criteria and the rounds, lists for the steps and the findings, and the
verification commands as code. The task's own free text — the intent, the
criterion descriptions, the notes — is markdown in its own right and is
rendered as such; fields that must not be interpreted are escaped.

Nothing in the detail pane is editable in v1. raul reads.

## Derived, never stored

Every view above is computed from the task files.

| view | rule |
|---|---|
| lane | human lane = `groomed`, `code-review`, `needs-human`; agent lane = `approved`, `executing`, `remediating` |
| needs you | the human lane, minus the tasks carrying an overlay |
| claimable | `approved`, no overlay, no live claim, every dependency `done` |
| blocked reason | dependency not done · overlay set · claimed by another session |
| review state | `pending-review` · `open-findings` · `clean` |
| criterion status | from the latest evidence |
| children progress | from the children's own statuses |
| next | the first claimable row of *Path* |

## States

| state | what raul shows |
|---|---|
| empty | "no tasks yet" in the pane, not a blank screen |
| no work folder | "no coding-lanes folder found" with the path it looked in |
| unreadable file | the file name and the error; the rest of the list still renders |
| stale | the list is read from disk on every redraw; a changed file shows up on the next redraw |

## Rendering a screen as text

`raul --render --plain` prints the current screen as plain text and exits.
It is how a prototype is captured and how a snapshot test is written — the
sketch is the tool's own output, not a drawing of it.

## Not defined yet

- the autopilot's internals — the tab is a placeholder, and its design is
  being settled by comparing terminal-text variants before it is built
- the journal — *Activity* is not a tab until the event kinds exist
- persisted sort keys and keybinds — the Settings tab is a placeholder too
- the review's interaction: what raul can do about a finding, if anything

## Mock data

raul reads `mock-data/coding-lanes/work/` — one task per phase, the edge
cases, and the smaller tasks — so the screens can be built against real shapes
before any data exists to manage. `make raul` points at it.
