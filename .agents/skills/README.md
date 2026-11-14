# Vendored skills

Third-party skills we depend on, copied into this repo so they can be pinned and
shipped with the tool.

**Upstream.** `github.com/mattpocock/skills` — MIT. See [LICENSE](LICENSE) and
[UPSTREAM.md](UPSTREAM.md) for provenance.

**Pinned to a commit, not `main`.** The pin is the same commit maitre vendored,
so this copy matches maitre's byte for byte and both can be re-synced to the
same place. A branch head drifts before anyone notices — see
[sync.sh](sync.sh).

## What we took, and why

| Skill | Why we need it |
|---|---|
| [codebase-design](codebase-design/SKILL.md) | The `crates/core`-vs-surface seam is our central architectural commitment: the domain is UI-free, talita writes and raul reads, and neither shells out to the other. This is the vocabulary for it: module, interface, depth, seam, adapter, leverage, locality. **Our own `core` package gets judged by the deletion test.** |
| [writing-for-agents](writing-for-agents/SKILL.md) | Most of what we write is for agents. This governs skills and `AGENTS.md` — including the positive-voice rule our house style follows. |
| [domain-modeling](domain-modeling/SKILL.md) | Keeps [docs/vocabulary.md](../../docs/vocabulary.md) and the ADRs in [docs/decisions/](../../docs/decisions/) sharp. It is the discipline behind our names of record: one word per state, one spelling per id. |
| [diagnosing-bugs](diagnosing-bugs/SKILL.md) | Build a feedback loop before hypothesising. No TDD conflict: its regression-test phase is regression testing, not test-first authoring. |

## What we did not take

| Skill | Why not |
|---|---|
| `code-review` | **Adapted instead of vendored** — it becomes our `reviewer` role, specified in [docs/roles/roles.md](../../docs/roles/roles.md) and written up as our own skill in [docs/skills/task-review.md](../../docs/skills/task-review.md). Its Spec axis must read the task's acceptance criteria, not an issue tracker. |
| `wayfinder` | Tracker-coupled and decision-shaped. We took its *ideas* — map-as-index (the work folder is the index), fog of war (the lanes are derived views), HITL-vs-AFK (the human lane versus the agent lane) — not the skill. We kept ids, because ids are of record here. |
| `prototype` | The method transfers; the artifacts assume a web stack. We already have raul for the equivalent. |
| `implement`, `implement-spec` | They assume an orchestrator driving multi-agent worktrees. Our orchestrator role exists, but what it drives does not yet (task-model §14). |
| `improve-codebase-architecture` | Nothing to scan in a small codebase. Its HTML report is a natural fit for raul later. |

## Our own skills

Not vendored, and not upstream: the skills coding-lanes defines.

| Skill | Where | What it covers |
|---|---|---|
| [orchestration-flow](orchestration-flow/SKILL.md) | `.agents/skills/` | the build loop: three agents, the Herdr workspace, the rounds, the budget |
| [task-authoring](../../docs/skills/task-authoring.md) | `docs/skills/` | the `creator` role — writing a task through talita |
| [task-review](../../docs/skills/task-review.md) | `docs/skills/` | the `reviewer` role — reviewing a change against its kickoff |

A skill we want loaded lives in `.agents/skills/`; one that is still a draft
lives in `docs/skills/` until it is promoted.
| [task-review](../../docs/skills/task-review.md) | the `reviewer` role — reviewing an implementation against a task |

## One note on the vendored files

The vendored skills refer to `GLOSSARY.md` and to ADRs. Our equivalents are
[docs/vocabulary.md](../../docs/vocabulary.md) and [docs/decisions/](../../docs/decisions/).
The vendored files stay byte-identical to upstream, so they are **not** edited
to say this — read the mapping here.

## Re-syncing

```bash
.agents/skills/sync.sh          # re-copy at the pinned commit, report what changed
.agents/skills/sync.sh --bump   # move to a newer commit, then re-copy
```

The script never edits a vendored file by hand. If a skill needs changing for
coding-lanes, it becomes **our** skill — under `docs/skills/` while it is a
draft, and under `.agents/skills/` once it is ready to load. The vendored copy
stays pristine so the diff against upstream stays readable.
