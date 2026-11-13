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

A tree, not a list: a **trunk** of what can run now, with **branches** for
what is waiting. The trunk is the agent lane; the branches are the human lane.

```
 Path Tree 
 EXECUTION  ready · approved · deps met
 │
 ● TSK27   Lorem ipsum dolor sit amet
 │
  ╵
 ├─ RUNNING · 6
 │   └─ TSK10   Tempor incididunt ut labore et dolore
 ├─ APPROVAL · 3
 │   └─ TSK04   Lorem ipsum dolor sit amet
 ├─ REVIEW · 3
 │   └─ TSK13   Praesentium voluptatum deleniti
 ├─ ESCALATED · 3
 │   └─ TSK19   Quaerat voluptatem consequatur
 └─ BLOCKED · 3
     └─ TSK08   waits on TSK05
```

The ids above are the ones `mock-data` generates today. The generator
renumbers when it changes, so the tree is the shape to build — not a fixture
to assert against.

| part | rule |
|---|---|
| trunk | claimable tasks, in dependency order. The first is marked `●` — it is what happens next |
| RUNNING | `executing` and `remediating`, with the runner holding the claim |
| APPROVAL | `groomed` — waiting on the human to approve |
| REVIEW | `code-review` — waiting on the human to review |
| ESCALATED | `needs-human` — the review budget ran out |
| BLOCKED | everything else that is not claimable: a dependency that is not done, an overlay, or a claim held elsewhere. Each shows what blocks it, forked under its blocker |
| order | trunk first, then the branches in the order above: RUNNING, APPROVAL, REVIEW, ESCALATED, BLOCKED |

Rules:

- The tree holds tasks only. Smaller tasks and drafts are not in it — the
  Backlog tab holds them.
- A task that is running is not blocked and not claimable: it has its own
  branch.
- A blocked task is forked under the task that blocks it, so the human sees
  the chain, not a flat list of stuck work.
- The trunk is empty when nothing is claimable; the tree then opens straight
  into the branches, and says so.
- The tree scrolls; it is rendered as styled lines, not widgets.

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

The task, close up. Same sections as the file, in reading order.

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

## Not defined yet

- the autopilot's internals — the tab is a placeholder
- the journal — *Activity* is not a tab until the event kinds exist
- persisted sort keys and keybinds — the Settings tab is a placeholder too
- the review's interaction: what raul can do about a finding, if anything

## Mock data

raul reads `mock-data/coding-lanes/work/` — 28 tasks across every phase and 5
smaller tasks — so the screens can be built against real shapes before any
data exists to manage.
