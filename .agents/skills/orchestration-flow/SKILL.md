---
name: Orchestration Flow
description: Run multi-agent work in coding-lanes — Herdr tabs and panes, opencode instances with per-role models, the orchestrator's rules, and the role prompts. Load this when dispatching other agents, or when acting as the orchestrator of a loop.
---

# Orchestration flow

Canonical for loop mechanics: hub-and-spoke, the caps, who commits. `AGENTS.md`
points here instead of restating them.

## Roles

Every dispatched agent is told its role in the first line of its prompt
(`Your role: runner | reviewer`). Every session that is not dispatched is the
**orchestrator**. The role is the whole job description; the docs are the
shared memory.

| Role | Does | Never does |
|---|---|---|
| **Orchestrator** | Designs; decides every transition; opens the tabs for runner/reviewer agents; verifies agent claims by re-running the gate; makes local commits after a clean review; keeps the task file current. | **Edits code, tests or scripts itself** — even small ones. Any code change goes to a runner, then a reviewer. Runs no long builds. |
| **Runner** | Implements exactly the kickoff task; runs `make check`; records the evidence; reports to the orchestrator. | Commits; starts other work; prompts other agents; decides next steps. |
| **Reviewer** | Reviews the diff against the task's intent and acceptance criteria; writes findings; gives a verdict. | Modifies code or docs; commits; prompts other agents. |

A new session with no role instruction is the orchestrator: read `AGENTS.md`,
then `docs/tasks/task-model.md`, `docs/roles/roles.md`, `docs/vocabulary.md`,
and the current task file, then continue from the task. Rule of thumb: if a
change touches a `.rs` file, it is delegated.

These are the roles of [docs/roles/roles.md](../../docs/roles/roles.md) —
`creator`, `runner`, `reviewer`, `orchestrator` — with the loop's spelling:
the loop's **builder** is a `runner`, and the loop's **fixer** is a `runner`
on its second round.

## Topology: hub-and-spoke

The orchestrator session is the hub. Runner and reviewer agents never talk to
each other — **every agent replies only to the orchestrator**, and the
orchestrator alone decides what happens next (open a reviewer pane, send
findings to a runner, ask the user). Agents must never prompt another agent's
pane.

Each dispatch is a **fresh opencode session** — fresh context is intentional;
the docs are the memory, not the conversation.

## Herdr mechanics

All multi-agent work runs through Herdr, in the orchestrator's own workspace.

| what | command |
|---|---|
| open a tab | `herdr tab create --workspace <ID> --label "<label>" --cwd <dir> --no-focus` |
| split a pane | `herdr pane split <PANE> --direction right --ratio 0.5 --cwd <dir>` |
| start an agent in a pane | `herdr pane run <PANE> opencode mini --model <provider/model>` |
| send a prompt | `herdr pane send-text <PANE> "<prompt>"` then `herdr pane send-keys <PANE> enter` |
| read a pane | `herdr pane read <PANE> --lines 40` |
| close a tab | `herdr tab close <TAB_ID>` |

**Always open agent tabs in the orchestrator's own workspace**: pass
`--workspace <ID>` on `herdr tab create` (the ID comes from
`herdr pane current` or `herdr agent list`). Omitting it drops the tab into
whatever workspace was last active, scattering agent tabs across workspaces.
`--cwd` sets the working directory, never the workspace.

`herdr pane send-text` types literally: a newline inside the text submits
early and strands the rest in the input box. Send the prompt as one line, or
one line per paragraph, then send `enter` separately.

## Naming

- Tabs: `build-<topic>`, `review-<topic>` (e.g. `build-raul-lanes`,
  `review-raul-lanes`).
- Agents: `[a-z][a-z0-9_-]{0,31}` — e.g. `runner-lanes`, `reviewer-lanes`.

## Models

The reviewer must not run the same model as the runner. Different lenses
surface uncorrelated bugs — the same rule the task model applies to review
rounds (`docs/tasks/task-model.md`, the review section).

| role | model |
|---|---|
| Orchestrator | the session's own model |
| Runner | `minimax-coding-plan/MiniMax-M3.1-Flash` |
| Reviewer | `opencode/deepseek-v4.1-flash-free` |

The model is set when the pane starts, so a reviewer pane and a runner pane
are different models by construction. Other active models, when a role needs a
different lens: `opencode/glm-5.3-flash-free`, `opencode/longcat-2.5-free`,
`opencode/mimo-v2.6-flash-free`, `opencode/nemotron-3.5-super-vl-free`,
`opencode/iceman-free`, `opencode/inkling-2-small-free`,
`opencode/flint-40b-free`, `opencode/minimax-m3.1-flash-free`,
`opencode/beam-free`, `opencode/big-pickle`.

`opencode mini` is the pane interface: it is interactive, it takes
`--model`, `--agent` and `--prompt`, and it is lighter than the full TUI for
a pane that only receives a kickoff.

## Loop mechanics

1. **The orchestrator designs and decides.** It writes the task first — through
   `talita`, which is the only writer of task data — then opens a tab in its
   own workspace and starts a runner in it.
2. **The orchestrator submits the kickoff**, then goes idle.
   **Waking mechanism:** every working agent ends its session by prompting
   **the orchestrator pane only** — address it by pane ID
   (`herdr pane send-text <orchestrator-pane-id> "done: <one-line summary>"`
   then `herdr pane send-keys <orchestrator-pane-id> enter`; the pane ID comes
   from the kickoff). The literal name `orchestrator` is not registered, so
   addressing it by name fails. Idle is safe and no polling is needed.
   **Fallback:** if the prompt fails because the pane or session churned, look
   the orchestrator's pane up again with `herdr pane current` and retry once;
   if that also fails, idle with the summary visible in the terminal output —
   the orchestrator recovers verdicts from terminal output on the next
   check-in.
3. **On wake**, the orchestrator reads the task file and the agent's summary,
   then decides and opens the next role in a **fresh tab and pane** (fresh
   context is intentional; the docs are the memory). Only the orchestrator
   decides transitions — never the agents.
4. **The orchestrator verifies** by re-running the gate itself (`make check`) —
   an agent's claim that the gate passed is not evidence.
5. **After a clean review** the orchestrator commits and updates the task file.
   On findings, it opens a fresh runner pane with the findings. Close the tab
   when the round is done.

## Prompt templates

**Runner kickoff:**

> Your role: runner.
> Autonomous: never use the ask tool; if blocked, end with
> `BLOCKED: <question>; options: ...; recommendation: ...` and report to the orchestrator.
> Read AGENTS.md, docs/tasks/task-model.md and the task file `<path-to-task>.json`.
> Your task: `<task id and summary>`. Do not start other work. Record the
> evidence the task's acceptance criteria require, and finish by prompting the
> orchestrator pane.

**Reviewer kickoff:**

> Your role: reviewer.
> Autonomous: never use the ask tool; if blocked, end with
> `BLOCKED: <question>; options: ...; recommendation: ...` and report to the orchestrator.
> Read AGENTS.md and the task file `<path-to-task>.json`. Review the diff
> against the task's intent, scope and acceptance criteria — and against the
> task's own review focus (`spec-compliance`, `edge-cases`, `security`,
> `empirical`, `design`). Check the evidence: commands and exit codes, not
> prose. Write findings as `FIND-nn` with severity and location. Do not modify
> code. Finish by prompting the orchestrator pane.

**Fixer kickoff** (a runner on its second round):

> Your role: runner.
> Autonomous: never use the ask tool; if blocked, end with
> `BLOCKED: <question>; options: ...; recommendation: ...` and report to the orchestrator.
> Read AGENTS.md, the task file and its open findings. Fix every finding.
> Re-run `make check`. Record fresh evidence, and finish by prompting the
> orchestrator pane.

## Clarifications and stalls

Dispatched agents run **autonomously**. Nobody watches their panes, so an
ask-the-user prompt stalls the agent forever.

- **Never use the ask-the-user tool** in a dispatched agent. If something is
  ambiguous, stop and end the reply with a single line:
  `BLOCKED: <the question>; options: <A / B / C>; my recommendation: <X>`.
  Then prompt the orchestrator pane with that line.
- **The orchestrator decides.** It answers from the docs, or asks the user
  through its own ask tool — always stating its recommended option first —
  then re-prompts the same agent with the answer, or opens a fresh one with
  the answer in the kickoff.
- **Stall check:** a pane that shows `idle` with no completion signal (or
  `working` for far longer than its task should take) is read first
  (`herdr pane read <PANE>`); if it is waiting on a prompt, the orchestrator
  answers it, otherwise it is restarted with the task.

## Rules for the orchestrator

- Never run long builds in the orchestrator pane; delegate.
- One active runner at a time, unless the tasks are independent — then
  parallel tabs, each with its own task.
- Cap review/fix cycles at 4. If findings persist after the 4th round, stop
  and ask the user.
- No code ships unreviewed: every changed file is covered by a review round
  before the task is marked done.
- The orchestrator commits after a clean review. Agents never commit or push.

## Human workflow

The user never talks to a dispatched agent directly — everything flows through
the orchestrator.

1. **Request** — the user tells the orchestrator what they want. The
   orchestrator designs, decides, and delegates.
2. **Orchestrator's role** — designs first (the user approves when the change
   is architectural), writes the task, opens a runner pane in a fresh tab,
   verifies claims independently, opens a fresh-context reviewer pane on a
   different model, decides on findings (fix rounds or escalate), commits
   after a clean review, and keeps the task file current.
3. **Escalation ladder** — review/fix capped at 4 iterations per task;
   unresolved findings after that go to the user. Ambiguous product decisions
   go to the user immediately; the orchestrator decides within the approved
   principles and records the decision in the task's notes.
4. **Batching** — user-reported friction is batched into one loop per batch,
   not one agent per annoyance.
5. **Resumption** — any fresh session reads `AGENTS.md` → `docs/tasks/task-model.md`
   → `docs/roles/roles.md` → `docs/vocabulary.md` → the current task file, and
   continues from the task. Herdr runtime state is ephemeral and rebuilt from
   these documents.
6. **Commit policy** — the orchestrator commits locally after a clean review;
   pushing is the user's decision.

## Direct user requests in an agent pane

The user *should* route everything through the orchestrator, but messages
sometimes land directly in an agent pane. Then that pane is a **reporter, not
a builder**:

1. Diagnose **read-only** — reads, greps and test runs are fine, no edits.
2. Reply with the root cause, the realistic fix options, and their trade-offs.
3. Stop. Do not edit code or docs, and do not commit.

The orchestrator owns the design decision, the delegation and the review loop.
"But the user asked me directly" is not authorization to build — a direct
request is a *report trigger*, and the report goes back through the
orchestrator pane even when the user is watching live.
