# Design docs

**coding-lanes** — an agentic todo list with superpowers. Two interfaces:
**talita**, the CLI tool that writes and manages tasks (**T**ask **A**ccess and
**L**ane **I**nteraction **T**erminal **A**pp), and **raul**, the TUI that
shows them (**R**eview, **A**pprove, **U**nderstand **L**ane).

| doc | what |
|---|---|
| [tasks/task-model.md](tasks/task-model.md) | the unit of work: file format, ids, statuses, transitions, review, evidence, steps |
| [tasks/smaller-tasks.md](tasks/smaller-tasks.md) | ideas, bugs and small features — the work that is not a task yet |
| [roles/roles.md](roles/roles.md) | actors, roles, ownership, session boundaries |

The two cross-reference each other: the task model names the roles that act,
the roles doc names the transitions they act on.
