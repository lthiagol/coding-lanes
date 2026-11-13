//! The Overview tab: what is the state of the project.
//!
//! Not a list — a summary the human reads in one screen.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use crate::data::{Reason, Work};

const LABEL: Style = Style::new().fg(Color::Gray);
const DIM: Style = Style::new().fg(Color::DarkGray);
const VALUE: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
const HEADING: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

/// Draw the tab.
pub fn render(frame: &mut Frame, area: Rect, work: &Work) {
    if work.missing {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled(
                    "no coding-lanes folder found",
                    Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    format!("looked in {}", work.root.display()),
                    DIM,
                )),
            ])
            .block(Block::bordered().title("overview")),
            area,
        );
        return;
    }

    if work.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled("no tasks yet", DIM)))
                .block(Block::bordered().title("overview")),
            area,
        );
        return;
    }

    let columns =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let left = Layout::vertical([
        Constraint::Length(11),
        Constraint::Length(6),
        Constraint::Min(3),
    ])
    .split(columns[0]);
    render_statuses(frame, left[0], work);
    render_kinds(frame, left[1], work);
    if !work.errors.is_empty() {
        render_errors(frame, left[2], work);
    }

    let right = Layout::vertical([
        Constraint::Length(6),
        Constraint::Length(4),
        Constraint::Min(3),
    ])
    .split(columns[1]);
    render_needs_you(frame, right[0], work);
    render_claimable(frame, right[1], work);
    render_next(frame, right[2], work);
}

/// The files that could not be read. The rest of the folder still renders.
fn render_errors(frame: &mut Frame, area: Rect, work: &Work) {
    let mut lines = Vec::new();
    for error in &work.errors {
        lines.push(Line::from(vec![
            Span::styled(format!("{}: ", error.file), VALUE),
            Span::styled(error.error.clone(), DIM),
        ]));
    }
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("unreadable")),
        area,
    );
}

/// Tasks per status.
fn render_statuses(frame: &mut Frame, area: Rect, work: &Work) {
    let mut lines = vec![Line::from(Span::styled("tasks by status", HEADING))];
    for (status, count) in work.status_counts() {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:<12}", status.as_str()), LABEL),
            Span::styled(count.to_string(), VALUE),
        ]));
    }
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("counts")),
        area,
    );
}

/// Smaller tasks per kind.
fn render_kinds(frame: &mut Frame, area: Rect, work: &Work) {
    let mut lines = vec![Line::from(Span::styled("smaller tasks by kind", HEADING))];
    for (kind, count) in work.kind_counts() {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:<12}", kind.as_str()), LABEL),
            Span::styled(count.to_string(), VALUE),
        ]));
    }
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("smaller tasks")),
        area,
    );
}

/// What waits on a human.
fn render_needs_you(frame: &mut Frame, area: Rect, work: &Work) {
    let (approve, review, escalated) = work.needs_you_counts();
    let total = approve + review + escalated;
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{total} "), VALUE),
            Span::styled("tasks wait on a human", LABEL),
        ]),
        Line::from(vec![
            Span::styled(format!("  {:<12}", Reason::Approve.label()), LABEL),
            Span::styled(approve.to_string(), VALUE),
        ]),
        Line::from(vec![
            Span::styled(format!("  {:<12}", Reason::Review.label()), LABEL),
            Span::styled(review.to_string(), VALUE),
        ]),
        Line::from(vec![
            Span::styled(format!("  {:<12}", Reason::Escalated.label()), LABEL),
            Span::styled(escalated.to_string(), VALUE),
        ]),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("needs you")),
        area,
    );
}

/// How many tasks an agent could claim right now.
fn render_claimable(frame: &mut Frame, area: Rect, work: &Work) {
    let count = work.claimable().len();
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{count} "), VALUE),
            Span::styled("tasks an agent could claim right now", LABEL),
        ]),
        Line::from(Span::styled("  approved, free, dependencies done", DIM)),
    ];
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("claimable")),
        area,
    );
}

/// What happens next: the first claimable task.
fn render_next(frame: &mut Frame, area: Rect, work: &Work) {
    let lines = match work.next() {
        Some(task) => vec![
            Line::from(vec![
                Span::styled(format!("{} ", task.id), VALUE),
                Span::styled(task.title.clone(), LABEL),
            ]),
            Line::from(Span::styled("  the first task of the path", DIM)),
        ],
        None => vec![
            Line::from(Span::styled("nothing is claimable right now", LABEL)),
            Line::from(Span::styled(
                "  no approved task has its dependencies done",
                DIM,
            )),
        ],
    };
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title("next")),
        area,
    );
}
