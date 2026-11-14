# AGENTS.md

Rules for agents working in this repository.

## Project

coding-lanes is a task management tool for agents — a todo list with
superpowers.

- Two interfaces:
  - **talita** — the CLI tool. Agent-facing, not designed for humans.
    **T**ask **A**ccess and **L**ane **I**nteraction **T**erminal **A**pp.
  - **raul** — the TUI. Human-facing, for reviewing tasks and understanding the
    project's flow and status. **R**eview, **A**pprove, **U**nderstand
    **L**ane.
- In an interactive session the human and agent groom a task; once agreed, the
  agent calls talita to write it down.
- talita writes and manages tasks; raul shows them.
- Tasks live in the managed project's own `coding-lanes/` folder, alongside its
  code. No central store — each project keeps its own data.
- `coding-lanes/` holds tasks, metadata, code review findings, and everything
  else the project uses to manage tasks.
- Never hand-edit files under `coding-lanes/`. Use talita.
- Dogfooding coding-lanes on this project is future work.

## Repo

- `crates/core` — shared model.
- `crates/talita` — talita; package `coding-lanes-talita`, binary `talita`.
  Off-limits until instructed.
- `crates/raul` — raul; package `coding-lanes-raul`, binary `raul`.
- `coding-lanes/` — data folder of a managed project (not created here).

```sh
make check
make raul
```

## Communication

- Be concise. No verbosity in docs, code comments, commit messages, or replies.
- Prefer short, direct sentences. Skip filler, restating the request, and
  summarizing what you just did unless asked.
- Document only what a reader needs; omit obvious explanations.

## Clarification

- Avoid assumptions. If anything is unclear or a decision is needed, use the
  ask tool.
- When asking, always state your recommended option first.

## Git

- Commit messages use conventional commits: `feat:`, `fix:`, `docs:`,
  `chore:`, `refactor:`, `test:`.

## Rules of this file

- Do not edit `AGENTS.md` without explicit instruction.

## Task model

- Task structure, statuses, metadata, and lane definitions are designed in
  `docs/tasks/task-model.md`; roles in `docs/roles/roles.md`; the TUI's
  screens in `docs/tui/screens.md`.
- Do not invent them ad hoc — follow the docs.
- `TODO.txt` at the root is the state of the work: what is done, what is open,
  what is next. Read it before starting; update it when a unit closes.

## Naming

- The TUI is called **raul**.
- The CLI tool is called **talita**.
- Use only the names defined here — `coding-lanes` (project), `raul` (TUI),
  `talita` (CLI tool). Do not invent aliases for the project, its crates, or
  its binaries without asking.

## Scope

- Scope is the TUI (raul). Do not write talita code until explicitly told.
- Work order: design raul's screens and data flow, then implement raul.
- talita is written only when explicitly instructed — never automatically
  because raul looks finished or mature enough.
- `crates/core` stays editable; it is shared with raul.
