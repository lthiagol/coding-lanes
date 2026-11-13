# AGENTS.md

Rules for agents working in this repository.

## Communication

- Be concise. No verbosity in docs, code comments, commit messages, or replies.
- Prefer short, direct sentences. Skip filler, restating the request, and
  summarizing what you just did unless asked.
- Document only what a reader needs; omit obvious explanations.

## Clarification

- Avoid assumptions. If anything is unclear or a decision is needed, use the
  ask tool.
- When asking, always state your recommended option first.

## Naming

- The TUI is called **raul**.
- The CLI stays the **cli tool** until we rename it. Renaming the CLI is out of
  scope for now — do not rename it, its crate, or its binary.
- Do not introduce other product names or aliases without asking.

## Scope

- The current work is the workspace itself: `crates/core` (shared model),
  `crates/cli` (CLI tool), `crates/tui` (raul). Do not add features unless
  asked.
