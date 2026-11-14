# Plan

The work ahead, in small units. `TODO.txt` is the orchestrator's memory — this
is the groomed backlog it points at, and **the order of record**.

Every unit has a **done when**: the sentence that says the unit is finished.
A unit without one is not ready, and belongs under **Grooming** instead.

| mark | meaning |
|---|---|
| **S** · **M** · **L** | size, the same scale as a task's effort |
| ⬚ | waits on another unit |
| ? | waits on a decision — see **Questions** |
| ● | in flight |

## Ready

| # | unit | size | done when |
|---|---|---|---|
| 0 | **Close the theme unit** | ● | round 4's four findings closed, the test target compiles, `cargo fmt --check` clean, a round 4 verdict of `ok`, `TODO.txt` updated |
| 1 | **`raul --render --plain`** | S | the current screen prints as text without a terminal, exits 0, and prints the same bytes twice in a row; the Overview's text is a snapshot file in the repo, and the snapshot test fails when the file and the output differ. ratatui has no plain-text backend, so the unit captures the `TestBackend` buffer |
| 2 | **The lease fix in core** | S | a lease written without seconds expires on the minute it says, and a test holds it. The rule stops comparing timestamps as strings |
| 3 | **The store discovery in core** ? | S | `crates/core` holds one rule — the nearest ancestor holding `coding-lanes/work` — and `crates/raul` calls it; the second copy in `crates/raul/src/main.rs` is gone; a test in core covers a project whose `coding-lanes/` holds no `work/`; `crates/talita` is untouched |
| 4 | **The Path data layer** ⬚ | S–M | the spine and the five branches are derived from the fixtures: trunk 1, AWAITING APPROVAL 2, BLOCKED 3, RUNNING 6, REVIEW 3, ESCALATED 3; a blocked task is forked under the first of its dependencies that is not done and names the rest; the fixtures gain a case for each blocking reason — dependency not done, overlay, claim held elsewhere — through `mock-data/generate.py` |
| 5 | **The Path drawing** ⬚ | M | at 100 columns the tree has the shape of `docs/tui/screens.md`: the spine, the `●` on the first row, the branch headers with their counts, and the blocked rows forked under their blocker; at 40, 60 and 100 columns no line exceeds the width, a count is never shown without the group it counts, and a hidden count is announced. The drawing may be ported from master-plan's `path_view.rs` — the technique, not its model |
| 6 | **The Needs you tab** ⬚ | S–M | approve, review and escalated each list their tasks, with the counts the Overview shows |
| 7 | **The Backlog tab** ⬚ | S–M | the smaller tasks and the draft tasks are listed together, grouped by kind; a dismissed or promoted item is visible as such |
| 8 | **The theme catalog: Nord** ⬚ | S | a preset built from Nord's own page, its values in `docs/tui/palettes.md`, all eighteen slots painted, and the render tests run against it on every tab. `nord-light` waits with the light theme |
| 9 | **The theme catalog: Catppuccin** ⬚ | S | the same, for `mocha`. The light flavours wait with the light theme |
| 10 | **The theme catalog: Dracula** ⬚ | S | the same, for `dracula`. `alucard` waits with the light theme |
| 11 | **The projection in core** | M | `task → markdown` is a pure function with no terminal in it; the escaping rule is the one in ADR 0009; the output for a fixture task is asserted |
| 12 | **`crates/engine`: vendor leaf's module** | M–L | the upstream commit and the file set are recorded in `VENDORED.md`, every deviation from upstream is listed there as a numbered item, the dependency set is `pulldown-cmark 0.12`, `syntect 5.2`, `mmdflux 2.5`, `unicodeit 0.2` and our `ratatui 0.30`, and a script re-syncs and prints the diff. The vendored copy is **not** byte-identical: leaf's theme is collapsed and its tests are ported |
| 13 | **`crates/engine`: bridge the slots** ? ⬚ | M | our eighteen slots map onto leaf's `MarkdownTheme`, so the detail pane speaks one palette rather than two |
| 14 | **The detail pane: the shell** ⬚ | S–M | a task opens, `esc` returns to the tab, and the pane is a split or a full screen as the spec says |
| 15 | **The detail pane: the document** ⬚ | M | the document carries the sections of `docs/tui/screens.md`; its text comes from the projection and nowhere else; `text.selected` gets its first user |
| 16 | **The pty harness** ⬚ | S–M | the sweep runs `raul --render --plain` at every width from 40 to 120 and writes one file per width; a width whose output has a line longer than the width fails the command. What is left of a pty's value is the terminal's own behaviour — keys, resize, the alternate screen |
| 17 | **The Settings tab** | S | a placeholder that says so: the tab exists, it lists nothing yet, and it says what it will hold |

## Grooming

Scope is not settled. These need design before they can be tasks with a
"done when".

| # | what | what has to be decided first |
|---|---|---|
| G1 | **The Autopilot tab** | what the automation *is*. Our flow — three agents, five rounds, a queue — is the candidate, but the tab is not the flow. maitre's Sous-Chef answers this with an order, a queue and a stamp; we would need the same, and we do not have an autopilot |
| G2 | **The journal** | the event kinds, and where the file lives. `activity.json` is named in the model and nothing writes it; the kinds are undefined |
| G3 | **talita's operation surface** | the verbs, the gates each one enforces, and the output shape. Out of scope until it is asked for explicitly |
| G4 | **The review's interaction** | what raul can do about a finding, if anything. `docs/tui/screens.md` lists it under "not defined yet" and nobody owns it |

## Questions

Marked so we can verify them later.

| # | question | why it matters |
|---|---|---|
| Q1 | **The Path tree's footer.** master-plan shows "N complete · open Milestones to list". We have no tab that lists done tasks | drop the footer, or give the done tasks somewhere to be seen? |
| Q2 | **The active tab's highlight.** It lost its inversion in the theme unit — now the main foreground in bold against the secondary | the mapping table asks for it, but it is a visible change. Keep, or restore the inverted block? |
| Q3 | **The journal's home.** Next to the tasks in `coding-lanes/`, or elsewhere in the project? | it decides where the file lives, and therefore what the work folder contains |
| Q4 | **The store discovery rule.** Stop at the nearest `coding-lanes/` — this folder is the project and it has no work folder, say so — or walk past it and risk showing an ancestor project's tasks as if they were this project's? | it decides what a half-made project shows, and unit 3 is blocked on it |
| Q5 | **leaf's markdown palette inside the detail pane, or our slots mapped onto it?** | the theme rule says a theme changes the paint and never the words, and the same fact is never painted two ways on one screen. Two palettes inside one screen breaks both |

## Deferred

- **The light themes** — our `light` preset and the light variants of the
  families (Catppuccin Latte, Dracula Alucard, Nord Light). Deferred until
  that changes.
- **The theme file on disk**, and a user theme that is not one of the
  built-ins.
- **talita** — no code, no commands, until it is asked for explicitly.
