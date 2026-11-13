# 0001. Naming — the project, the two interfaces, the binaries

- **Status:** ✅ accepted
- **Date:** 2026-11-13
- **Deciders:** coding-lanes

## Context

coding-lanes has two interfaces and they are not interchangeable. One is written by
agents. One is read by humans. Both act on the same records.

Names surface everywhere: a binary name, a package name, a crate path, a help line,
a window title, a command in a transcript a human reads later. A name is therefore
not decoration — it is a word an agent types and a human reads.

The risk to avoid is names that describe a mechanism rather than a purpose. "The CLI
tool" and "the TUI" are true and useless: they tell a reader what shape the program
has, not what it is for, and they leave the human with no word for the tool they read.
The second risk is naming both surfaces after the same thing, so that a transcript
cannot say which one did what.

The project needs names that carry an acronym's worth of meaning — the expansion has
to say what the tool does — while staying short enough to type and to read in a
terminal.

## Decision

### The project

| | |
|---|---|
| Project | **coding-lanes** |
| Tagline | *An agentic todo list with superpowers* |
| Workspace layout | `crates/core`, `crates/talita`, `crates/raul` |

The name says what the project is: lanes of work, each with a next action and an
owner. It is the same word the TUI groups tasks by, and the same word the acronym
`raul` expands to (*Understand Lane*).

### The two interfaces

| | talita | raul |
|---|---|---|
| What it is | the CLI tool | the TUI |
| Audience | agent | human |
| Acronym | **T**ask **A**ccess and **L**ane **I**nteraction **T**erminal **A**pp | **R**eview, **A**pprove, **U**nderstand **L**ane |
| Binary | `talita` | `raul` |
| Package | `coding-lanes-talita` | `coding-lanes-raul` |
| Crate | `crates/talita` | `crates/raul` |
| Writes task data | yes | no — see [ADR 0007](0007-one-writer.md) |

### Why talita

It is the tool that **reaches into the task**: it creates, edits, claims, approves,
records evidence. *Task Access* names that, and *Lane Interaction* names what it
does to the lanes — it moves work between them, one task file at a time.

It is also the surface an agent uses without a human present, so it is the one that
must read as a tool first: a short command word, a stable verb set, no display
charm.

### Why raul

It is the tool that makes the lane **legible to a human**. It does not create work
and does not approve it; it shows what is waiting, what is waiting on whom, and what
the human is being asked to decide. *Review, Approve, Understand Lane* is that job
in three words.

### Why the TUI keeps the name raul

`raul` is not a job title and not a role. It is the name of a surface, and surfaces
wear the product's name. Keeping it means a transcript reads unambiguously:

```
talita task approve TSK01
raul
```

Two different verbs, two different names, one line of history to read. Renaming
raul would lose that, and would lose a short, pronounceable word that fits a
terminal title bar.

### The alternatives, and why they lost

| Option | Lost because |
|---|---|
| "The CLI tool" and "the TUI" | mechanism, not purpose; no word for a human to hold in their head, and nothing to type |
| `coding-lanes-cli` and `coding-lanes-tui` | true and flat. The suffix says what the program is shaped like, and carries no meaning beyond the package name it duplicates |
| Naming both surfaces after roles — `coder` and `reviewer` | a role is a job, and a surface is not a job. It also collides with the roles in [ADR 0005](0005-roles.md) |
| Naming the CLI after the lane and the TUI after the board | two metaphors for one idea; the TUI shows lanes, not boards, and "board" would need a second definition |
| `talita` renamed later | the name is in package names, crate paths and every transcript. Renaming costs more than it ever earns, so the name is chosen once |

## Consequences

### Positive

- A human has a word for each tool, and a transcript never needs a gloss.
- The acronym expansions say what each tool is for, so the names survive being read
  months later by someone who has forgotten them.
- Package and crate names follow the binary names, so there is one name per
  surface.

### Negative

- **Two invented names have to be learned.** A new reader meets `talita` and
  `raul` before meeting either tool. The expansions in the docs and the help text
  carry the weight.
- The names do not describe a workflow. `raul` in particular hides the fact that it
  is read-only today; that is the cost of naming a surface by its audience rather
  than by its features.

### Follow-up

- The help text and window titles use the same names, decided in
  [docs/vocabulary.md](../vocabulary.md) § *What is strings only*.
