# Upstream provenance

| | |
|---|---|
| Repository | `github.com/mattpocock/skills` |
| License | MIT — see [LICENSE](LICENSE) |
| Pinned commit | `5beb35b040e3de97e54fa9e386d7e42d15c295c2` |
| Commit date | 2026-10-19 |
| Vendored on | 2026-11-13 |

| Skill | Upstream path | Files |
|---|---|---|
| `codebase-design` | `skills/engineering/codebase-design/` | `SKILL.md`, `DEEPENING.md`, `DESIGN-IT-TWICE.md` |
| `diagnosing-bugs` | `skills/engineering/diagnosing-bugs/` | `SKILL.md`, `scripts/hitl-loop.template.sh` |
| `domain-modeling` | `skills/engineering/domain-modeling/` | `SKILL.md`, `GLOSSARY-FORMAT.md`, `ADR-FORMAT.md` |
| `writing-for-agents` | `skills/productivity/writing-for-agents/` | `SKILL.md`, `SKILL-MECHANICS.md` |

The upstream `agents/openai.yaml` files are **not** vendored — they configure a
different harness.

## Rules for vendored copies

1. **Pinned to a commit.** Never `main`. A vendored copy at a branch head drifts
   before anyone notices.
2. **Byte-identical to upstream.** No edits in place. A skill that needs changing
   for coding-lanes becomes ours, under `docs/skills/`.
3. **Re-sync by script.** `.agents/skills/sync.sh` prints a diff before
   overwriting.
4. **Record the commit.** This file is the provenance; the commit hash is the pin.

## Attribution

These skills are the work of Matt Pocock, MIT licensed. Used and redistributed
under that license. See [LICENSE](LICENSE).
