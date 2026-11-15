//! The window: the tab bar, the content pane and the footer.

use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use crate::data::Work;
use crate::overview;
use crate::tabs::Tab;
use crate::theme::{Theme, block_title_style};

/// What a key asks the window to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Redraw,
}

/// The state of the window.
pub struct App {
    /// The work folder raul reads.
    pub root: PathBuf,
    /// The active tab.
    pub tab: Tab,
    /// The help overlay is open.
    pub help: bool,
    /// The colours of the window.
    pub theme: Theme,
}

impl App {
    /// A window with a theme of its own.
    pub fn new(root: PathBuf, theme: Theme) -> Self {
        Self {
            root,
            tab: Tab::Overview,
            help: false,
            theme,
        }
    }

    /// Handle a key. Returns what the window does next.
    pub fn on_key(&mut self, key: KeyEvent) -> Action {
        if key.kind != KeyEventKind::Press {
            return Action::Redraw;
        }
        if self.help {
            self.help = false;
            return match key.code {
                KeyCode::Char('q') => Action::Quit,
                _ => Action::Redraw,
            };
        }
        match key.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
            KeyCode::Esc => Action::Quit,
            KeyCode::Char('?') => {
                self.help = true;
                Action::Redraw
            }
            KeyCode::Char(digit @ '1'..='6') => {
                self.tab = Tab::from_index(digit as usize - '1' as usize);
                Action::Redraw
            }
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                self.tab = self.tab.next();
                Action::Redraw
            }
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                self.tab = self.tab.previous();
                Action::Redraw
            }
            _ => Action::Redraw,
        }
    }

    /// Draw the window.
    pub fn draw(&self, frame: &mut Frame, work: &Work) {
        let area = frame.area();
        let rows = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(area);

        frame.render_widget(self.title_line(rows[0].width), rows[0]);
        frame.render_widget(self.tab_bar(rows[1].width), rows[1]);
        match self.tab {
            Tab::Overview => overview::render(frame, rows[2], work, &self.theme),
            tab => frame.render_widget(placeholder(tab, &self.theme), rows[2]),
        }
        frame.render_widget(self.footer(rows[3].width), rows[3]);

        if self.help {
            render_help(frame, area, &self.theme);
        }
    }

    /// The title line: the name of the tool and the folder it reads.
    ///
    /// A terminal too narrow for the name gives the whole line to the
    /// folder: a part of the line is never cut in half.
    fn title_line(&self, width: u16) -> Line<'static> {
        const NAME: &str = " coding-lanes ";
        let name = Span::styled(NAME, self.theme.chrome.title);
        let folder = self.theme.text.dim;
        let name_width = NAME.chars().count() as u16;
        if width < name_width {
            // The name does not fit: the folder it reads takes its place.
            let path = self.path_tail(usize::from(width));
            let padding = " ".repeat(usize::from(width).saturating_sub(path.chars().count()));
            return Line::from(vec![Span::raw(padding), Span::styled(path, folder)]);
        }
        let columns = usize::from(width - name_width);
        let path = self.path_tail(columns);
        let padding = " ".repeat(columns.saturating_sub(path.chars().count()));
        Line::from(vec![name, Span::raw(padding), Span::styled(path, folder)])
    }

    /// The path, in the columns it has: the tail, which names the folder,
    /// is what is kept.
    fn path_tail(&self, columns: usize) -> String {
        let path = self.root.display().to_string();
        let length = path.chars().count();
        if length <= columns {
            return path;
        }
        if columns == 0 {
            return String::new();
        }
        let tail: String = path.chars().skip(length - (columns - 1)).collect();
        format!("…{tail}")
    }

    /// The tab bar, with the active tab marked.
    ///
    /// A terminal too narrow for the six tabs gets the active tab alone.
    /// The parts of the line are dropped whole: no label is ever cut in
    /// half.
    fn tab_bar(&self, width: u16) -> Line<'static> {
        let full: usize = Tab::ALL
            .iter()
            .map(|tab| tab.label().chars().count() + 2)
            .sum::<usize>()
            + Tab::ALL.len()
            - 1;
        if full <= width as usize {
            let mut spans = Vec::new();
            for (index, tab) in Tab::ALL.iter().enumerate() {
                if index > 0 {
                    spans.push(Span::raw(" "));
                }
                spans.push(Span::styled(
                    format!(" {} ", tab.label()),
                    self.tab_style(*tab),
                ));
            }
            return Line::from(spans);
        }
        // Too narrow for the six tabs: the active tab, marked. The parts
        // are dropped whole, from the least to the most useful.
        let marked = format!(" ‹ {} › ", self.tab.label());
        let plain = format!(" {} ", self.tab.label());
        let index = format!("{}/{}", self.tab.index() + 1, Tab::ALL.len());
        let parts = [
            (marked, self.tab_style(self.tab)),
            (plain, self.tab_style(self.tab)),
            (index, self.theme.chrome.tab_index),
        ];
        for (part, style) in parts {
            if part.chars().count() <= width as usize {
                return Line::from(vec![Span::styled(part, style)]);
            }
        }
        Line::from(Span::raw(""))
    }

    /// The style of a tab in the bar.
    fn tab_style(&self, tab: Tab) -> Style {
        if tab == self.tab {
            self.theme.chrome.tab_active
        } else {
            self.theme.chrome.tab_inactive
        }
    }

    /// The footer: the keys that do something on this tab.
    ///
    /// A terminal too narrow for every key keeps the keys that matter most,
    /// and no key is ever cut in half.
    fn footer(&self, width: u16) -> Line<'static> {
        let key = self.theme.chrome.key;
        let label = self.theme.chrome.footer;
        let space = || Span::raw("   ");
        // (where it sits in the line, its spans, whether it matters most)
        let mut parts: Vec<(usize, Vec<Span<'static>>, bool)> = vec![
            (
                0,
                vec![Span::styled(" q ", key), Span::styled(" quit", label)],
                true,
            ),
            (
                1,
                vec![
                    space(),
                    Span::styled(" 1-6 ", key),
                    Span::styled(" tab", label),
                ],
                true,
            ),
            (
                2,
                vec![
                    space(),
                    Span::styled(" ←→ ", key),
                    Span::styled(" switch tab", label),
                ],
                false,
            ),
            (
                3,
                vec![
                    space(),
                    Span::styled(" ? ", key),
                    Span::styled(" help", label),
                ],
                true,
            ),
        ];
        if self.help {
            parts.push((
                4,
                vec![
                    space(),
                    Span::styled(" esc ", key),
                    Span::styled(" close help", label),
                ],
                true,
            ));
        }
        // The keys that matter most are kept; the rest only if they fit.
        let mut chosen = vec![false; parts.len()];
        let mut used = 0usize;
        for matters in [true, false] {
            for (index, (_, spans, needed)) in parts.iter().enumerate() {
                if *needed != matters {
                    continue;
                }
                let columns = spans
                    .iter()
                    .map(|span| span.content.chars().count())
                    .sum::<usize>();
                if used + columns <= usize::from(width) {
                    chosen[index] = true;
                    used += columns;
                }
            }
        }
        let mut order: Vec<usize> = (0..parts.len()).collect();
        order.sort_by_key(|index| parts[*index].0);
        let mut spans = Vec::new();
        for index in order {
            if chosen[index] {
                spans.extend(parts[index].1.clone());
            }
        }
        Line::from(spans)
    }
}

/// The tabs that are not built yet.
fn placeholder(tab: Tab, theme: &Theme) -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(
        format!("the {} tab is not built yet", tab.label()),
        theme.text.dim,
    )))
    .wrap(Wrap { trim: true })
    .block(
        Block::bordered()
            .title(tab.label())
            .title_style(block_title_style(theme))
            .border_style(theme.chrome.border),
    )
}

/// The help overlay.
fn render_help(frame: &mut Frame, area: Rect, theme: &Theme) {
    let width = area.width.saturating_mul(3) / 5;
    let height = area.height.saturating_mul(3) / 5;
    let popup = Rect {
        x: area.x + (area.width.saturating_sub(width)) / 2,
        y: area.y + (area.height.saturating_sub(height)) / 2,
        width,
        height,
    };
    let key = theme.chrome.key;
    let label = theme.text.normal;
    let lines = vec![
        Line::from(vec![
            Span::styled(" 1-6 ", key),
            Span::styled(" jump to a tab", label),
        ]),
        Line::from(vec![
            Span::styled(" ← → ", key),
            Span::styled(" switch tab", label),
        ]),
        Line::from(vec![
            Span::styled(" h l ", key),
            Span::styled(" switch tab", label),
        ]),
        Line::from(vec![
            Span::styled(" tab ", key),
            Span::styled(" next tab", label),
        ]),
        Line::from(vec![
            Span::styled(" shift-tab ", key),
            Span::styled(" previous tab", label),
        ]),
        Line::from(vec![Span::styled(" q ", key), Span::styled(" quit", label)]),
        Line::from(vec![
            Span::styled(" ? ", key),
            Span::styled(" this help", label),
        ]),
    ];
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: true }).block(
            Block::bordered()
                .title("keys")
                .title_style(block_title_style(theme))
                .border_style(theme.chrome.border_focus),
        ),
        popup,
    );
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    // The colour type is named here to read the paint back: no colour is
    // written outside the theme.
    use ratatui::style::{Color, Modifier};

    use super::*;
    use crate::data::Work;
    use crate::theme::Preset;

    fn mock_work() -> Work {
        Work::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../mock-data/coding-lanes/work"))
    }

    /// A window on the mock data, with the default theme.
    fn app() -> App {
        App::new(
            PathBuf::from("/tmp/coding-lanes/work"),
            Theme::preset(Preset::Dark),
        )
    }

    /// The window as it is drawn: a theme, a tab, and the help overlay.
    fn window(theme: &Theme, tab: Tab, help: bool) -> App {
        let mut app = App::new(PathBuf::from("/tmp/coding-lanes/work"), *theme);
        app.tab = tab;
        app.help = help;
        app
    }

    /// The paint of one cell: its colours and its attributes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct Paint {
        fg: Color,
        bg: Color,
        modifier: Modifier,
    }

    /// Draw the window and return what it shows: the rows, and the paint
    /// of every cell.
    fn render(app: &App, width: u16, height: u16) -> (Vec<String>, Vec<Vec<Paint>>) {
        let work = mock_work();
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test terminal");
        terminal
            .draw(|frame| app.draw(frame, &work))
            .expect("drawing the window");
        let buffer = terminal.backend().buffer();
        let area = *buffer.area();
        let rows = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buffer.cell((x, y)).map(|cell| cell.symbol()).unwrap_or(" "))
                    .collect::<String>()
            })
            .collect();
        let paint = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| {
                        buffer
                            .cell((x, y))
                            .map(|cell| Paint {
                                fg: cell.fg,
                                bg: cell.bg,
                                modifier: cell.modifier,
                            })
                            .unwrap_or_default()
                    })
                    .collect()
            })
            .collect();
        (rows, paint)
    }

    /// Draw the window with a theme and return the rows it shows.
    fn draw_with(theme: &Theme, width: u16, height: u16) -> Vec<String> {
        let app = window(theme, Tab::Overview, false);
        render(&app, width, height).0
    }

    /// Draw the window and return the rows it shows.
    fn draw(width: u16, height: u16) -> Vec<String> {
        draw_with(&Theme::preset(Preset::Dark), width, height)
    }
    #[test]
    fn a_narrow_title_drops_whole_parts() {
        let app = app();
        for width in 0..=24 {
            let title = app.title_line(width);
            let text = title
                .spans
                .iter()
                .map(|span| span.content.to_string())
                .collect::<String>();
            assert!(text.chars().count() <= width as usize, "{width}: {text}");
            // The name is whole, or the folder is.
            if text.contains("coding-lane") {
                assert!(text.contains("coding-lanes"), "{width}: {text}");
            }
        }
    }

    #[test]
    fn a_narrow_tab_bar_drops_whole_parts() {
        let app = app();
        for width in 0..=24 {
            let bar = app.tab_bar(width);
            let text = bar
                .spans
                .iter()
                .map(|span| span.content.to_string())
                .collect::<String>();
            assert!(text.chars().count() <= width as usize, "{width}: {text}");
            // The active tab alone, the index alone, or nothing: no part
            // of the line is cut in half.
            assert!(
                [" ‹ Overview › ", " Overview ", "1/6", ""].contains(&text.as_str()),
                "{width}: {text}"
            );
        }
    }

    #[test]
    fn a_narrow_window_does_not_cut_a_label() {
        let rows = draw(40, 12);
        let text = rows.join("\n");
        assert!(text.contains("Overview"), "{text}");
        assert!(!text.contains("Settings"), "{text}");
        assert!(text.contains("quit"), "{text}");
        assert!(text.contains("help"), "{text}");
    }

    #[test]
    fn a_narrow_footer_does_not_cut_a_key() {
        let rows = draw(30, 8);
        let footer = rows.last().expect("a footer row").trim_end().to_string();
        // Every key shown is whole: nothing is cut in the middle of a word.
        assert!(footer.contains("quit"), "{footer}");
        assert!(!footer.contains("hel"), "{footer}");
        assert!(footer.chars().count() <= 30, "{footer}");
    }

    #[test]
    fn a_wide_window_shows_every_tab() {
        let rows = draw(100, 20);
        let text = rows.join("\n");
        for tab in Tab::ALL {
            assert!(text.contains(tab.label()), "{tab:?} is missing");
        }
        assert!(text.contains("switch tab"), "{text}");
    }

    /// Every screen the window can show.
    fn screens() -> Vec<(Tab, bool)> {
        let mut screens: Vec<(Tab, bool)> = Tab::ALL.iter().map(|tab| (*tab, false)).collect();
        // The help overlay is a screen of its own.
        screens.push((Tab::Overview, true));
        screens
    }

    #[test]
    fn the_presets_say_the_same_thing() {
        for (tab, help) in screens() {
            for (width, height) in [(100, 20), (60, 16), (40, 12)] {
                let dark = render(
                    &window(&Theme::preset(Preset::Dark), tab, help),
                    width,
                    height,
                )
                .0;
                let terminal = render(
                    &window(&Theme::preset(Preset::Terminal), tab, help),
                    width,
                    height,
                )
                .0;
                // A theme changes the paint, never the words.
                assert_eq!(dark, terminal, "{tab:?} help={help} {width}x{height}");
            }
        }
    }

    #[test]
    fn the_presets_differ_in_paint() {
        for (tab, help) in screens() {
            let dark = render(&window(&Theme::preset(Preset::Dark), tab, help), 100, 20).1;
            let terminal = render(
                &window(&Theme::preset(Preset::Terminal), tab, help),
                100,
                20,
            )
            .1;
            assert_ne!(
                dark, terminal,
                "{tab:?} help={help}: the presets paint the same"
            );
        }
    }

    #[test]
    fn the_terminal_preset_paints_with_attributes() {
        // In the terminal preset the colours are the terminal's: what
        // tells the presets apart is the attributes.
        let terminal = render(
            &window(&Theme::preset(Preset::Terminal), Tab::Overview, false),
            100,
            20,
        )
        .1;
        let attributes = terminal
            .iter()
            .flatten()
            .any(|paint| paint.modifier != Modifier::empty());
        assert!(attributes, "the terminal preset paints no attributes");
    }
}
