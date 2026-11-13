use anyhow::Result;
use coding_lanes_core::Store;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

fn main() -> Result<()> {
    let store = Store::load(&Store::default_path()).unwrap_or_default();

    let mut terminal = setup()?;
    let result = run(&mut terminal, &store);
    restore(&mut terminal)?;
    result
}

fn run(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    store: &Store,
) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, store))?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, store: &Store) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let title = Paragraph::new(Line::from(vec![Span::styled(
        " coding-lanes ",
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(Block::default().borders(Borders::ALL).title("coding-lanes"));
    frame.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = if store.tasks.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "no tasks yet",
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        store
            .tasks
            .iter()
            .map(|task| {
                let style = if task.done {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{} ", if task.done { "[x]" } else { "[ ]" }), style),
                    Span::styled(format!("#{:<4} ", task.id), style),
                    Span::styled(task.title.clone(), style),
                ]))
            })
            .collect()
    };

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("tasks"));
    frame.render_widget(list, chunks[1]);

    let help = Paragraph::new(Line::from(vec![
        Span::styled(" q ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" quit    "),
        Span::styled(" esc ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" quit"),
    ]));
    frame.render_widget(help, chunks[2]);
}

fn setup() -> Result<ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(ratatui::Terminal::new(
        ratatui::backend::CrosstermBackend::new(stdout),
    )?)
}

fn restore(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
