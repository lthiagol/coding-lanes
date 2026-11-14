# Palettes

Where raul's themes come from, and how to write a new one.

Our slots are defined in [theme.md](theme.md). This doc is the **source**
side: the palettes a theme may be built from, where each one came from, and
the rules that keep a new theme consistent with the others.

## Provenance

| what | where it comes from | licence |
|---|---|---|
| the slot structure | ours, shaped by leaf's `UiTheme` | — |
| leaf's four presets — `arctic`, `forest`, `ocean`, `solarized-dark` | leaf's `src/theme/presets.rs` | MIT |
| the fourteen families | the user's own leaf theme files, ported to Frond's mapping | MIT, each family's own terms |
| the role names of each family | the family's published style guide | — |
| the mapping rules | [Frond's `COLORS.md`](https://github.com/lthiagol/Frond/blob/main/docs/COLORS.md), adapted to our slots | ours |

Frond is our own other project. It did this exact work — the same upstream,
the same slots idea — for the same reason. Where its mapping says a role comes
from a source key, that answer is the answer here too.

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

The last two are ours: a human lane and an agent lane need two colours that
are not "success" and "error". The accent and the second accent are the
natural pair — the accent is already the focus colour, so the agent lane takes
the other one.

## The families

Fourteen built-ins, in six families. The role names are the family's own.

### Catppuccin — latte, frappe, macchiato, mocha

`base` · `mantle` · `surface1` · `overlay0` · `lavender`/`mauve` · `green` ·
`peach` · `red` · `yellow`

content = **Base**, sidebar and status = **Mantle**, borders = **Surface 1**,
dimmed = **Overlay 0**, selection = the flavour's accent (**Lavender** or
**Mauve**), status semantics = **Green**, **Peach**, **Red**, **Yellow**.

### Dracula — dracula, dracula-alucard

`background` · `background_darker` · `comment` · `current_line` · `purple` ·
`green` · `orange` · `red` · `yellow`

content = **Background**, sidebar and status = **Background Darker**, dimmed =
**Comment**, borders from the **Current Line** family, selection = **Purple**.
Dracula's status colours are **syntax** colours — the spec assigns
Green/Orange/Red/Yellow to code tokens, and leaf reuses them. Faithful to
Dracula, not an oversight.

### Nord

`nord0` · `nord1` · `nord3` · `nord8` · `nord11` · `nord13` · `nord14`

content = **nord0**, sidebar and status = **nord1**, dimmed and borders =
**nord3**, selection and accent = **nord8**, status semantics = the **aurora**
colours: **nord14** green, **nord13** yellow, **nord11** red.

### Rosé Pine — main, dawn, moon

`base` · `surface` · `muted` · `highlightMed` · `highlightHigh` · `overlay` ·
`rose` · `foam` · `gold` · `love`

content = **base**, sidebar and status = **surface**, dimmed = **muted**,
borders = **highlightMed**/**highlightHigh**, selection = **rose**, status
semantics = **foam** green, **gold** warning, **love** error.

### Tokyo Night — day, night, storm

`bg` · `bg_dark` · a muted blue-grey · `blue`

content = **bg**, sidebar and status = **bg_dark**, dimmed = the muted
blue-grey, selection = **blue**.

### gruvbox

`bg0` · `bg0_h` · `bg1` · `bg2` · `gray` · `orange` · `yellow` · `green` · `red`

content = **bg0**, sidebar and status = **bg0_h** (hard dark background — the
community convention for chrome), borders = **bg1**, dimmed = **gray**,
selection = **orange**.

### leaf's own — arctic, forest, ocean, solarized-dark

Hand-tuned by leaf, not derived from a family. The values are leaf's
constants, taken verbatim. `ocean` is leaf's default and the model for our
own default.

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
semantic.info = "#81a1c1"
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
5. **Status semantics come from the family's own semantics.** If a family
   has no warning colour, it has no warning colour — use its accent and say
   so in the theme's note, rather than inventing a hue.
6. **A theme is a palette, not a mood.** No new colours beyond the family's
   roles, except where rule 5 applies.
7. **The preset changes nothing but colour.** The same words, the same
   layout, the same counts. This is checked by the render tests, which run
   against every preset.

## Not yet

- the fourteen families as presets — the mapping above is the work plan; the
  values come from each family's guide and from the user's leaf theme files
- the theme file on disk
- a user theme that is not one of the built-ins
