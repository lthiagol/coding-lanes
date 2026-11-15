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
| **the dark palette** — our own hand-tuned preset | ours, like leaf's four own presets | — |
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
| `chrome.title` | `status_brand_fg` | the family's brightest foreground |
| `chrome.tab_active` | `toc_primary_active` | the family's main foreground |
| `chrome.tab_inactive` | `toc_primary_inactive` | the family's second foreground |
| `chrome.tab_index` | `toc_index_inactive` | the family's dimmed foreground |
| `chrome.border` | `toc_border` | the family's border colour |
| `chrome.border_focus` | `toc_accent` | the family's accent |
| `chrome.footer` | `status_shortcut_fg` | the family's dimmed foreground — its muted one where the dimmed is a border grade |
| `chrome.key` | `status_search_fg` | the family's attention colour |
| `text.normal` | `toc_hover_fg` | the family's main foreground |
| `text.dim` | `toc_header_fg` | the family's muted foreground |
| `text.strong` | `toc_secondary_text_active` | a raised foreground |
| `text.selected` | `toc_active_bg` | the accent, at full opacity |
| `semantic.ok` | `status_success_fg` | the family's success colour |
| `semantic.warn` | `status_warning_fg` | the family's warning colour |
| `semantic.error` | `status_error_fg` | the family's error colour |
| `semantic.info` | `status_filename_fg` | the family's nearest-neutral foreground |
| `semantic.human` | `toc_accent` | the family's accent |
| `semantic.agent` | `status_watch_fg` | the family's second accent |

The last two are ours: the human lane and the agent lane need two colours that
are not "success" and "error". The accent and the second accent are the
natural pair — the accent is already the focus colour, so the agent lane takes
the other one.

A family that has no role of its own for one of these takes the colour it
derives for it, and says so in its note below. A family whose dimmed
foreground is too faint for a word — a border grade — lends the footer's
labels its muted foreground instead, and says so too.

`semantic.info` takes a neutral foreground, not a hue of its own: information
is not a lane, and it does not compete with the human lane for attention.

**The test.** A colour is neutral when the spread between its three channels
— the highest minus the lowest — is at most 48 of 255, under a fifth of the
range. It is a tolerance, not an equality: a neutral need not have three
equal channels. The code holds this same test, and it reads the family
tables below.

**How a neutral is derived.** A family's page does not always name a
neutral, so it is derived from the roles the page does have: it is the
family's *nearest-neutral foreground* — of the family's **word greys** (the
grades it paints text with, not its accents and not the chrome grades it
draws lines with), the one whose three channels lie closest together, inside
the tolerance. A word grey the theme spends on another slot is not a
candidate: the main text colour is `text.normal` and the tab that is showing,
the muted one is `text.dim`, the raised one is `text.strong`, the title one is
`chrome.title`, the tabs that are not showing are `chrome.tab_inactive`, the
footer's labels are `chrome.footer` and the index is `chrome.tab_index`. A
chrome grade stays a chrome grade: a family that borrows one for a word does
not make it a candidate. Where the family leaves no such foreground, or spends
every one of them on another role, the main foreground is the neutral — the
way Dracula's does. Each family's table below names the role it is and writes
the derivation down.

A family's own blue or pink is kept for the record, and no slot spends it:
every one of those is well outside the tolerance.

## Ours: the dark palette

One hand-tuned palette, not derived from a family: ours, the way leaf's four
own presets are ours. It is the default, and it is the baseline a later
family theme is compared against.

Its values follow the mapping above: the accent is the focus border, the
selection and the human lane; the second accent is the agent lane; the
information colour is a neutral foreground. The selected row is the accent
whole, with the background colour as its foreground: 9.7:1.

**The palette assumes a terminal with a dark background, and raul paints no
background of its own.** Every figure below is measured against `#0d1117`,
the background the palette is measured against.

The eighteen slots, with the role each value plays and the attributes it
carries. The doc is the source of truth: the code is checked against this
table by a test.

| slot | role | foreground | background | attributes |
|---|---|---|---|---|
| `chrome.title` | the brightest foreground | `#ffffff` | — | bold |
| `chrome.tab_active` | the main foreground | `#c9d1d9` | — | bold |
| `chrome.tab_inactive` | the second foreground | `#9aa5b1` | — | — |
| `chrome.tab_index` | the dimmed foreground | `#7d8590` | — | — |
| `chrome.border` | the border colour | `#3d444d` | — | — |
| `chrome.border_focus` | the accent | `#79c0ff` | — | — |
| `chrome.footer` | the dimmed foreground | `#7d8590` | — | — |
| `chrome.key` | the attention colour | `#ffd166` | — | bold |
| `text.normal` | the main foreground | `#c9d1d9` | — | — |
| `text.dim` | the muted foreground | `#8b949e` | — | — |
| `text.strong` | the raised foreground | `#e6edf3` | — | bold |
| `text.selected` | the accent, at full opacity | `#0d1117` | `#79c0ff` | — |
| `semantic.ok` | the success colour | `#7ec699` | — | — |
| `semantic.warn` | the warning colour | `#ff9e64` | — | — |
| `semantic.error` | the error colour | `#e06c75` | — | — |
| `semantic.info` | a neutral foreground | `#b0b0b0` | — | — |
| `semantic.human` | the accent | `#79c0ff` | — | — |
| `semantic.agent` | the second accent | `#d2a8ff` | — | — |

**The agent lane does not use blue.** ANSI blue is `#0000ee`: about 2.0:1 on
the background, the lowest-contrast colour of the fifteen other than black,
and the agent lane is the lane the product is built around. The second accent
is a light violet, 9.7:1 on the background.

Contrast against `#0d1117`, for the record:

| colour | ratio | | colour | ratio |
|---|---|---|---|---|
| `#ffffff` | 18.9:1 | | `#79c0ff` accent | 9.7:1 |
| `#e6edf3` | 16.0:1 | | `#d2a8ff` agent | 9.7:1 |
| `#ffd166` | 13.1:1 | | `#7ec699` ok | 9.4:1 |
| `#c9d1d9` | 12.3:1 | | `#ff9e64` warn | 9.3:1 |
| `#b0b0b0` | 8.7:1 | | `#e06c75` error | 5.9:1 |
| `#9aa5b1` | 7.6:1 | | `#8b949e` dim | 6.2:1 |
| | | | `#7d8590` dimmed | 5.1:1 |
| | | | `#3d444d` border | 1.9:1 |

Every foreground that carries text is at 5.1:1 or better. The border is the
one value below that: it is a line, not a word.

## Catppuccin

Four flavours: **latte** (light), **frappé**, **macchiato**, **mocha** (dark).
Source: [catppuccin.com/palette](https://catppuccin.com/palette/).

The base is `#303446` in frappé, `#24273a` in macchiato and `#1e1e2e` in mocha
— the family's own. The theme paints no background of its own, and every
figure below is measured against the base of the flavour.

| slot | the family's role | frappé | macchiato | mocha |
|---|---|---|---|---|
| `chrome.title` | rosewater — the raised and the brightest | `#f2d5cf` | `#f4dbd6` | `#f5e0dc` |
| `chrome.tab_active` | text — the main foreground | `#c6d0f5` | `#cad3f5` | `#cdd6f4` |
| `chrome.tab_inactive` | overlay2 — borrowed: the second foreground | `#949cbb` | `#939ab7` | `#9399b2` |
| `chrome.tab_index` | overlay0 — the dimmed chrome | `#737994` | `#6e738d` | `#6c7086` |
| `chrome.border` | surface1 — the border | `#51576d` | `#494d64` | `#45475a` |
| `chrome.border_focus` | mauve — the accent | `#ca9ee6` | `#c6a0f6` | `#cba6f7` |
| `chrome.footer` | subtext1 — borrowed: the muted foreground | `#b5bfe2` | `#b8c0e0` | `#bac2de` |
| `chrome.key` | yellow — the attention colour | `#e5c890` | `#eed49f` | `#f9e2af` |
| `text.normal` | text — the main foreground | `#c6d0f5` | `#cad3f5` | `#cdd6f4` |
| `text.dim` | subtext1 — the muted foreground | `#b5bfe2` | `#b8c0e0` | `#bac2de` |
| `text.strong` | rosewater — the raised foreground | `#f2d5cf` | `#f4dbd6` | `#f5e0dc` |
| `text.selected` | mauve — the accent, at full opacity | `#ca9ee6` | `#c6a0f6` | `#cba6f7` |
| `semantic.ok` | green — the success colour | `#a6d189` | `#a6da95` | `#a6e3a1` |
| `semantic.warn` | peach — the warning colour | `#ef9f76` | `#f5a97f` | `#fab387` |
| `semantic.error` | red — the error colour | `#e78284` | `#ed8796` | `#f38ba8` |
| `semantic.info` | subtext0 — **the neutral** | `#a5adce` | `#a5adcb` | `#a6adc8` |
| `semantic.human` | mauve — the accent | `#ca9ee6` | `#c6a0f6` | `#cba6f7` |
| `semantic.agent` | teal — the second accent | `#81c8be` | `#8bd5ca` | `#94e2d5` |

Contrast against the base, for the record:

| slot | frappé | macchiato | mocha |
|---|---|---|---|
| `chrome.title` | 8.9:1 | 11.2:1 | 12.9:1 |
| `chrome.tab_active` | 8.1:1 | 9.9:1 | 11.3:1 |
| `chrome.tab_inactive` | 4.5:1 | 5.3:1 | 5.8:1 |
| `chrome.tab_index` | 2.9:1 | 3.2:1 | 3.4:1 |
| `chrome.border` | 1.7:1 | 1.8:1 | 1.8:1 |
| `chrome.border_focus` | 5.6:1 | 6.8:1 | 8.1:1 |
| `chrome.footer` | 6.8:1 | 8.2:1 | 9.3:1 |
| `chrome.key` | 7.6:1 | 10.2:1 | 12.9:1 |
| `text.normal` | 8.1:1 | 9.9:1 | 11.3:1 |
| `text.dim` | 6.8:1 | 8.2:1 | 9.3:1 |
| `text.strong` | 8.9:1 | 11.2:1 | 12.9:1 |
| `text.selected` | 5.6:1 | 6.8:1 | 8.1:1 |
| `semantic.ok` | 7.1:1 | 9.2:1 | 11.0:1 |
| `semantic.warn` | 5.8:1 | 7.6:1 | 9.3:1 |
| `semantic.error` | 4.6:1 | 6.0:1 | 7.1:1 |
| `semantic.info` | 5.6:1 | 6.6:1 | 7.4:1 |
| `semantic.human` | 5.6:1 | 6.8:1 | 8.1:1 |
| `semantic.agent` | 6.4:1 | 8.7:1 | 11.0:1 |

**The title and the raised text borrow `rosewater`.** Catppuccin names no
raised foreground: in the dark flavours every grey it has is dimmer than its
text colour, and the brightest colour the family has is `rosewater`. It is an
accent, not one of the word greys, so it is not a candidate for the neutral,
and it is spent on the title and the raised text besides. In mocha its three
channels are 25 apart, inside the tolerance of a neutral, and it is still not
one: the neutral is drawn from the word greys.

**The tabs that are not showing borrow `overlay2`.** Catppuccin's three word
greys are spent — on the ordinary text, the dimmed text and the information —
so the second foreground borrows the brightest overlay. The overlay is a
chrome grade, and a chrome grade stays one: it is not a candidate for the
neutral.

**The footer's labels borrow the muted foreground.** `overlay0` is a border
grade — 2.9:1 in frappé — and the footer's labels are words. The index keeps
the border grade; the labels take `subtext1`.

**The neutral is `subtext0`.** Catppuccin's greys below the text are
`subtext1` and `subtext0`. The text colour is spent on `text.normal` and
`subtext1` on the dimmed text and the footer's labels, so `subtext0` is the
grey the information takes. It is inside the tolerance in every flavour — 25
apart in latte, 41 in frappé, 38 in macchiato and 34 in mocha — and nearer to
a plain grey than `subtext1` in every one of them.

**The accent is mauve, and the second accent is teal.** Catppuccin's style
guide selects with a translucent **Overlay 2** — a terminal has no alpha, so
the selection is the accent whole: the base colour on the accent, 5.6:1 in
frappé.

**The blue is kept for the record.** It is not used by any slot.

## Dracula

Two variants: **Dracula Classic** (dark) and **Alucard Classic** (light).
Source: [draculatheme.com/spec](https://draculatheme.com/spec).

The base is `#282A36` — the family's own background. The theme paints no
background of its own, and every figure below is measured against it.

| slot | the family's role | dracula |
|---|---|---|
| `chrome.title` | ANSI bright white — the raised and the brightest | `#FFFFFF` |
| `chrome.tab_active` | foreground — the main foreground | `#F8F8F2` |
| `chrome.tab_inactive` | comment — borrowed: the second foreground | `#6272A4` |
| `chrome.tab_index` | comment — the dimmed foreground | `#6272A4` |
| `chrome.border` | current line — the border | `#6272A4` |
| `chrome.border_focus` | purple — the accent | `#BD93F9` |
| `chrome.footer` | comment — the dimmed foreground | `#6272A4` |
| `chrome.key` | yellow — the attention colour | `#F1FA8C` |
| `text.normal` | foreground — the main foreground | `#F8F8F2` |
| `text.dim` | comment — the muted foreground | `#6272A4` |
| `text.strong` | ANSI bright white — the raised foreground | `#FFFFFF` |
| `text.selected` | purple — the accent, at full opacity | `#BD93F9` |
| `semantic.ok` | green — the success colour | `#50FA7B` |
| `semantic.warn` | orange — the warning colour | `#FFB86C` |
| `semantic.error` | red — the error colour | `#FF5555` |
| `semantic.info` | foreground — **the neutral** | `#F8F8F2` |
| `semantic.human` | purple — the accent | `#BD93F9` |
| `semantic.agent` | cyan — the second accent | `#8BE9FD` |

Contrast against the base, for the record:

| slot | dracula |
|---|---|
| `chrome.title` | 14.2:1 |
| `chrome.tab_active` | 13.4:1 |
| `chrome.tab_inactive` | 3.0:1 |
| `chrome.tab_index` | 3.0:1 |
| `chrome.border` | 3.0:1 |
| `chrome.border_focus` | 5.9:1 |
| `chrome.footer` | 3.0:1 |
| `chrome.key` | 12.7:1 |
| `text.normal` | 13.4:1 |
| `text.dim` | 3.0:1 |
| `text.strong` | 14.2:1 |
| `text.selected` | 5.9:1 |
| `semantic.ok` | 10.4:1 |
| `semantic.warn` | 8.4:1 |
| `semantic.error` | 4.5:1 |
| `semantic.info` | 13.4:1 |
| `semantic.human` | 5.9:1 |
| `semantic.agent` | 10.3:1 |

Dracula's status colours are **syntax** colours — the spec assigns
Green/Orange/Red/Yellow to code tokens, and a terminal theme reuses them.
Faithful to Dracula, not an oversight. The spec also publishes *functional*
colours for UI (`#DE5735` red, `#A39514` orange, `#089108` green, `#0081D6`
cyan, `#815CD6` purple), marked **not for terminal applications** — we use
the syntax ones.

**The neutral is the `foreground`.** Dracula publishes two near-grey word
colours, the foreground and the ANSI bright white, and no other: the comment
is a blue of its own, 66 channels apart, well outside the tolerance, and the
selection is a background and not a word colour at all. The foreground's
three channels are 6 apart. Both greys are spent on other roles — the bright
white on the title and the raised text, the foreground on the ordinary text —
so the information takes the main foreground, the way the rule says.

**The raised and the brightest foreground are the ANSI bright white.**
Dracula's spec publishes an ANSI palette for terminal applications, and raul
is one: its bright white is `#FFFFFF`, 14.2:1 on the background, a step
above the plain foreground `#F8F8F2` at 13.4:1.

**The tabs that are not showing borrow the comment.** Dracula names no second
foreground: besides the two whites it publishes one dimmed grey. The comment
reads at 3.0:1 on the background, and it carries the muted text, the dimmed
chrome and the tabs that are not showing. The spec gives the current line and
the comment the same value on purpose, so the border is that grey too.

**The selection is the accent, whole.** Dracula selects with its own
selection colour `#44475A`; a terminal has no alpha, so the accent is used
whole: the base colour on the accent, 5.9:1.

**The pink is kept for the record.** It is not used by any slot.

## Nord

One palette, sixteen colours, four named groups: **polar night** (nord0–3),
**snow storm** (nord4–6), **frost** (nord7–10), **aurora** (nord11–15).
Source: [nordtheme.com/docs/colors-and-palettes](https://www.nordtheme.com/docs/colors-and-palettes).

The base is `#2e3440` — nord0, the family's own. The theme paints no
background of its own, and every figure below is measured against it.

| slot | the family's role | nord |
|---|---|---|
| `chrome.title` | nord6 — the raised and the brightest | `#eceff4` |
| `chrome.tab_active` | nord5 — the main foreground | `#e5e9f0` |
| `chrome.tab_inactive` | nord9 — the frost, the second foreground | `#81a1c1` |
| `chrome.tab_index` | nord2 — the dimmed chrome | `#434c5e` |
| `chrome.border` | nord3 — the border | `#4c566a` |
| `chrome.border_focus` | nord8 — the accent | `#88c0d0` |
| `chrome.footer` | nord4 — borrowed: the muted foreground | `#d8dee9` |
| `chrome.key` | nord13 — the attention colour | `#ebcb8b` |
| `text.normal` | nord5 — the main foreground | `#e5e9f0` |
| `text.dim` | nord4 — the muted foreground | `#d8dee9` |
| `text.strong` | nord6 — the raised foreground | `#eceff4` |
| `text.selected` | nord8 — the accent, at full opacity | `#88c0d0` |
| `semantic.ok` | nord14 — the success colour | `#a3be8c` |
| `semantic.warn` | nord13 — the warning colour | `#ebcb8b` |
| `semantic.error` | nord11 — the error colour | `#bf616a` |
| `semantic.info` | nord5 — **the neutral** | `#e5e9f0` |
| `semantic.human` | nord8 — the accent | `#88c0d0` |
| `semantic.agent` | nord7 — the second accent | `#8fbcbb` |

Contrast against the base, for the record:

| slot | nord |
|---|---|
| `chrome.title` | 10.8:1 |
| `chrome.tab_active` | 10.3:1 |
| `chrome.tab_inactive` | 4.6:1 |
| `chrome.tab_index` | 1.4:1 |
| `chrome.border` | 1.7:1 |
| `chrome.border_focus` | 6.2:1 |
| `chrome.footer` | 9.2:1 |
| `chrome.key` | 8.0:1 |
| `text.normal` | 10.3:1 |
| `text.dim` | 9.2:1 |
| `text.strong` | 10.8:1 |
| `text.selected` | 6.2:1 |
| `semantic.ok` | 6.1:1 |
| `semantic.warn` | 8.0:1 |
| `semantic.error` | 3.1:1 |
| `semantic.info` | 10.3:1 |
| `semantic.human` | 6.2:1 |
| `semantic.agent` | 6.0:1 |

Nord's light variant follows Nord's own **bright ambiance** style: snow storm
as the base, polar night as the text. Nord's aurora colours work the same in
both.

**Nord's greys carry the words, and there are three of them.** Nord's greys
are the polar night (nord0 to nord3) and the snow storm (nord4 to nord6).
The polar night is a chrome grade — the base, the elevated chrome, the dimmed
chrome and the border — and on the family's own base it is faint: nord3 reads
at 1.7:1. The snow storm is what carries words, and it is three greys for
four roles: the muted text, the ordinary text, and the raised and the
brightest.

**The muted text is derived: the dimmest snow storm grey, nord4.** Nord has
no muted foreground of its own for words — the greys other than the snow
storm are the polar night, and they are a chrome grade: nord3 reads at 1.7:1
on the base, which is a border, not a word, and rule 3 says the muted text is
still readable. So the muted text is the dimmest snow storm grey, nord4, which
reads at 9.2:1: a step down from the text, and still a word.

**The neutral gives way.** The snow storm's three greys are all spent on the
text roles, so no foreground is left for the information; the information
takes the main foreground, nord5, the way Dracula's does. It is 11 channels
apart — nearer to a plain grey than the muted text's nord4, which is 17. The
frost and the aurora are hues and not word greys: nord7 is 45 channels apart
and near enough to the tolerance, but it is the agent lane.

**nord10 is kept for the record.** It is a hue of its own, 78 channels apart,
and no slot spends it; the information takes a neutral, and a hue is not one.

**The second foreground is the frost, nord9.** Nord's palette names no second
foreground; the theme takes nord9 for the tabs that are not showing, and it
reads at 4.6:1 on the base.

**The footer's labels borrow the muted text.** nord2 is a border grade — 1.4:1
on the base — and the footer's labels are words. The index keeps the border
grade; the labels take nord4.

**The error is the family's own red.** nord11 reads at 3.1:1 on the base: the
aurora is low-contrast on the polar night, and rule 7 asks for the family's
own semantics, so the failure keeps the family's red.

**The light variant waits with the light theme.** Nord Light spends the snow
storm on its base and takes its words from the polar night, so both its muted
text and its neutral wait with the light theme. Its palette is recorded in
*Not yet*.

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
chrome.title = "#eceff4"
chrome.tab_active = "#e5e9f0"
chrome.tab_inactive = "#81a1c1"
chrome.tab_index = "#434c5e"
chrome.border = "#4c566a"
chrome.border_focus = "#88c0d0"
# the footer's labels borrow the muted text: the dimmed chrome is a border grade
chrome.footer = "#d8dee9"
chrome.key = "#ebcb8b"
text.normal = "#e5e9f0"
text.dim = "#d8dee9"
text.strong = "#eceff4"
text.selected = "#88c0d0"
semantic.ok = "#a3be8c"
semantic.warn = "#ebcb8b"
semantic.error = "#bf616a"
# the neutral: no grey is left, so the information takes the main foreground
semantic.info = "#e5e9f0"
semantic.human = "#88c0d0"
semantic.agent = "#8fbcbb"
```

The file format is later; the presets ship as code first.

## The rules for a new theme

1. **Every slot gets a value.** A theme that leaves a slot unset falls back
   to the default, and the screen changes between presets — which breaks the
   rule that a theme changes the paint and never the words.
2. **Selection is the accent at full opacity.** A terminal has no alpha
   compositing, so a translucent selection does not exist. Every colour theme
   uses the accent colour whole. This is leaf's rule and Frond kept it.
3. **Dimmed is still readable.** `text.dim` is for secondary text, not for
   text that may be missed: it reads at 3:1 or better on the background it
   sits on. If a family has no dimmed role, derive one from its muted
   foreground — do not reach for black. A theme that takes its muted text
   from another of the family's colours says so in its note.
4. **Chrome is not a word.** A border, an index and the tab bar's chrome may
   sit below 3:1 — they are lines. The footer's labels are words and are
   not: a family whose dimmed foreground is a border grade lends them its
   muted foreground instead, and says so in its note.
5. **The human lane and the agent lane are different colours.** They are the
   two halves of the product; they must never share one.
6. **The lanes are readable.** The lane colours carry the two headlines the
   product is built around, so they are picked to read on the background
   they sit on — ANSI blue on the background is 2.0:1, which is not
   readable, and the agent lane does not use it.
7. **Status semantics come from the family's own semantics.** If a family has
   no warning colour, it has no warning colour — use its accent and say so in
   the theme's note, rather than inventing a hue.
8. **The information colour is a neutral.** It is the family's
   nearest-neutral foreground, derived and named above. A theme that has no
   such role — or that spends every one of them on another role — says in
   its note where its neutral comes from.
9. **A theme is a palette, not a mood.** No new colours beyond the family's
   roles, except where rule 6 applies.
10. **The preset changes nothing but colour.** The same words, the same layout,
    the same counts. This is checked by the render tests, which run against
    every preset, on every tab.

## Not yet

- the three families as presets — the values above are the work plan; the
  mapping is done, the code is not
- the theme file on disk
- a user theme that is not one of the built-ins

**The light variants are deferred with the light theme**: Catppuccin Latte,
Dracula Alucard and Nord Light wait until that changes. Their palettes are
recorded here, so they are ready when the time comes:

- **Latte**: base `#eff1f5`, mantle `#e6e9ef`, crust `#dce0e8`, surface0
  `#ccd0da`, surface1 `#bcc0cc`, surface2 `#acb0be`, overlay0 `#9ca0b0`,
  overlay1 `#8c8fa1`, overlay2 `#7c7f93`, text `#4c4f69`, subtext0 `#6c6f85`,
  subtext1 `#5c5f77`, rosewater `#dc8a78`, mauve `#8839ef`, teal `#179299`,
  green `#40a02b`, peach `#fe640b`, red `#d20f39`, yellow `#df8e1d`, blue
  `#1e66f5`.
- **Alucard**: background `#FFFBEB`, background darker `#BCBAB3`, current line
  `#6C664B`, selection `#CFCFDE`, foreground `#1F1F1F`, comment `#6C664B`,
  purple `#644AC9`, cyan `#036A96`, green `#14710A`, orange `#A34D14`, red
  `#CB3A2A`, yellow `#846E15`, pink `#A3144D`.
- **Nord Light**: base `#d8dee9` nord4, elevated `#e5e9f0` nord5, dimmed
  chrome `#434c5e` nord2, border `#4c566a` nord3, main foreground `#2e3440`
  nord0, raised `#eceff4` nord6, second foreground `#81a1c1` nord9, accent
  `#88c0d0` nord8, agent `#8fbcbb` nord7, ok `#a3be8c` nord14, warn `#ebcb8b`
  nord13, error `#bf616a` nord11, key `#ebcb8b` nord13, frost `#5e81ac`
  nord10.
