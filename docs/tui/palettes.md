# Palettes

Where raul's themes come from, and how to write a new one.

Our slots are defined in [theme.md](theme.md). This doc is the **source**
side: the palettes a theme may be built from, where each one came from, and
the rules that keep a new theme consistent with the others.

## Provenance

| what | where it comes from | licence |
|---|---|---|
| the slot structure | ours, shaped by leaf's `UiTheme` | — |
| leaf's own presets — `arctic`, `forest`, `ocean`, `solarized-dark` | leaf's `src/theme/presets.rs` | MIT |
| **Catppuccin** — latte, frappé, macchiato, mocha | [catppuccin.com/palette](https://catppuccin.com/palette/) — the values below are that page's | MIT |
| **Dracula** — dracula, alucard | [draculatheme.com/spec](https://draculatheme.com/spec) — the values below are that page's | MIT |
| **Nord** — nord, nord-light | [nordtheme.com/docs/colors-and-palettes](https://www.nordtheme.com/docs/colors-and-palettes) — the values below are that page's | MIT |
| the mapping rules | [Frond's `COLORS.md`](https://github.com/lthiagol/Frond/blob/main/docs/COLORS.md), adapted to our slots | ours |

**Those three pages are the source of truth.** If a colour is disputed, the
page wins; if we need a role the page does not have, it is derived from a
role it does have, and the derivation is written down here.

Frond is our own other project. It did this exact work for the same upstream.
Where its mapping answers a question, that answer is the answer here.

## The slot mapping

Every theme maps the same way. The left column is ours; the middle is leaf's
key; the right is where a family's role comes from.

| our slot | leaf key | family role |
|---|---|---|
| `chrome.border` | `toc_border` | the family's border colour |
| `chrome.border_focus` | `toc_accent` | the family's accent |
| `chrome.title` | `status_brand_fg` | the family's brightest foreground |
| `chrome.tab_active` | `toc_primary_active` | the family's main foreground |
| `chrome.tab_inactive` | `toc_primary_inactive` | the family's second foreground |
| `chrome.tab_index` | `toc_index_inactive` | the family's dimmed foreground |
| `chrome.footer` | `status_shortcut_fg` | the family's dimmed foreground |
| `chrome.key` | `status_search_fg` | the family's attention colour |
| `text.normal` | `toc_hover_fg` | the family's main foreground |
| `text.dim` | `toc_header_fg` | the family's muted foreground |
| `text.strong` | `toc_secondary_text_active` | a raised foreground |
| `text.selected` | `toc_active_bg` | the accent, at full opacity |
| `semantic.ok` | `status_success_fg` | the family's success colour |
| `semantic.warn` | `status_warning_fg` | the family's warning colour |
| `semantic.error` | `status_error_fg` | the family's error colour |
| `semantic.info` | `status_filename_fg` | a neutral foreground |
| `semantic.human` | `toc_accent` | the family's accent |
| `semantic.agent` | `status_watch_fg` | the family's second accent |

The last two are ours: the human lane and the agent lane need two colours that
are not "success" and "error". The accent and the second accent are the
natural pair — the accent is already the focus colour, so the agent lane takes
the other one.

## Catppuccin

Four flavours: **latte** (light), **frappé**, **macchiato**, **mocha** (dark).
Source: [catppuccin.com/palette](https://catppuccin.com/palette/).

| role | latte | frappé | macchiato | mocha |
|---|---|---|---|---|
| base | `#eff1f5` | `#303446` | `#24273a` | `#1e1e2e` |
| mantle (elevated) | `#e6e9ef` | `#292c3c` | `#1e2030` | `#181825` |
| surface1 (border) | `#bcc0cc` | `#51576d` | `#494d64` | `#45475a` |
| overlay0 (dimmed) | `#9ca0b0` | `#737994` | `#6e738d` | `#6c7086` |
| text | `#4c4f69` | `#c6d0f5` | `#cad3f5` | `#cdd6f4` |
| subtext0 | `#6c6f85` | `#a5adce` | `#a5adcb` | `#a6adc8` |
| accent — human | `#8839ef` mauve | `#ca9ee6` | `#c6a0f6` | `#cba6f7` |
| agent | `#179299` teal | `#81c8be` | `#8bd5ca` | `#94e2d5` |
| ok | `#40a02b` green | `#a6d189` | `#a6da95` | `#a6e3a1` |
| warn | `#fe640b` peach | `#ef9f76` | `#f5a97f` | `#fab387` |
| error | `#d20f39` red | `#e78284` | `#ed8796` | `#f38ba8` |
| key | `#df8e1d` yellow | `#e5c890` | `#eed49f` | `#f9e2af` |
| info | `#1e66f5` blue | `#8caaee` | `#8aadf4` | `#89b4fa` |

Mauve is the accent; teal is the second. Catppuccin's style guide selects with
a translucent **Overlay 2** — a terminal has no alpha, so the accent is used
whole.

## Dracula

Two variants: **Dracula Classic** (dark) and **Alucard Classic** (light).
Source: [draculatheme.com/spec](https://draculatheme.com/spec).

| role | dracula | alucard |
|---|---|---|
| background | `#282A36` | `#FFFBEB` |
| background darker (elevated) | `#191A21` | `#BCBAB3` |
| current line (border) | `#6272A4` | `#6C664B` |
| selection | `#44475A` | `#CFCFDE` |
| foreground | `#F8F8F2` | `#1F1F1F` |
| comment (dimmed) | `#6272A4` | `#6C664B` |
| accent — human | `#BD93F9` purple | `#644AC9` |
| agent | `#8BE9FD` cyan | `#036A96` |
| ok | `#50FA7B` green | `#14710A` |
| warn | `#FFB86C` orange | `#A34D14` |
| error | `#FF5555` red | `#CB3A2A` |
| key | `#F1FA8C` yellow | `#846E15` |
| info | `#FF79C6` pink | `#A3144D` |

Dracula's status colours are **syntax** colours — the spec assigns
Green/Orange/Red/Yellow to code tokens, and a terminal theme reuses them.
Faithful to Dracula, not an oversight. The spec also publishes *functional*
colours for UI (`#DE5735` red, `#A39514` orange, `#089108` green, `#0081D6`
cyan, `#815CD6` purple), marked **not for terminal applications** — we use
the syntax ones.

## Nord

One palette, sixteen colours, four named groups: **polar night** (nord0–3),
**snow storm** (nord4–6), **frost** (nord7–10), **aurora** (nord11–15).
Source: [nordtheme.com/docs/colors-and-palettes](https://www.nordtheme.com/docs/colors-and-palettes).

| role | nord (dark) | nord-light |
|---|---|---|
| base | `#2e3440` nord0 | `#d8dee9` nord4 |
| elevated | `#3b4252` nord1 | `#e5e9f0` nord5 |
| border / dimmed | `#4c566a` nord3 | `#4c566a` nord3 |
| text | `#eceff4` nord6 | `#2e3440` nord0 |
| accent — human | `#88c0d0` nord8 | `#88c0d0` nord8 |
| agent | `#8fbcbb` nord7 | `#8fbcbb` nord7 |
| ok | `#a3be8c` nord14 | `#a3be8c` nord14 |
| warn | `#ebcb8b` nord13 | `#ebcb8b` nord13 |
| error | `#bf616a` nord11 | `#bf616a` nord11 |
| key | `#ebcb8b` nord13 | `#ebcb8b` nord13 |
| info | `#5e81ac` nord10 | `#5e81ac` nord10 |

Nord's light variant follows Nord's own **bright ambiance** style: snow storm
as the base, polar night as the text. Nord's aurora colours work the same in
both.

## leaf's own

Four hand-tuned presets, not derived from a family: **arctic**, **forest**,
**ocean**, **solarized-dark**. The values are leaf's constants, taken
verbatim from `src/theme/presets.rs`. `ocean` is leaf's default and the model
for ours.

## The theme file

A theme is a TOML file with the slots under a section. The format is leaf's,
so a theme written for leaf can be read here:

```toml
# coding-lanes theme
[theme]
preset = "nord"

[colors]
chrome.border = "#3b4252"
chrome.border_focus = "#88c0d0"
chrome.title = "#eceff4"
chrome.tab_active = "#eceff4"
chrome.tab_inactive = "#d8dee9"
chrome.tab_index = "#4c566a"
chrome.footer = "#4c566a"
chrome.key = "#ebcb8b"
text.normal = "#eceff4"
text.dim = "#d8dee9"
text.strong = "#eceff4"
text.selected = "#88c0d0"
semantic.ok = "#a3be8c"
semantic.warn = "#ebcb8b"
semantic.error = "#bf616a"
semantic.info = "#5e81ac"
semantic.human = "#88c0d0"
semantic.agent = "#8fbcbb"
```

The file format is later; the presets ship as code first.

## The rules for a new theme

1. **Every slot gets a value.** A theme that leaves a slot unset falls back
   to the default, and the screen changes between presets — which breaks the
   rule that a theme changes the paint and never the words.
2. **Selection is the accent at full opacity.** A terminal has no alpha
   compositing, so a translucent selection does not exist. Every theme uses the
   accent colour whole. This is leaf's rule and Frond kept it.
3. **Dimmed is still readable.** `text.dim` is for secondary text, not for
   text that may be missed. If a family has no dimmed role, derive one from
   its muted foreground — do not reach for black.
4. **The human lane and the agent lane are different colours.** They are the
   two halves of the product; they must never share one.
5. **Status semantics come from the family's own semantics.** If a family has
   no warning colour, it has no warning colour — use its accent and say so in
   the theme's note, rather than inventing a hue.
6. **A theme is a palette, not a mood.** No new colours beyond the family's
   roles, except where rule 5 applies.
7. **The preset changes nothing but colour.** The same words, the same layout,
   the same counts. This is checked by the render tests, which run against
   every preset.

## Not yet

- the three families as presets — the values above are the work plan; the
  mapping is done, the code is not
- the theme file on disk
- a user theme that is not one of the built-ins
