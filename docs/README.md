# Design docs

**coding-lanes** — an agentic todo list with superpowers. Two interfaces:
**talita**, the CLI tool that writes and manages tasks (**T**ask **A**ccess and
**L**ane **I**nteraction **T**erminal **A**pp), and **raul**, the TUI that
shows them (**R**eview, **A**pprove, **U**nderstand **L**ane).

| doc | what |
|---|---|
| [tasks/task-model.md](tasks/task-model.md) | the unit of work: file format, ids, statuses, transitions, gates, review, evidence |
| [tasks/smaller-tasks.md](tasks/smaller-tasks.md) | ideas, bugs and small features — the work that is not a task yet |
| [roles/roles.md](roles/roles.md) | actors, roles, ownership, session boundaries |
| [tui/screens.md](tui/screens.md) | raul: the window, the tabs, the detail pane, the derived views |
| [tui/theme.md](tui/theme.md) | the colour slots, the phase-to-slot mapping, the presets |
| [tui/palettes.md](tui/palettes.md) | where the themes come from: the families, the mapping, the rules for a new one |
| [vocabulary.md](vocabulary.md) | the names we use, and the ones we rejected |
| [plan.md](plan.md) | the work ahead: small units with their done-when lines, the grooming, the open questions |
| [decisions/](decisions/) | decision records: what we chose, and why |
