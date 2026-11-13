---
name: Orchestration Flow
description: Three agents, one flow — the orchestrator session takes the human's requests and decides, dispatching builder and reviewer panes through Herdr, each an opencode instance on its own model. Load this when running a build loop in this repository.
metadata:
  opencode/autoinvoke: false
---

# Orchestration flow

Canonical for loop mechanics: hub-and-spoke, the caps, who commits. `AGENTS.md`
points here instead of restating them.

This is the flow for **building** this repository. It is not a description of
the project: the project has its own docs, and the agents read those.

## The three agents

| Agent | Where it runs | Does | Never does |
|---|---|---|---|
| **Orchestrator** | the human's session — where the request is typed, and where every agent reports back | Designs; decides every transition; opens the panes; verifies claims by re-running the gate; commits after a clean review | **Edits code, tests or scripts itself** — even small ones. Any code change goes to a builder, then a reviewer. Runs no long builds |
| **Builder** | its own pane, in its own tab | Implements exactly the kickoff; runs the project's gate; reports to the orchestrator | Commits; starts other work; prompts other agents; decides next steps |
| **Reviewer** | its own pane, in its own tab | Reviews the change against the kickoff and the project's rules; writes findings; gives a verdict. May run **targeted** commands to check a claim | Modifies code or docs; commits; prompts other agents; runs long builds |

The orchestrator is the only agent the human talks to, and the only one the
others report to. It is the session that decides what happens next.

These three mirror the project's roles — `orchestrator`, `runner`, `reviewer`
— with the loop's own spelling: the loop's **builder** plays the project's
`runner`, and there is no separate fixer: a builder on its second round is
the fixer.

## Topology: hub-and-spoke

The orchestrator session is the hub. The builder and the reviewer never talk to
each other — **every agent replies only to the orchestrator**, and the
orchestrator alone decides what happens next (open a reviewer pane, send
findings to a builder, ask the human). An agent must never prompt another
agent's pane.

Each dispatch is a **fresh opencode session** — fresh context is intentional;
the repository's docs are the memory, not the conversation.

## Herdr mechanics

All multi-agent work runs through Herdr, in the orchestrator's own workspace.

| what | command |
|---|---|
| open the Orchestration workspace | `herdr workspace create --cwd <dir> --label "Orchestration" --no-focus` |
| list workspaces | `herdr workspace list` |
| close the Orchestration workspace | `herdr workspace close <ID>` |
| open a tab | `herdr tab create --workspace <ID> --label "<label>" --cwd <dir> --no-focus` |
| split a pane | `herdr pane split <PANE> --direction right --ratio 0.5 --cwd <dir>` |
| start an agent in a pane | `herdr pane run <PANE> opencode mini --model <provider/model#variant>` |
| send a prompt | `herdr pane send-text <PANE> "<prompt>"` then `herdr pane send-keys <PANE> enter` |
| read a pane | `herdr pane read <PANE> --lines 40` |
| close a tab | `herdr tab close <TAB_ID>` |

**All agent tabs live in the Orchestration workspace**, not in the
orchestrator's own workspace. See the next section.

`herdr pane send-text` types literally: a newline inside the text submits
early and strands the rest in the input box. Send the prompt as one line, or
one line per paragraph, then send `enter` separately.

`opencode mini` is the pane interface: interactive, takes `--model`,
`--agent` and `--prompt`, and is lighter than the full TUI for a pane that
only receives a kickoff.

## The Orchestration workspace

One workspace per loop holds every agent pane, so the human can watch the
whole loop in one place — and so a tab can never land in the workspace that
happened to be focused.

1. At the start of a loop the orchestrator creates it:

   ```sh
   herdr workspace create --cwd <dir> --label "Orchestration" --no-focus
   ```

   The id comes from the command's output, or from `herdr workspace list`.
2. **Every** agent tab is created there, with `--workspace <orchestration-id>`:

   ```sh
   herdr tab create --workspace <orchestration-id> --label "build-<topic>" --cwd <dir> --no-focus
   ```

   `--cwd` sets the working directory, never the workspace. Omitting
   `--workspace` drops the tab into whatever workspace was last active, and
   the agent ends up somewhere the human is not watching.
3. The workspace survives the rounds of one loop — it is created once and
   reused. At the end of the loop it is closed:

   ```sh
   herdr workspace close <orchestration-id>
   ```

The orchestrator's own session stays where it is. It receives the reports;
the Orchestration workspace holds the work.

## Naming

- Tabs: `build-<topic>`, `review-<topic>` (e.g. `build-m1-engine`,
  `review-m1-engine`).
- Agents: `[a-z][a-z0-9_-]{0,31}` — e.g. `builder-m1`, `reviewer-m1`.

## Models

The reviewer must not run the same model *and the same reasoning effort* as
the builder. Different lenses surface uncorrelated bugs — the same rule the
project's own review rounds follow.

| role | model |
|---|---|
| Orchestrator | the session's own model |
| Builder | `minimax-coding-plan/MiniMax-M3.1-Flash-tuned#medium` |
| Reviewer | `minimax-coding-plan/MiniMax-M3.1-Flash-tuned#high` |

The variant is the model's reasoning effort: the builder works at `medium`,
the reviewer thinks harder at `high`. The model is set when the pane starts,
so the two panes differ by construction.

Other active models, when a role needs a different lens:
`minimax-coding-plan/MiniMax-M3.1-Flash-tuned#low`, `#xhigh`, `#max`,
`minimax-coding-plan/MiniMax-M3#thinking`, and the `opencode/*` family
(`deepseek-v4.1-flash-free`, `glm-5.3-flash-free`, `longcat-2.5-free`,
`mimo-v2.6-flash-free`, `nemotron-3.5-super-vl-free`, `iceman-free`,
`inkling-2-small-free`, `flint-40b-free`, `minimax-m3.1-flash-free`,
`beam-free`, `big-pickle`).

## Loop mechanics

1. **The orchestrator designs and decides.** It decides what the unit of work
   is, creates the Orchestration workspace (once per loop), and opens a tab in
   it for the builder.
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
3. **On wake**, the orchestrator reads the agent's summary, then decides and
   opens the next role in a **fresh tab and pane**. Only the orchestrator
   decides transitions — never the agents.
4. **The orchestrator verifies** by re-running the gate itself — an agent's
   claim that the gate passed is not evidence.
5. **After a clean review** the orchestrator commits and updates the project's
   records. On findings, it opens a fresh builder tab in the same workspace
   with the findings. Close the tab when the round is done; keep the
   workspace for the next round.

**Every round is builder → reviewer.** Nothing a builder produces is reviewed
by the builder, and nothing is committed without a review round — not a
one-line fix, not a rename. The review is the gate, not a courtesy.

**The budget is five rounds.** Round 1 is builder → reviewer. Each
`changes-needed` verdict starts a new round. Round 5 is the last one: if it
comes back `changes-needed`, the orchestrator stops, keeps the workspace
open, and asks the human — with the findings, the rounds, and what it would
change next.

A round the builder ends `BLOCKED` does not spend one of the five: it ended
before anything was built. Only real build-and-review rounds count.

## Prompt templates

**Builder kickoff:**

> Your role: builder.
> Autonomous: never use the ask tool; if blocked, end with
> `BLOCKED: <question>; options: ...; recommendation: ...` and report to the orchestrator.
> Read AGENTS.md and the project's docs it points to.
> Your task: `<the unit of work>`. Do not start other work. Run the project's
> gate before you report. Finish by prompting the orchestrator pane.

**Reviewer kickoff:**

> Your role: reviewer.
> Autonomous: never use the ask tool; if blocked, end with
> `BLOCKED: <question>; options: ...; recommendation: ...` and report to the orchestrator.
> Read AGENTS.md and the project's docs it points to. This is review round
> <N> of at most 5. Review the change against the kickoff and the project's
> rules — not against the builder's own summary.
> Try to break it: different inputs, edge cases, adversarial paths. You may
> run targeted commands to check a claim — not long builds; the orchestrator
> runs the gate. Findings carry severity and location. Do not modify code.
> Finish with a verdict (`ok` or `changes-needed`) by prompting the orchestrator
> pane.

**Fixer kickoff** (a builder on its second round):

> Your role: builder.
> Autonomous: never use the ask tool; if blocked, end with
> `BLOCKED: <question>; options: ...; recommendation: ...` and report to the orchestrator.
> Read AGENTS.md, the project's docs, and the open findings. Fix every finding.
> Re-run the project's gate. Finish by prompting the orchestrator pane.

## Clarifications and stalls

Dispatched agents run **autonomously**. Nobody watches their panes, so an
ask-the-user prompt stalls the agent forever.

- **Never use the ask-the-user tool** in a dispatched agent. If something is
  ambiguous, stop and end the reply with a single line:
  `BLOCKED: <the question>; options: <A / B / C>; my recommendation: <X>`.
  Then prompt the orchestrator pane with that line.
- **The orchestrator decides.** It answers from the docs, or asks the human
  through its own ask tool — always stating its recommended option first —
  then re-prompts the same agent with the answer, or opens a fresh one with
  the answer in the kickoff.
- **Stall check:** a pane that shows `idle` with no completion signal (or
  `working` for far longer than its task should take) is read first
  (`herdr pane read <PANE>`); if it is waiting on a prompt, the orchestrator
  answers it, otherwise it is restarted with the task.

## Rules for the orchestrator

- Never run long builds in the orchestrator pane; delegate.
- One active builder at a time, unless the units of work are independent — then
  parallel tabs, each with its own unit of work.
- **Everything a builder produces gets a reviewer round.** No exceptions.
- **At most 5 rounds** of builder and reviewer per unit of work. Round 5 comes
  back `changes-needed` → stop and ask the human.
- **The orchestrator runs the gate itself** before opening a reviewer round. An
  agent's claim that the gate passed is not evidence.
- **The reviewer may run targeted commands** to check a claim — never long
  builds, and never edits.
- No code ships unreviewed: every changed file is covered by a review round
  before the unit is done.
- The orchestrator commits after a clean review. Agents never commit or push.

## Human workflow

The human types in the orchestrator session and nowhere else. Everything flows
through it.

1. **Request** — the human tells the orchestrator what they want. The
   orchestrator designs, decides, and delegates.
2. **Orchestrator's role** — designs first (the human approves when the change
   is architectural), opens the Orchestration workspace, starts a builder in a
   fresh tab there, verifies claims independently, opens a fresh-context
   reviewer in a fresh tab on a different reasoning effort, decides on
   findings (fix rounds, up to 5), commits after a clean review, and updates
   the project's records.
3. **Escalation ladder** — review/fix capped at 5 rounds per unit;
   unresolved findings after that go to the human. Ambiguous product decisions
   go to the human immediately; the orchestrator decides within the approved
   principles and records the decision.
4. **Batching** — human-reported friction is batched into one loop per batch,
   not one agent per annoyance.
5. **Resumption** — any fresh session reads `AGENTS.md` and the docs it points
   to, and continues from the current unit of work. Herdr runtime state is
   ephemeral and rebuilt from these documents.
6. **Commit policy** — the orchestrator commits locally after a clean review;
   pushing is the human's decision.

## Direct human requests in an agent pane

The human *should* route everything through the orchestrator, but messages
sometimes land directly in an agent pane. Then that pane is a **reporter, not
a builder**:

1. Diagnose **read-only** — reads, greps and test runs are fine, no edits.
2. Reply with the root cause, the realistic fix options, and their trade-offs.
3. Stop. Do not edit code or docs, and do not commit.

The orchestrator owns the design decision, the delegation and the review loop.
"But the human asked me directly" is not authorization to build — a direct
request is a *report trigger*, and the report goes back through the
orchestrator pane even when the human is watching live.
