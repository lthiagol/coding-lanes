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
- In an interactive session the human and agent discuss and groom a task;
  once agreed, the agent calls the CLI tool to write it down. The CLI tool
  enforces patterns and formats, so agents should not hand-edit task files.
- Tasks carry status and metadata, all managed through the CLI tool.
- `coding-lanes/` at the repo root holds the data: tasks, metadata, code
  review findings, and everything else the project uses to manage tasks.
  The CLI tool writes it, raul reads it.
- Never hand-edit files under `coding-lanes/`. Use the CLI tool.
- Dogfooding coding-lanes on this project is future work.

## Repo

- `crates/core` — shared model, editable.
- `crates/cli` — CLI tool; binary `coding-lanes`. Off-limits until instructed.
- `crates/tui` — raul; binary `raul`.
- `coding-lanes/` — data folder (not yet created).

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
- Work order:
  1. Design raul's screens and data flow.
  2. Implement raul.
  3. The CLI tool is written only when explicitly instructed — never
     automatically because raul looks finished or mature enough.
- `crates/core` stays editable; it is shared with raul.
