# AGENTS.md

Rules for agents working in this repository.

## Project

coding-lanes is a task management tool for agents — a todo list with
superpowers.

- Two interfaces:
  - **CLI tool** — agent-facing, not designed for humans.
  - **TUI (raul)** — human-facing, for reviewing tasks and understanding the
    project's flow and status. `raul` = **R**eview, **A**pprove,
    **U**nderstand **L**ane.
- In an interactive session the human and agent groom a task; once agreed, the
  agent calls the CLI tool to write it down.
- The CLI tool writes and manages tasks; raul shows them.
- Tasks live in the managed project's own `coding-lanes/` folder, alongside its
  code. No central store — each project keeps its own data.
- `coding-lanes/` holds tasks, metadata, code review findings, and everything
  else the project uses to manage tasks.
- Never hand-edit files under `coding-lanes/`. Use the CLI tool.
- Dogfooding coding-lanes on this project is future work.

## Repo

- `crates/core` — shared model.
- `crates/cli` — CLI tool; binary `coding-lanes`. Off-limits until instructed.
- `crates/tui` — raul; binary `raul`.
- `coding-lanes/` — data folder of a managed project (not created here).

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo run -p coding-lanes-tui   # raul
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

## Naming

- The TUI is called **raul**.
- The CLI stays the **cli tool** until we rename it. Renaming the CLI is out of
  scope for now — do not rename it, its crate, or its binary.
- Do not introduce other product names or aliases without asking.

## Scope

- Scope is the TUI (raul). Do not write CLI tool code until explicitly told.
- Work order: design raul's screens and data flow, then implement raul.
- The CLI tool is written only when explicitly instructed — never
  automatically because raul looks finished or mature enough.
- `crates/core` stays editable; it is shared with raul.
