//! The theme: the colours of raul.
//!
//! A slot names a role; a preset decides what the role looks like. The
//! domain knows nothing about this: `crates/core` never names a colour, and
//! a phase names a slot, never a colour.

use coding_lanes_core::Status;
use ratatui::style::{Modifier, Style};

use crate::data::Reason;

/// The role a colour plays.
///
/// The name of the slot is the name of the role, not of a colour: the same
/// slot is painted differently by every preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot {
    /// The title line, the product name.
    ChromeTitle,
    /// The tab that is showing.
    ChromeTabActive,
    /// The other tabs.
    ChromeTabInactive,
    /// The `3/6` index.
    ChromeTabIndex,
    /// A block's border.
    ChromeBorder,
    /// The border of the focused block.
    ChromeBorderFocus,
    /// The footer's text.
    ChromeFooter,
    /// A key label in the footer and in the help overlay.
    ChromeKey,
    /// Ordinary text.
    TextNormal,
    /// Secondary text: the empty state, a marker, a hint under a line, a
    /// separator, a path, a tab that is not built yet.
    TextDim,
    /// A count, a title that matters.
    TextStrong,
    /// The selected row.
    TextSelected,
    /// A clean result, a finished task.
    SemanticOk,
    /// Something wants attention and is not broken.
    SemanticWarn,
    /// A failure: an unreadable file, a failed criterion, an escalation, no
    /// coding-lanes folder found.
    SemanticError,
    /// Neutral information.
    ///
    /// The colour carries no hue of its own: the spread between its
    /// highest and lowest channel is at most 48 of 255.
    SemanticInfo,
    /// The human lane: approve, review, and the facts that wait on it.
    SemanticHuman,
    /// The agent lane: claimable, running.
    SemanticAgent,
}

impl Slot {
    /// Every slot, in the order of the spec.
    ///
    /// The list and the names are the keys of the theme file format,
    /// which comes later: they are pinned by the tests, and nothing draws
    /// a slot by name yet.
    #[allow(dead_code)]
    pub const ALL: [Slot; 18] = [
        Slot::ChromeTitle,
        Slot::ChromeTabActive,
        Slot::ChromeTabInactive,
        Slot::ChromeTabIndex,
        Slot::ChromeBorder,
        Slot::ChromeBorderFocus,
        Slot::ChromeFooter,
        Slot::ChromeKey,
        Slot::TextNormal,
        Slot::TextDim,
        Slot::TextStrong,
        Slot::TextSelected,
        Slot::SemanticOk,
        Slot::SemanticWarn,
        Slot::SemanticError,
        Slot::SemanticInfo,
        Slot::SemanticHuman,
        Slot::SemanticAgent,
    ];

    /// The name of the slot, as the spec writes it.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Slot::ChromeTitle => "chrome.title",
            Slot::ChromeTabActive => "chrome.tab_active",
            Slot::ChromeTabInactive => "chrome.tab_inactive",
            Slot::ChromeTabIndex => "chrome.tab_index",
            Slot::ChromeBorder => "chrome.border",
            Slot::ChromeBorderFocus => "chrome.border_focus",
            Slot::ChromeFooter => "chrome.footer",
            Slot::ChromeKey => "chrome.key",
            Slot::TextNormal => "text.normal",
            Slot::TextDim => "text.dim",
            Slot::TextStrong => "text.strong",
            Slot::TextSelected => "text.selected",
            Slot::SemanticOk => "semantic.ok",
            Slot::SemanticWarn => "semantic.warn",
            Slot::SemanticError => "semantic.error",
            Slot::SemanticInfo => "semantic.info",
            Slot::SemanticHuman => "semantic.human",
            Slot::SemanticAgent => "semantic.agent",
        }
    }
}

/// The colours of the chrome: the frame around the content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chrome {
    pub title: Style,
    pub tab_active: Style,
    pub tab_inactive: Style,
    pub tab_index: Style,
    pub border: Style,
    pub border_focus: Style,
    pub footer: Style,
    pub key: Style,
}

/// The colours of the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text {
    pub normal: Style,
    pub dim: Style,
    pub strong: Style,
    pub selected: Style,
}

/// The colours that carry a meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Semantic {
    pub ok: Style,
    pub warn: Style,
    pub error: Style,
    pub info: Style,
    pub human: Style,
    pub agent: Style,
}

/// The colours of raul, one per slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub chrome: Chrome,
    pub text: Text,
    pub semantic: Semantic,
}

impl Theme {
    /// The theme of a preset.
    pub fn preset(preset: Preset) -> Self {
        let style = |slot: Slot| style_of(preset, slot);
        Theme {
            chrome: Chrome {
                title: style(Slot::ChromeTitle),
                tab_active: style(Slot::ChromeTabActive),
                tab_inactive: style(Slot::ChromeTabInactive),
                tab_index: style(Slot::ChromeTabIndex),
                border: style(Slot::ChromeBorder),
                border_focus: style(Slot::ChromeBorderFocus),
                footer: style(Slot::ChromeFooter),
                key: style(Slot::ChromeKey),
            },
            text: Text {
                normal: style(Slot::TextNormal),
                dim: style(Slot::TextDim),
                strong: style(Slot::TextStrong),
                selected: style(Slot::TextSelected),
            },
            semantic: Semantic {
                ok: style(Slot::SemanticOk),
                warn: style(Slot::SemanticWarn),
                error: style(Slot::SemanticError),
                info: style(Slot::SemanticInfo),
                human: style(Slot::SemanticHuman),
                agent: style(Slot::SemanticAgent),
            },
        }
    }

    /// The style of a slot.
    pub fn slot(&self, slot: Slot) -> Style {
        match slot {
            Slot::ChromeTitle => self.chrome.title,
            Slot::ChromeTabActive => self.chrome.tab_active,
            Slot::ChromeTabInactive => self.chrome.tab_inactive,
            Slot::ChromeTabIndex => self.chrome.tab_index,
            Slot::ChromeBorder => self.chrome.border,
            Slot::ChromeBorderFocus => self.chrome.border_focus,
            Slot::ChromeFooter => self.chrome.footer,
            Slot::ChromeKey => self.chrome.key,
            Slot::TextNormal => self.text.normal,
            Slot::TextDim => self.text.dim,
            Slot::TextStrong => self.text.strong,
            Slot::TextSelected => self.text.selected,
            Slot::SemanticOk => self.semantic.ok,
            Slot::SemanticWarn => self.semantic.warn,
            Slot::SemanticError => self.semantic.error,
            Slot::SemanticInfo => self.semantic.info,
            Slot::SemanticHuman => self.semantic.human,
            Slot::SemanticAgent => self.semantic.agent,
        }
    }
}

/// The slot that paints a phase.
///
/// A phase never names a colour; it names a slot.
pub fn phase_slot(phase: Status) -> Slot {
    match phase {
        // Nothing has been said about it yet.
        Status::Draft => Slot::TextDim,
        // A human approves it.
        Status::Groomed => Slot::SemanticHuman,
        // An agent can take it.
        Status::Approved => Slot::SemanticAgent,
        // An agent is on it.
        Status::Executing => Slot::SemanticAgent,
        // A human reviews it.
        Status::CodeReview => Slot::SemanticHuman,
        // It is being fixed.
        Status::Remediating => Slot::SemanticWarn,
        // The review budget ran out; a person has to step in.
        Status::NeedsHuman => Slot::SemanticError,
        // It is finished.
        Status::Done => Slot::SemanticOk,
    }
}

/// The style of a phase in a theme.
pub fn phase_style(theme: &Theme, phase: Status) -> Style {
    theme.slot(phase_slot(phase))
}

/// The slot that paints a reason.
///
/// A reason is a phase in words: the human lane is approve and review, and
/// an escalation is a failure.
pub fn reason_slot(reason: Reason) -> Slot {
    match reason {
        // A human approves it.
        Reason::Approve => Slot::SemanticHuman,
        // A human reviews it.
        Reason::Review => Slot::SemanticHuman,
        // The review budget ran out; a person has to step in.
        Reason::Escalated => Slot::SemanticError,
    }
}

/// The style of a reason in a theme.
pub fn reason_style(theme: &Theme, reason: Reason) -> Style {
    theme.slot(reason_slot(reason))
}

/// The style of the title of a block.
///
/// The title of a block is `text.strong`: a title that matters. It sits on
/// the border, and the border's attributes bleed into it, so the dimming of
/// a dimmed border is taken away again: a title is never dimmed.
pub fn block_title_style(theme: &Theme) -> Style {
    theme.text.strong.remove_modifier(Modifier::DIM)
}

/// The palettes raul ships with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    /// The dark palette. The default.
    Dark,
    /// The colours of the terminal itself.
    Terminal,
}

impl Preset {
    /// Every preset, in the order of the spec.
    pub const ALL: [Preset; 2] = [Preset::Dark, Preset::Terminal];

    /// The name of the preset, as the flag writes it.
    pub fn name(&self) -> &'static str {
        match self {
            Preset::Dark => "dark",
            Preset::Terminal => "terminal",
        }
    }

    /// The preset of a name, if there is one.
    pub fn parse(name: &str) -> Option<Self> {
        Preset::ALL
            .iter()
            .copied()
            .find(|preset| preset.name() == name)
    }
}

/// The style of a slot in a preset.
///
/// The slot is looked up by the enum, never by its name: the name is for
/// the theme file, and a name that is spelled twice is a name that can
/// disagree with itself.
fn style_of(preset: Preset, slot: Slot) -> Style {
    match preset {
        Preset::Dark => dark_style(slot),
        Preset::Terminal => terminal_style(slot),
    }
}

/// The colours of the dark palette.
///
/// The palette is ours: the eighteen values are written down in
/// docs/tui/palettes.md, with the role each one plays. The background is
/// the terminal's, and the palette assumes a dark one.
mod dark {
    use ratatui::style::Color;

    /// The background the palette is drawn on.
    pub const BACKGROUND: Color = Color::Rgb(0x0d, 0x11, 0x17);
    /// The main foreground: ordinary text, the tab that is showing.
    pub const FOREGROUND: Color = Color::Rgb(0xc9, 0xd1, 0xd9);
    /// The second foreground: the tabs that are not showing.
    pub const SECOND: Color = Color::Rgb(0x9a, 0xa5, 0xb1);
    /// The muted foreground: secondary text.
    pub const MUTED: Color = Color::Rgb(0x8b, 0x94, 0x9e);
    /// The dimmed foreground: the footer, the index.
    pub const DIMMED: Color = Color::Rgb(0x7d, 0x85, 0x90);
    /// The raised foreground: a count, a title that matters.
    pub const RAISED: Color = Color::Rgb(0xe6, 0xed, 0xf3);
    /// The brightest foreground: the name of the tool.
    pub const BRIGHTEST: Color = Color::Rgb(0xff, 0xff, 0xff);
    /// The border colour.
    pub const BORDER: Color = Color::Rgb(0x3d, 0x44, 0x4d);
    /// The accent: the focus border, the selection, the human lane.
    pub const ACCENT: Color = Color::Rgb(0x79, 0xc0, 0xff);
    /// The second accent: the agent lane.
    pub const SECOND_ACCENT: Color = Color::Rgb(0xd2, 0xa8, 0xff);
    /// A clean result, a finished task.
    pub const OK: Color = Color::Rgb(0x7e, 0xc6, 0x99);
    /// Something wants attention and is not broken.
    pub const WARN: Color = Color::Rgb(0xff, 0x9e, 0x64);
    /// A failure.
    pub const ERROR: Color = Color::Rgb(0xe0, 0x6c, 0x75);
    /// Neutral information: a foreground without a hue of its own.
    pub const INFO: Color = Color::Rgb(0xb0, 0xb0, 0xb0);
    /// The attention colour: a key label.
    pub const ATTENTION: Color = Color::Rgb(0xff, 0xd1, 0x66);
}

/// The style of a slot in the dark palette.
///
/// The palette is ours: its eighteen values are written down in
/// docs/tui/palettes.md, with the role each one plays. It assumes a
/// terminal with a dark background.
fn dark_style(slot: Slot) -> Style {
    use dark::{
        ACCENT, ATTENTION, BACKGROUND, BORDER, BRIGHTEST, DIMMED, ERROR, FOREGROUND, INFO, MUTED,
        OK, RAISED, SECOND, SECOND_ACCENT, WARN,
    };
    match slot {
        // The brightest foreground: the name of the tool.
        Slot::ChromeTitle => Style::new().fg(BRIGHTEST).add_modifier(Modifier::BOLD),
        // The main foreground, raised: the tab that is showing.
        Slot::ChromeTabActive => Style::new().fg(FOREGROUND).add_modifier(Modifier::BOLD),
        // The second foreground: the other tabs.
        Slot::ChromeTabInactive => Style::new().fg(SECOND),
        // The dimmed foreground: the `3/6` index.
        Slot::ChromeTabIndex => Style::new().fg(DIMMED),
        // The border colour.
        Slot::ChromeBorder => Style::new().fg(BORDER),
        // The accent: the border of the focused block.
        Slot::ChromeBorderFocus => Style::new().fg(ACCENT),
        // The dimmed foreground: the footer's text.
        Slot::ChromeFooter => Style::new().fg(DIMMED),
        // The attention colour: a key label.
        Slot::ChromeKey => Style::new().fg(ATTENTION).add_modifier(Modifier::BOLD),
        // The main foreground: ordinary text.
        Slot::TextNormal => Style::new().fg(FOREGROUND),
        // The muted foreground: secondary text.
        Slot::TextDim => Style::new().fg(MUTED),
        // The raised foreground: a count, a title that matters.
        Slot::TextStrong => Style::new().fg(RAISED).add_modifier(Modifier::BOLD),
        // The selection is the accent, whole: a terminal has no alpha
        // compositing, so a translucent selection is not there to be had.
        Slot::TextSelected => Style::new().fg(BACKGROUND).bg(ACCENT),
        // A clean result, a finished task.
        Slot::SemanticOk => Style::new().fg(OK),
        // Something wants attention and is not broken.
        Slot::SemanticWarn => Style::new().fg(WARN),
        // A failure.
        Slot::SemanticError => Style::new().fg(ERROR),
        // Neutral information: it does not compete with the human lane.
        Slot::SemanticInfo => Style::new().fg(INFO),
        // The human lane takes the accent.
        Slot::SemanticHuman => Style::new().fg(ACCENT),
        // The agent lane takes the second accent.
        Slot::SemanticAgent => Style::new().fg(SECOND_ACCENT),
    }
}

/// The style of a slot in the colours of the terminal.
///
/// The colours are the terminal's; what is left are the attributes every
/// terminal has. A slot with no attribute is the terminal's own text.
///
/// The six semantic slots are pairwise distinct, and none of them paints
/// like ordinary text: the phases are told apart by the attributes alone.
fn terminal_style(slot: Slot) -> Style {
    match slot {
        Slot::ChromeTitle => Style::new().add_modifier(Modifier::BOLD),
        Slot::ChromeTabActive => Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED),
        Slot::ChromeTabInactive => Style::new().add_modifier(Modifier::DIM),
        Slot::ChromeTabIndex => Style::new().add_modifier(Modifier::DIM),
        Slot::ChromeBorder => Style::new().add_modifier(Modifier::DIM),
        Slot::ChromeBorderFocus => Style::new().add_modifier(Modifier::BOLD),
        Slot::ChromeFooter => Style::new().add_modifier(Modifier::DIM),
        Slot::ChromeKey => Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED),
        Slot::TextNormal => Style::new(),
        Slot::TextDim => Style::new().add_modifier(Modifier::DIM),
        Slot::TextStrong => Style::new().add_modifier(Modifier::BOLD),
        Slot::TextSelected => Style::new().add_modifier(Modifier::REVERSED),
        // The six semantic slots are pairwise distinct, and none of them
        // paints like ordinary text: the phases are told apart by the
        // attributes alone.
        Slot::SemanticOk => Style::new().add_modifier(Modifier::ITALIC),
        Slot::SemanticWarn => Style::new().add_modifier(Modifier::UNDERLINED),
        Slot::SemanticError => Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
            .add_modifier(Modifier::UNDERLINED),
        Slot::SemanticInfo => Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::DIM),
        Slot::SemanticHuman => Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED),
        Slot::SemanticAgent => Style::new()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    /// The palettes doc: the source of truth for the dark palette.
    const PALETTES: &str = include_str!("../../../docs/tui/palettes.md");

    /// The tolerance of the neutral, in channels of 255.
    ///
    /// The doc states it; the test below holds the two together.
    const NEUTRAL_TOLERANCE: u8 = 48;

    /// Every phase of the lifecycle and the slot the spec gives it.
    fn phases() -> [(Status, Slot); 8] {
        [
            (Status::Draft, Slot::TextDim),
            (Status::Groomed, Slot::SemanticHuman),
            (Status::Approved, Slot::SemanticAgent),
            (Status::Executing, Slot::SemanticAgent),
            (Status::CodeReview, Slot::SemanticHuman),
            (Status::Remediating, Slot::SemanticWarn),
            (Status::NeedsHuman, Slot::SemanticError),
            (Status::Done, Slot::SemanticOk),
        ]
    }

    #[test]
    fn every_phase_maps_to_a_slot() {
        for (phase, slot) in phases() {
            assert_eq!(phase_slot(phase), slot, "{phase:?} maps elsewhere");
        }
    }

    #[test]
    fn every_slot_is_named() {
        // The names are the keys of the theme file format, so they are
        // pinned here: a typo in one of them is a slot that cannot be set
        // from a theme file.
        let names: Vec<&str> = Slot::ALL.iter().map(|slot| slot.name()).collect();
        assert_eq!(
            names,
            vec![
                "chrome.title",
                "chrome.tab_active",
                "chrome.tab_inactive",
                "chrome.tab_index",
                "chrome.border",
                "chrome.border_focus",
                "chrome.footer",
                "chrome.key",
                "text.normal",
                "text.dim",
                "text.strong",
                "text.selected",
                "semantic.ok",
                "semantic.warn",
                "semantic.error",
                "semantic.info",
                "semantic.human",
                "semantic.agent",
            ]
        );
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), Slot::ALL.len(), "two slots share a name");
        for slot in Slot::ALL {
            assert!(slot.name().contains('.'), "{} is not named", slot.name());
        }
    }

    #[test]
    fn every_reason_maps_to_a_slot() {
        assert_eq!(reason_slot(Reason::Approve), Slot::SemanticHuman);
        assert_eq!(reason_slot(Reason::Review), Slot::SemanticHuman);
        assert_eq!(reason_slot(Reason::Escalated), Slot::SemanticError);
    }

    #[test]
    fn the_semantic_slots_of_the_terminal_preset_are_told_apart() {
        let theme = Theme::preset(Preset::Terminal);
        let semantic = [
            Slot::SemanticOk,
            Slot::SemanticWarn,
            Slot::SemanticError,
            Slot::SemanticInfo,
            Slot::SemanticHuman,
            Slot::SemanticAgent,
        ];
        let normal = theme.slot(Slot::TextNormal);
        for slot in semantic {
            let style = theme.slot(slot);
            assert_ne!(style, normal, "{} paints like ordinary text", slot.name());
            for other in semantic {
                if other == slot {
                    continue;
                }
                assert_ne!(
                    style,
                    theme.slot(other),
                    "{} and {} paint alike",
                    slot.name(),
                    other.name()
                );
            }
        }
    }

    #[test]
    fn the_human_and_the_agent_lane_are_different_colours() {
        for preset in Preset::ALL {
            let theme = Theme::preset(preset);
            assert_ne!(
                theme.slot(Slot::SemanticHuman),
                theme.slot(Slot::SemanticAgent),
                "{} paints the two lanes alike",
                preset.name()
            );
        }
    }

    #[test]
    fn every_phase_looks_different_from_every_other_phase() {
        for preset in Preset::ALL {
            let theme = Theme::preset(preset);
            for (phase, slot) in phases() {
                for (other, other_slot) in phases() {
                    if phase == other || slot == other_slot {
                        // Two phases may share a slot: the spec says so.
                        continue;
                    }
                    assert_ne!(
                        theme.slot(slot),
                        theme.slot(other_slot),
                        "{} paints {phase:?} and {other:?} alike",
                        preset.name()
                    );
                }
            }
        }
    }

    #[test]
    fn every_slot_is_painted_in_the_dark_preset() {
        let theme = Theme::preset(Preset::Dark);
        for slot in Slot::ALL {
            // A slot that paints nothing is a slot that was forgotten.
            let style = theme.slot(slot);
            assert!(
                style.fg.is_some() || style.bg.is_some() || !style.add_modifier.is_empty(),
                "{} paints nothing",
                slot.name()
            );
        }
    }

    /// The relative luminance of a colour, by the WCAG formula.
    fn luminance(color: Color) -> f64 {
        let Color::Rgb(r, g, b) = color else {
            return 0.0;
        };
        let channel = |value: u8| {
            let value = f64::from(value) / 255.0;
            if value <= 0.03928 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
    }

    /// The contrast ratio of two colours, from 1 to 21.
    fn contrast(one: Color, other: Color) -> f64 {
        let (one, other) = (luminance(one), luminance(other));
        let (light, dark) = if one > other {
            (one, other)
        } else {
            (other, one)
        };
        (light + 0.05) / (dark + 0.05)
    }

    #[test]
    fn the_dark_preset_reads_on_its_background() {
        let theme = Theme::preset(Preset::Dark);
        let background = Color::Rgb(0x0d, 0x11, 0x17);
        for slot in Slot::ALL {
            let style = theme.slot(slot);
            let Some(foreground) = style.fg else {
                continue;
            };
            // The border is a line, not a word: it is allowed to be faint.
            if slot == Slot::ChromeBorder {
                continue;
            }
            // The words are read on whatever is behind them.
            let behind = style.bg.unwrap_or(background);
            let ratio = contrast(foreground, behind);
            assert!(
                ratio >= 4.5,
                "{} reads at {ratio:.1}:1, and a word needs 4.5:1",
                slot.name()
            );
        }
    }

    #[test]
    fn the_information_colour_is_neutral() {
        // Information is not a lane: it takes no hue of its own, so it
        // does not compete with the human lane for attention.
        //
        // The test is the one the doc states: the spread between the
        // three channels, within a tolerance. A neutral need not have
        // three equal channels.
        let theme = Theme::preset(Preset::Dark);
        let Color::Rgb(r, g, b) = theme.semantic.info.fg.expect("info has a colour") else {
            panic!("info is not a colour of the dark palette");
        };
        let spread = r.max(g).max(b) - r.min(g).min(b);
        assert!(
            spread <= NEUTRAL_TOLERANCE,
            "info is not neutral: its channels are {spread} apart, and the tolerance is {NEUTRAL_TOLERANCE}"
        );
    }

    #[test]
    fn the_tolerance_of_the_neutral_is_the_one_the_doc_states() {
        // The doc states the tolerance; the code holds the same one.
        assert!(
            PALETTES.contains(&format!("at most {NEUTRAL_TOLERANCE} of 255")),
            "the doc does not state the tolerance of {NEUTRAL_TOLERANCE} the code holds"
        );
    }

    /// The spread between the three channels of a colour.
    fn spread(color: Color) -> u8 {
        let Color::Rgb(r, g, b) = color else {
            return 255;
        };
        r.max(g).max(b) - r.min(g).min(b)
    }

    #[test]
    fn the_neutrals_of_the_families_are_neutral() {
        // Each family table names the neutral it derives; every one of
        // them must pass the test the code holds.
        let mut checked = 0;
        for line in PALETTES.lines() {
            if !line.contains("**the neutral**") {
                continue;
            }
            for hex in hexes_in(line) {
                assert!(
                    spread(hex) <= NEUTRAL_TOLERANCE,
                    "{hex:?} is named a neutral, and it is {0} apart",
                    spread(hex)
                );
                checked += 1;
            }
        }
        assert!(checked >= 3, "no family names a neutral: {checked} found");
    }

    /// The colours a line of the doc names, in the order it names them.
    fn hexes_in(line: &str) -> Vec<Color> {
        let mut colours = Vec::new();
        let mut rest = line;
        while let Some(start) = rest.find('#') {
            rest = &rest[start + 1..];
            if rest.len() < 6 {
                break;
            }
            let Some(end) = rest.find(|c: char| !c.is_ascii_hexdigit()) else {
                break;
            };
            if end != 6 {
                rest = &rest[end..];
                continue;
            }
            let digits = &rest[..6];
            let channel =
                |index: usize| u8::from_str_radix(&digits[index..index + 2], 16).expect("hex");
            colours.push(Color::Rgb(channel(0), channel(2), channel(4)));
            rest = &rest[6..];
        }
        colours
    }

    /// The rows of the dark palette table of the doc: the slot, the
    /// foreground, the background and whether it is bold.
    fn dark_table() -> Vec<(String, Option<Color>, Option<Color>, bool)> {
        let section = PALETTES
            .split_once("## Ours: the dark palette")
            .expect("the doc has a dark palette section")
            .1;
        let section = section
            .split_once("\n## ")
            .map_or(section, |(head, _)| head);
        let mut rows = Vec::new();
        for line in section.lines() {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            // | slot | role | foreground | background | attributes |
            if cells.len() < 6 {
                continue;
            }
            let slot = cells[1].trim_matches('`');
            if !slot.contains('.') {
                continue;
            }
            let foreground = hexes_in(cells[3]).first().copied();
            let background = hexes_in(cells[4]).first().copied();
            let bold = cells[5] == "bold";
            rows.push((slot.to_string(), foreground, background, bold));
        }
        rows
    }

    #[test]
    fn the_dark_palette_is_the_one_the_doc_records() {
        // The doc is the source of truth: the code is checked against
        // its table, so the two cannot drift apart silently.
        let theme = Theme::preset(Preset::Dark);
        let rows = dark_table();
        assert_eq!(
            rows.len(),
            Slot::ALL.len(),
            "the doc's dark table does not hold the eighteen slots"
        );
        for (slot, foreground, background, bold) in rows {
            let style = theme.slot(
                *Slot::ALL
                    .iter()
                    .find(|known| known.name() == slot)
                    .unwrap_or_else(|| panic!("{slot} is not a slot")),
            );
            assert_eq!(
                style.fg, foreground,
                "{slot} paints a foreground the doc does not record"
            );
            assert_eq!(
                style.bg, background,
                "{slot} paints a background the doc does not record"
            );
            assert_eq!(
                style.add_modifier.contains(Modifier::BOLD),
                bold,
                "{slot} is bold in one place and not in the other"
            );
        }
    }

    /// The families, and the heading that names each of them.
    const FAMILIES: [&str; 3] = ["Catppuccin", "Dracula", "Nord"];

    /// The text of a section of the doc.
    fn section(doc: &str, heading: &str) -> String {
        let rest = doc
            .split_once(&format!("## {heading}"))
            .unwrap_or_else(|| panic!("the doc has no {heading} section"))
            .1;
        rest.split_once("\n## ")
            .map_or(rest, |(head, _)| head)
            .to_string()
    }

    /// The rows of a family's tables that name a slot: the slot and the
    /// cells after it — the role first, then the values or the ratios.
    fn family_rows(section: &str) -> Vec<(String, Vec<String>)> {
        let mut rows = Vec::new();
        for line in section.lines() {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            if cells.len() < 4 || !cells[1].starts_with('`') {
                continue;
            }
            let slot = cells[1].trim_matches('`');
            if !slot.contains('.') {
                continue;
            }
            rows.push((
                slot.to_string(),
                cells[2..].iter().map(|cell| cell.to_string()).collect(),
            ));
        }
        rows
    }

    /// The values of a family's slots: the slot and the colour of each
    /// flavour.
    fn family_values(section: &str) -> Vec<(String, Vec<Color>)> {
        family_rows(section)
            .into_iter()
            .filter_map(|(slot, cells)| {
                let values: Vec<Color> = cells[1..]
                    .iter()
                    .filter_map(|cell| hexes_in(cell).first().copied())
                    .collect();
                (!values.is_empty()).then_some((slot, values))
            })
            .collect()
    }

    /// The contrast figures of a family's slots: the slot and the ratio of
    /// each flavour.
    fn family_ratios(section: &str) -> Vec<(String, Vec<f64>)> {
        family_rows(section)
            .into_iter()
            .filter_map(|(slot, cells)| {
                let ratios: Vec<f64> = cells.iter().filter_map(|cell| ratio_in(cell)).collect();
                (!ratios.is_empty()).then_some((slot, ratios))
            })
            .collect()
    }

    /// The ratio a cell states, if it states one.
    fn ratio_in(cell: &str) -> Option<f64> {
        let (number, _) = cell.split_once(":1")?;
        number.trim().parse().ok()
    }

    /// The base of a family: the background its figures are measured
    /// against, one for each flavour.
    fn base_of(section: &str) -> Vec<Color> {
        let line = section
            .lines()
            .find(|line| line.starts_with("The base is"))
            .unwrap_or_else(|| panic!("the section does not name a base"));
        let base = hexes_in(line);
        assert!(!base.is_empty(), "the base names no colour");
        base
    }

    /// The slots that hold a word. The chrome that draws lines is not one.
    fn is_a_word(slot: &str) -> bool {
        !matches!(slot, "chrome.border" | "chrome.tab_index")
    }

    #[test]
    fn every_family_table_names_every_slot() {
        // The mapping table promises every theme the same eighteen slots,
        // and a family that leaves one of them out has no value for it.
        for heading in FAMILIES {
            let section = section(PALETTES, heading);
            let named: Vec<String> = family_values(&section)
                .into_iter()
                .map(|(slot, _)| slot)
                .collect();
            for slot in Slot::ALL {
                assert!(
                    named.contains(&slot.name().to_string()),
                    "{heading} has no {} row",
                    slot.name()
                );
            }
            assert_eq!(
                named.len(),
                Slot::ALL.len(),
                "{heading} names {} rows, and the mapping promises {}",
                named.len(),
                Slot::ALL.len()
            );
        }
    }

    #[test]
    fn the_words_of_the_families_read_on_their_base() {
        // The rule the doc states: a dimmed word reads at 3:1 or better on
        // the background it sits on.
        assert!(
            PALETTES.contains("at 3:1 or better"),
            "the doc does not state the floor the code holds"
        );
        for heading in FAMILIES {
            let section = section(PALETTES, heading);
            let base = base_of(&section);
            for (slot, values) in family_values(&section) {
                if !is_a_word(&slot) {
                    continue;
                }
                assert_eq!(
                    values.len(),
                    base.len(),
                    "{heading} paints {slot} in {} flavours and has {} bases",
                    values.len(),
                    base.len()
                );
                for (colour, base) in values.iter().zip(base.iter()) {
                    let ratio = contrast(*colour, *base);
                    assert!(
                        ratio >= 3.0,
                        "{heading}'s {slot} reads at {ratio:.1}:1, and a word needs 3:1"
                    );
                }
            }
        }
    }

    #[test]
    fn the_contrast_figures_of_the_families_are_right() {
        // Every figure the doc states is recomputed from the values the
        // same table states.
        for heading in FAMILIES {
            let section = section(PALETTES, heading);
            let base = base_of(&section);
            let values = family_values(&section);
            for (slot, ratios) in family_ratios(&section) {
                let colours = values
                    .iter()
                    .find(|(name, _)| name == &slot)
                    .map(|(_, colours)| colours)
                    .unwrap_or_else(|| panic!("{heading} states a ratio for {slot} and no value"));
                assert_eq!(
                    ratios.len(),
                    colours.len(),
                    "{heading} states {} ratios for {slot} and {} values",
                    ratios.len(),
                    colours.len()
                );
                for (ratio, (colour, base)) in ratios.iter().zip(colours.iter().zip(base.iter())) {
                    let computed = contrast(*colour, *base);
                    assert!(
                        (computed - ratio).abs() < 0.05,
                        "{heading}'s {slot} reads at {computed:.2}:1, and the doc says {ratio}:1"
                    );
                }
            }
        }
    }

    #[test]
    fn the_contrast_figures_of_the_dark_palette_are_right() {
        // The same, for our own palette.
        let section = section(PALETTES, "Ours: the dark palette");
        let mut checked = 0;
        for line in section.lines() {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            for pair in cells.windows(2) {
                let Some(colour) = hexes_in(pair[0]).first().copied() else {
                    continue;
                };
                let Some(ratio) = ratio_in(pair[1]) else {
                    continue;
                };
                let computed = contrast(colour, dark::BACKGROUND);
                assert!(
                    (computed - ratio).abs() < 0.05,
                    "{} reads at {computed:.2}:1, and the doc says {ratio}:1",
                    pair[0]
                );
                checked += 1;
            }
        }
        assert!(checked >= 14, "the dark palette states {checked} figures");
    }

    #[test]
    fn the_nord_table_keeps_its_words_readable() {
        // The fourth review: Nord's border, its muted text and its second
        // foreground were merged into one border grade, and the muted text
        // read at 1.7:1 on the family's own base. The muted text is the
        // dimmest snow storm grey, and the second foreground is the frost.
        let section = section(PALETTES, "Nord");
        let base = base_of(&section);
        let values = family_values(&section);
        let value = |slot: &str| {
            values
                .iter()
                .find(|(name, _)| name == slot)
                .map(|(_, colours)| colours[0])
                .unwrap_or_else(|| panic!("the Nord table has no {slot} row"))
        };
        let border = value("chrome.border");
        for slot in ["text.dim", "chrome.tab_inactive"] {
            let ratio = contrast(value(slot), base[0]);
            assert!(
                ratio >= 4.5,
                "Nord's {slot} reads at {ratio:.1}:1 on its base, and a word needs 4.5:1"
            );
            assert_ne!(
                value(slot),
                border,
                "Nord's {slot} is the border's colour, and a word is not a line"
            );
        }
    }

    #[test]
    fn the_example_theme_file_matches_the_nord_table() {
        // The example in the doc is a Nord theme: every value in it is the
        // value the Nord table states.
        let section = section(PALETTES, "Nord");
        let values = family_values(&section);
        let block = PALETTES
            .split_once("```toml")
            .and_then(|(_, rest)| rest.split_once("```"))
            .map(|(block, _)| block)
            .expect("the doc has a theme file example");
        let mut checked = 0;
        for line in block.lines() {
            let Some((key, value)) = line.split_once(" = ") else {
                continue;
            };
            if !key.contains('.') {
                continue;
            }
            let Some(colour) = hexes_in(value).first().copied() else {
                continue;
            };
            let expected = values
                .iter()
                .find(|(slot, _)| slot == key)
                .map(|(_, colours)| colours[0])
                .unwrap_or_else(|| panic!("the Nord table has no {key} row"));
            assert_eq!(
                colour, expected,
                "the example's {key} is not the value the Nord table states"
            );
            checked += 1;
        }
        assert_eq!(
            checked,
            Slot::ALL.len(),
            "the example names {checked} slots"
        );
    }
    #[test]
    fn the_presets_differ_in_paint() {
        // The presets are not the same palette under another name.
        assert_ne!(Theme::preset(Preset::Dark), Theme::preset(Preset::Terminal));
    }

    #[test]
    fn the_terminal_preset_has_no_colour_of_its_own() {
        let theme = Theme::preset(Preset::Terminal);
        for slot in Slot::ALL {
            let style = theme.slot(slot);
            assert!(
                style.fg.is_none_or(|color| color == Color::Reset),
                "{} names a colour",
                slot.name()
            );
            assert!(
                style.bg.is_none_or(|color| color == Color::Reset),
                "{} names a colour",
                slot.name()
            );
        }
    }

    #[test]
    fn a_preset_is_parsed_by_its_name() {
        for preset in Preset::ALL {
            assert_eq!(Preset::parse(preset.name()), Some(preset));
        }
        assert_eq!(Preset::parse("light"), None);
        assert_eq!(Preset::parse(""), None);
    }
}
