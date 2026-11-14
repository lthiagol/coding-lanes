# Plan

The work ahead, in small units. `TODO.txt` is the orchestrator's memory — this
is the groomed backlog it points at.

Every unit has a **done when**: the sentence that says the unit is finished.
A unit without one is not ready, and belongs under **Grooming** instead.

| mark | meaning |
|---|---|
| **S** · **M** · **L** | size, the same scale as a task's effort |
| ⬚ | waiting on something else |
| ? | needs a decision before it can start — see **Questions** |

## Ready

| # | unit | size | done when |
|---|---|---|---|
| 1 | **Store discovery in core.** One rule, both tools call it | S | `Store::discover` finds the nearest `coding-lanes/work`; raul and talita agree on a project whose `coding-lanes/` has no `work/` in it, and a test says so |
| 2 | **The Path data layer.** `running()`, `blocked()` with the blocker named, the forking groups | S–M | the three sets are derived from the fixtures and pinned by tests; a blocked task names its blocker |
| 3 | **The Path rendering.** The tree as styled lines: spine, trunk markers, branch headers with counts, blocked items forked under their blocker | M | the tree draws the fixtures; the drawing may be ported from master-plan's `path_view.rs` — the technique, not its model |
| 4 | **The Path at narrow widths.** The tree must not cut a count or a title | M | at 40 columns nothing is cut, and a count is never shown as if it were the whole count |
| 5 | **`raul --render --plain`.** Print a screen as text and exit | S | the Overview renders as text; the output is what a snapshot test compares |
| 6 | **The Needs you tab.** The three reasons, one row per task | S–M | approve, review and escalated each list their tasks, with the counts the Overview shows |
| 7 | **The Backlog tab.** Smaller tasks and drafts in one list | S–M | the smaller tasks and the draft tasks are listed together, grouped by kind; a dismissed or promoted item is visible as such |
| 8 | **The lease fix in core.** Parse the timestamps instead of comparing strings | S | a lease written without seconds expires on the minute it says, and a test holds it |
| 9 | **The theme catalog.** One family at a time: Nord, then Catppuccin, then Dracula | S each | a family is a preset built from its own page, with its values in `docs/tui/palettes.md`, and the render tests run against it |
| 10 | **The projection in core.** A task → markdown, pure, with the escaping rule | M | `task → markdown` is a function with no terminal in it, the escaping rule is written down, and the output is asserted in tests |
| 11 | **`crates/engine`.** Vendor leaf's engine: MIT, the upstream commit, `VENDORED.md`, a sync script | M–L | the engine renders markdown to ratatui lines; the provenance is recorded and the vendored copy is byte-identical to upstream |
| 12 | **The detail pane.** Render the projection through the engine | M–L | a task opens as a document; `Slot::TextSelected` gets its first user |
| 13 | **The pty harness.** `scripts/pty-sweep.py`, the size sweep, as repo code | M | the sweep runs over the built binary and its output is a file; the size-dependent defects are caught by a command, not by a review round |

## Grooming

Scope is not settled. These need design before they can be tasks with a
"done when".

| # | what | what has to be decided first |
|---|---|---|
| G1 | **The Autopilot tab** | what the automation *is*. Our flow — three agents, five rounds, a queue — is the candidate, but the tab is not the flow. maitre's Sous-Chef answers this with an order, a queue and a stamp; we would need the same, and we do not have an autopilot |
| G2 | **The journal** | the event kinds, and where the file lives. `activity.json` is named in the model and nothing writes it; the kinds are undefined |
| G3 | **talita's operation surface** | the verbs, the gates each one enforces, and the output shape. Out of scope until it is asked for explicitly |

## Questions

Marked so we can verify them later rather than lose them.

| # | question | why it matters |
|---|---|---|
| Q1 | **The Path tree's footer.** master-plan shows "N complete · open Milestones to list". Ours would point at a tab that does not list done tasks | drop the footer, or give the done tasks somewhere to be seen? |
| Q2 | **The active tab's highlight.** It lost its inversion in the theme unit — now the main foreground in bold against the secondary | the mapping table asks for it, but it is a visible change. Keep, or restore the inverted block? |
| Q3 | **The journal's home.** Next to the tasks in `coding-lanes/`, or elsewhere in the project? | it decides where the file lives, and therefore what the work folder contains |

## Deferred

- **The light themes** — our `light` preset and the light variants of the
  families (Catppuccin Latte, Dracula Alucard, Nord Light). Deferred until
  that changes.
- **talita** — no code, no commands, until it is asked for explicitly.
