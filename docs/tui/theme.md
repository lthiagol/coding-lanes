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
| `text.dim` | secondary text: the *waits on*, the empty state, a marker |
| `text.strong` | a count, a title that matters |
| `text.selected` | the selected row |
| `semantic.ok` | a clean result, a finished task |
| `semantic.warn` | something wants attention and is not broken |
| `semantic.error` | a failure: an unreadable file, a failed criterion, an escalation |
| `semantic.info` | neutral information |
| `semantic.human` | the human lane: approve, review |
| `semantic.agent` | the agent lane: claimable, running |

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

## The presets

| preset | what it is |
|---|---|
| `dark` | the default: the dark palette |
| `terminal` | no colours of our own — the terminal's palette is used |

The preset is chosen with a flag for now: `raul --theme dark|terminal`.
A theme file on disk is later.

**The light themes are deferred.** No light preset is built until that changes:
not ours, and not the light variants of the families — Catppuccin Latte,
Dracula Alucard and Nord Light wait with them. The slots are shaped so a light
preset drops in without touching anything else.

**Selection is the accent at full opacity.** A terminal has no alpha
compositing, so a translucent selection is not available; every preset uses
the accent colour whole.

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
