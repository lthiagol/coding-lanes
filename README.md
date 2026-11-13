# coding-lanes

An agentic todo list with superpowers.

A task management tool for agents. Two interfaces:

- **talita** — the CLI tool. Agent-facing. **T**ask **A**ccess and **L**ane
  **I**nteraction **T**erminal **A**pp. Writes and manages tasks.
- **raul** — the TUI. Human-facing. **R**eview, **A**pprove, **U**nderstand
  **L**ane. Shows them.

## Status

Early. The scope right now is raul. The task model and the roles are
designed in [docs/](docs/); talita's full command surface comes later, when it
is asked for.

| doc | what |
|---|---|
| [docs/tasks/task-model.md](docs/tasks/task-model.md) | the unit of work: format, ids, statuses, transitions, review, evidence |
| [docs/roles/roles.md](docs/roles/roles.md) | actors, roles, ownership, session boundaries |

## Layout

```
crates/
  core/    # shared model   (coding-lanes-core)
  talita/  # CLI tool       (coding-lanes-talita, binary talita)
  raul/    # TUI            (coding-lanes-raul, binary raul)
```

Tasks live in a managed project's own `coding-lanes/` folder:

```
<project>/coding-lanes/tasks/TSK01-<slug>.json
```

## Develop

Rust is pinned via `rust-toolchain.toml` (stable, minimal profile, with
`rustfmt` and `clippy`).

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Make targets

| target | what |
|---|---|
| `make help` | list targets |
| `make check` | the gate: fmt, clippy, tests |
| `make build` | build the workspace |
| `make build-raul` | build raul only |
| `make build-talita` | build talita only |
| `make build-release` | release binaries |
| `make raul` | run raul |
| `make test` | tests |
| `make fmt` / `make fmt-check` | format / check formatting |
| `make lint` | clippy with warnings as errors |
| `make doc` | API docs |
| `make clean` | remove build artifacts |

## License

MIT OR Apache-2.0
