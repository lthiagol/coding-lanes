//! The window: the tab bar, the content pane and the footer.

use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::data::Work;
use crate::overview;
use crate::tabs::Tab;

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
}

impl App {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            tab: Tab::Overview,
            help: false,
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
        frame.render_widget(self.tab_bar(), rows[1]);
        match self.tab {
            Tab::Overview => overview::render(frame, rows[2], work),
            tab => frame.render_widget(placeholder(tab), rows[2]),
        }
        frame.render_widget(self.footer(), rows[3]);

        if self.help {
            render_help(frame, area);
        }
    }

    /// The title line: the name of the tool and the folder it reads.
    fn title_line(&self, width: u16) -> Line<'static> {
        let title = Span::styled(
            " coding-lanes ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
        let available = width.saturating_sub(13) as usize;
        let path = self.root.display().to_string();
        let path = if path.chars().count() > available {
            // Keep the tail: the folder name matters more than the root.
            let tail: Vec<char> = path.chars().rev().take(available.saturating_sub(1)).collect();
            let tail: String = tail.into_iter().rev().collect();
            format!("…{tail}")
        } else {
            path
        };
        let padding = " ".repeat(available.saturating_sub(path.chars().count()));
        Line::from(vec![
            title,
            Span::raw(padding),
            Span::styled(path, Style::default().fg(Color::DarkGray)),
        ])
    }

    /// The tab bar, with the active tab marked.
    fn tab_bar(&self) -> Line<'static> {
        let mut spans = Vec::new();
        for (index, tab) in Tab::ALL.iter().enumerate() {
            if index > 0 {
                spans.push(Span::raw(" "));
            }
            let style = if *tab == self.tab {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            spans.push(Span::styled(format!(" {} ", tab.label()), style));
        }
        Line::from(spans)
    }

    /// The footer: the keys that do something on this tab.
    fn footer(&self) -> Line<'static> {
        let key = Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        let label = Style::default().fg(Color::DarkGray);
        let mut spans = vec![
            Span::styled(" q ", key),
            Span::styled(" quit", label),
            Span::raw("   "),
            Span::styled(" 1-6 ", key),
            Span::styled(" tab", label),
            Span::raw("   "),
            Span::styled(" ←→ ", key),
            Span::styled(" switch tab", label),
            Span::raw("   "),
            Span::styled(" ? ", key),
            Span::styled(" help", label),
        ];
        if self.help {
            spans.push(Span::raw("   "));
            spans.push(Span::styled(" esc ", key));
            spans.push(Span::styled(" close help", label));
        }
        Line::from(spans)
    }
}

/// The tabs that are not built yet.
fn placeholder(tab: Tab) -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(
        format!("the {} tab is not built yet", tab.label()),
        Style::default().fg(Color::DarkGray),
    )))
    .block(Block::bordered().title(tab.label()))
}

/// The help overlay.
fn render_help(frame: &mut Frame, area: Rect) {
    let width = area.width.saturating_mul(3) / 5;
    let height = area.height.saturating_mul(3) / 5;
    let popup = Rect {
        x: area.x + (area.width.saturating_sub(width)) / 2,
        y: area.y + (area.height.saturating_sub(height)) / 2,
        width,
        height,
    };
    let key = Style::default()
        .fg(Color::Black)
        .bg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let label = Style::default().fg(Color::Gray);
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
        Line::from(vec![
            Span::styled(" q ", key),
            Span::styled(" quit", label),
        ]),
        Line::from(vec![
            Span::styled(" ? ", key),
            Span::styled(" this help", label),
        ]),
    ];
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("keys")),
        popup,
    );
}

#[cfg(test)]
mod dump {
    use super::*;
    use crate::data::Work;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use std::path::Path;

    #[test]
    fn dump() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../mock-data/coding-lanes/work");
        let work = Work::load(&root);
        let app = App::new(root);
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        let frame = terminal.draw(|f| app.draw(f, &work)).unwrap();
        let buffer = &frame.buffer;
        let area = buffer.area();
        for y in 0..area.height {
            let mut line = String::new();
            for x in 0..area.width {
                line.push_str(buffer.get(x, y).symbol());
            }
            println!("{y:>2}|{line}|");
        }
    }
}
