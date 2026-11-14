# Theme

The colours of raul.

The TUI has a theme. The domain does not: a task file never names a colour,
and `crates/core` stays UI-free.

## The slots

A slot names a **role**, not a colour. Two presets can paint the same slot
differently.

| slot | what it paints |
|---|---|
| `chrome.title` | the title line, the product name |
| `chrome.tab_active` | the tab that is showing |
| `chrome.tab_inactive` | the other tabs |
| `chrome.tab_index` | the `3/6` index |
| `chrome.border` | a block's border |
| `chrome.border_focus` | the border of the focused block |
| `chrome.footer` | the footer's text |
| `chrome.key` | a key label in the footer and in the help overlay |
| `text.normal` | ordinary text |
| `text.dim` | secondary text: the empty state, a marker, a hint under a line, the path in the title line |
| `text.strong` | a count, a title that matters |
| `text.selected` | the selected row |
| `semantic.ok` | a clean result, a finished task |
| `semantic.warn` | something wants attention and is not broken |
| `semantic.error` | a failure: an unreadable file, a failed criterion, an escalation, no coding-lanes folder found |
| `semantic.info` | neutral information |
| `semantic.human` | the human lane: approve, review, and the words that name it |
| `semantic.agent` | the agent lane: claimable, running |

The words that name a lane take the lane's slot: "wait on a human" is
`semantic.human`, and the number beside it is `text.strong`. The same fact
is never painted two ways on one screen.

## The phases and their slots

Eight phases, five colours. A phase never names a colour; it names a slot.

| phase | slot | why |
|---|---|---|
| `draft` | `text.dim` | nothing has been said about it yet |
| `groomed` | `semantic.human` | a human approves it |
| `approved` | `semantic.agent` | an agent can take it |
| `executing` | `semantic.agent` | an agent is on it |
| `code-review` | `semantic.human` | a human reviews it |
| `remediating` | `semantic.warn` | it is being fixed |
| `needs-human` | `semantic.error` | the review budget ran out; a person has to step in |
| `done` | `semantic.ok` | it is finished |

## The reasons and their slots

Three reasons wait on a human, and they take the same slots as the phases
they stand for:

| reason | slot | why |
|---|---|---|
| `approve` | `semantic.human` | a human approves it |
| `review` | `semantic.human` | a human reviews it |
| `escalated` | `semantic.error` | the review budget ran out; a person has to step in |

An escalation is the one reason that is a failure, and it is painted like one.

## The titles of the blocks

The title of a block is `text.strong`: a title that matters. It sits on the
border, and the border's attributes bleed into it, so the dimming of a dimmed
border is taken away again: a title is never dimmed.

The border of a block is `chrome.border`, and the border of the focused one is
`chrome.border_focus`.

## The presets

| preset | what it is |
|---|---|
| `dark` | the default: the dark palette |
| `terminal` | no colours of our own — the terminal's palette is used |

The preset is chosen with a flag for now: `raul --theme dark|terminal`.
A theme file on disk is later.

The `dark` preset is ours, hand-tuned, and its eighteen values are written
down in [palettes.md](palettes.md), with their roles and their contrast.

**The dark preset assumes a terminal with a dark background, and does not
paint one.** The background is the terminal's own; every colour of the
palette is picked to read on a dark one.

**The light themes are deferred.** No light preset is built until that changes:
not ours, and not the light variants of the families — Catppuccin Latte,
Dracula Alucard and Nord Light wait with them. The slots are shaped so a light
preset drops in without touching anything else.

**Selection is the accent at full opacity.** A terminal has no alpha
compositing, so a translucent selection is not available; every colour
preset uses the accent colour whole. The terminal preset has no accent:
its selection is reversed video, and its own section below says so.

## The terminal preset

The `terminal` preset names no colour of its own: the terminal's palette is
used. What is left are the attributes every terminal has, and there are only
five of them — bold, dim, italic, underlined, reversed.

So the terminal preset tells things apart by attribute, and it says what it
can and cannot do:

- **the semantic roles are told apart.** The five semantic roles of the
  phases — `semantic.ok`, `semantic.warn`, `semantic.error`,
  `semantic.human` and `semantic.agent` — each hold a different set of
  attributes, and `semantic.info` holds a sixth. None of them holds the
  same set as `text.normal`. A finished task and a task being fixed look
  different, and so do the human lane and the agent lane.
- **the two lanes never share one.** They hold different attributes, so the
  rule that the lanes are two colours holds here too.
- **the selection is not a colour.** `text.selected` is reversed video: the
  shape of the selection carries the meaning, not a hue.
- **some roles are weaker than they would be with a colour.** `semantic.info`
  is bold and dim, so the heading is louder than the `draft` row under it,
  which is dim alone; with a colour it would be a hue of its own.
  `semantic.warn` is underlined and `semantic.error` is bold, underlined and
  reversed: loud, but not red.
- **what it cannot do:** it cannot say *which* hue a thing is, and it cannot
  tell a colour from the terminal's own foreground. The five attributes are
  enough for the eight phases and not enough for the eighteen slots: the
  weak chrome slots — the muted foreground, the dimmed foreground, the
  border, the footer and the inactive tab — all hold DIM, and the title,
  the strong text and the focused border all hold BOLD. The terminal preset
  tells the phases apart; it does not tell the chrome apart.

The preset exists for terminals whose palette the reader has chosen. Where
the paint matters, the `dark` preset is the one.

## Rules

1. No colour is written inline. Every colour comes from a slot.
2. A slot names a role. A preset decides what the role looks like.
3. The domain knows nothing about any of this.
4. A theme changes the paint, never the words: the same screen says the same
   thing in every preset.

## Not yet

- **the catalog.** Catppuccin (latte, frappé, macchiato, mocha), Dracula
  (dracula, alucard) and Nord (nord, nord-light). The values come from each
  family's own page, and the mapping to our slots is in
  [palettes.md](palettes.md).
- **a theme file on disk.** The preset is a flag for now; a file comes with
  the catalog.
