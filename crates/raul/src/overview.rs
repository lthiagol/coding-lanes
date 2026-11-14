//! The Overview tab: what is the state of the project.
//!
//! Not a list — a summary the human reads in one screen.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};

use coding_lanes_core::Status;

use crate::data::{Reason, Work};

const LABEL: Style = Style::new().fg(Color::Gray);
const DIM: Style = Style::new().fg(Color::DarkGray);
const VALUE: Style = Style::new().fg(Color::White).add_modifier(Modifier::BOLD);
const HEADING: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
const ALERT: Style = Style::new().fg(Color::Red).add_modifier(Modifier::BOLD);

/// Below this width the blocks are stacked in one column.
const NARROW: u16 = 58;
/// The rows a block of the Overview takes for its border.
const BORDER: u16 = 2;
/// The columns the label of a count takes, so the counts line up.
const LABEL_WIDTH: usize = 12;

/// Draw the tab.
pub fn render(frame: &mut Frame, area: Rect, work: &Work) {
    if work.missing {
        render_missing(frame, area, work);
        return;
    }
    if work.is_empty() {
        render_empty(frame, area, work);
        return;
    }
    if area.width < NARROW {
        render_stacked(frame, area, work);
    } else {
        render_wide(frame, area, work);
    }
}

/// There is no work folder.
fn render_missing(frame: &mut Frame, area: Rect, work: &Work) {
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled("no coding-lanes folder found", ALERT)),
            Line::from(Span::styled(
                format!("looked in {}", work.root.display()),
                DIM,
            )),
        ])
        .wrap(Wrap { trim: true })
        .block(Block::bordered().title("overview")),
        area,
    );
}

/// There is nothing to read — but a file that does not parse still shows.
fn render_empty(frame: &mut Frame, area: Rect, work: &Work) {
    let empty = Paragraph::new(Line::from(Span::styled("no tasks yet", DIM)))
        .block(Block::bordered().title("overview"));
    if work.errors.is_empty() {
        frame.render_widget(
            empty,
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 3.min(area.height),
            },
        );
        return;
    }
    let panel = errors_panel(work);
    let width = area.width.saturating_sub(BORDER);
    let needed = panel.rows(width) + BORDER;
    // The empty state in its box, and the files that could not be read.
    if area.height >= 3 + needed {
        frame.render_widget(
            empty,
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 3,
            },
        );
        frame.render_widget(
            errors_block(&panel),
            Rect {
                x: area.x,
                y: area.y + 3,
                width: area.width,
                height: needed,
            },
        );
        return;
    }
    // Not enough room for both: the empty state gives up its box, and the
    // file that could not be read is named.
    let empty_rows = u16::from(area.height >= 2);
    if empty_rows == 1 {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled("no tasks yet", DIM))),
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 1,
            },
        );
    }
    let y = area.y + empty_rows;
    let height = area.height - empty_rows;
    if height >= needed {
        frame.render_widget(
            errors_block(&panel),
            Rect {
                x: area.x,
                y,
                width: area.width,
                height: needed,
            },
        );
    } else if height >= 1 {
        // The block does not fit: the file is named on one row.
        frame.render_widget(
            Paragraph::new(error_marker(work, area.width)),
            Rect {
                x: area.x,
                y,
                width: area.width,
                height: 1,
            },
        );
    }
}

/// The files that could not be read, in a box.
fn errors_block(panel: &Panel) -> Paragraph<'static> {
    Paragraph::new(panel.lines.clone())
        .wrap(Wrap { trim: true })
        .block(Block::bordered().title(panel.title))
}

/// The line that names a file that could not be read, when the block does
/// not fit.
fn error_marker(work: &Work, width: u16) -> Line<'static> {
    let first = &work.errors[0];
    let mut text = format!("{}: {}", first.file, first.error);
    if work.errors.len() > 1 {
        text.push_str(&format!(" (+{} more)", work.errors.len() - 1));
    }
    Line::from(Span::styled(cut_to(&text, usize::from(width)), DIM))
}

/// Two columns: the counts on the left, the numbers that ask for a decision
/// on the right.
fn render_wide(frame: &mut Frame, area: Rect, work: &Work) {
    let columns =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    // The unreadable block outranks the kinds: a file that could not be read
    // matters more than how many smaller tasks there are.
    let mut left = vec![counts_panel(work, columns[0].width)];
    if !work.errors.is_empty() {
        left.push(errors_panel(work));
    }
    left.push(kinds_panel(work, columns[0].width));
    let right = vec![
        needs_you_panel(work, columns[1].width),
        claimable_panel(work),
        next_panel(work),
    ];
    let fits = column_fits(
        &left,
        columns[0].width.saturating_sub(BORDER),
        columns[0].height,
    ) && column_fits(
        &right,
        columns[1].width.saturating_sub(BORDER),
        columns[1].height,
    );
    let area = summary_area(frame, area, work, fits);
    let columns =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    draw_panels(frame, columns[0], &left);
    draw_panels(frame, columns[1], &right);
}

/// One column, for a narrow terminal.
fn render_stacked(frame: &mut Frame, area: Rect, work: &Work) {
    let mut panels = vec![
        needs_you_panel(work, area.width),
        claimable_panel(work),
        next_panel(work),
    ];
    panels.push(counts_panel(work, area.width));
    // The unreadable block outranks the kinds: a file that could not be read
    // matters more than how many smaller tasks there are.
    if !work.errors.is_empty() {
        panels.push(errors_panel(work));
    }
    panels.push(kinds_panel(work, area.width));
    let width = area.width.saturating_sub(BORDER);
    let fits = column_fits(&panels, width, area.height);
    let area = summary_area(frame, area, work, fits);
    draw_panels(frame, area, &panels);
}

/// The area the blocks get, and the summary line above them.
///
/// The key numbers stay visible, whatever the height and the width: when a
/// block does not fit, the numbers go on top of the blocks.
fn summary_area(frame: &mut Frame, area: Rect, work: &Work, fits: bool) -> Rect {
    if fits || area.height == 0 {
        return area;
    }
    let Some(summary) = summary_line(work, area.width) else {
        return area;
    };
    frame.render_widget(
        Paragraph::new(summary),
        Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        },
    );
    Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height - 1,
    }
}

/// Whether every block of the column is drawn.
fn column_fits(panels: &[Panel], width: u16, height: u16) -> bool {
    layout_of(panels, width, height)
        .iter()
        .all(|rows| *rows > 0)
}

/// The key numbers on one line, for a terminal too short for the blocks.
///
/// The longest form that fits the row is the one shown, so a part of the
/// line is never cut in half. When not even the shortest form fits, there
/// is no summary line: a cut number is worse than none. Every form keeps
/// the qualifier of its number, so a number on this line is never read as
/// the number of tasks in the project.
fn summary_line(work: &Work, width: u16) -> Option<Line<'static>> {
    let (approve, review, escalated) = work.needs_you_counts();
    let total = approve + review + escalated;
    let claimable = work.claimable().len();
    let human = format!(
        "{total} {} {} on a human",
        task_word(total),
        wait_word(total)
    );
    // The short forms keep the qualifier: a bare number would be read as
    // the number of tasks in the project.
    let brief = format!("{total} {} a human", need_word(total));
    let shortest = format!("{total} {}", wait_word(total));
    let claim = format!("{claimable} claimable");
    let short_claim = format!("{claimable} claim");
    let separator = || Span::styled(" · ", DIM);
    let mut forms: Vec<Vec<Span<'static>>> = Vec::new();
    if let Some(task) = work.next() {
        forms.push(vec![
            Span::styled(human.clone(), VALUE),
            separator(),
            Span::styled(claim.clone(), VALUE),
            separator(),
            Span::styled(format!("next {}", task.id), VALUE),
        ]);
    }
    forms.push(vec![
        Span::styled(human.clone(), VALUE),
        separator(),
        Span::styled(claim.clone(), VALUE),
    ]);
    forms.push(vec![
        Span::styled(brief.clone(), VALUE),
        separator(),
        Span::styled(claim.clone(), VALUE),
    ]);
    forms.push(vec![
        Span::styled(brief, VALUE),
        separator(),
        Span::styled(short_claim.clone(), VALUE),
    ]);
    forms.push(vec![
        Span::styled(shortest.clone(), VALUE),
        separator(),
        Span::styled(claim, VALUE),
    ]);
    forms.push(vec![
        Span::styled(shortest, VALUE),
        separator(),
        Span::styled(short_claim, VALUE),
    ]);
    forms
        .into_iter()
        .find(|form| width_of(form) <= usize::from(width))
        .map(Line::from)
}

/// The columns a line takes.
fn width_of(spans: &[Span<'_>]) -> usize {
    spans.iter().map(|span| span.content.chars().count()).sum()
}

/// The text, cut to `width` columns.
fn cut_to(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    if width == 0 {
        return String::new();
    }
    let head: String = text.chars().take(width - 1).collect();
    format!("{head}…")
}

/// A block of the Overview: a titled box with lines in it.
struct Panel {
    title: &'static str,
    /// The lines it shows when there is room.
    lines: Vec<Line<'static>>,
    /// The lines it shows when there is less room: complete, but denser.
    compact: Option<Vec<Line<'static>>>,
    /// Whether the dense form is only used when each of its lines fits on
    /// one row. A dense form that wraps in the middle of a value puts the
    /// value out of the reader's reach.
    compact_whole: bool,
    /// The lines at the end that may be dropped when there is no room.
    optional: usize,
    /// Whether the block is left out when it does not fit whole.
    atomic: bool,
    /// What the hidden lines are called.
    more: &'static str,
    /// What the marker says when the block is left out.
    marker: Option<String>,
}

impl Panel {
    /// The lines to draw in a box of `content` rows and `width` columns.
    ///
    /// A block that cannot show everything says how much is hidden, so a
    /// count is never shown as if it were the whole count.
    fn shown(&self, width: u16, content: u16) -> Vec<Line<'static>> {
        if fits(&self.lines, width, content) {
            return self.lines.clone();
        }
        let essential = self.lines.len().saturating_sub(self.optional);
        if essential > 0 && fits(&self.lines[..essential], width, content) {
            return self.lines[..essential].to_vec();
        }
        if let Some(compact) = self.dense(width)
            && fits(compact, width, content)
        {
            return compact.to_vec();
        }
        if self.atomic {
            return Vec::new();
        }
        if content < 2 {
            return Vec::new();
        }
        cut(&self.lines, width, content, self.more)
    }

    /// The dense form, when it is worth using.
    ///
    /// A dense form that wraps in the middle of a value hides the value from
    /// the reader, so a block that asks for one is drawn in its full form
    /// instead.
    fn dense(&self, width: u16) -> Option<&[Line<'static>]> {
        let compact = self.compact.as_ref()?;
        if self.compact_whole
            && compact
                .iter()
                .any(|line| rows_of(std::slice::from_ref(line), width) > 1)
        {
            return None;
        }
        Some(compact)
    }

    /// The rows the block wants.
    fn rows(&self, width: u16) -> u16 {
        rows_of(&self.lines, width)
    }

    /// The rows the block needs when it gives up its last lines.
    fn essential_rows(&self, width: u16) -> u16 {
        let essential = self.lines.len().saturating_sub(self.optional);
        rows_of(&self.lines[..essential], width)
    }

    /// The rows the dense form takes, when it is worth using.
    fn dense_rows(&self, width: u16) -> Option<u16> {
        let compact = self.dense(width)?;
        Some(rows_of(compact, width))
    }
}

/// Whether the lines fit in the box.
fn fits(lines: &[Line<'static>], width: u16, content: u16) -> bool {
    rows_of(lines, width) <= content
}

/// The lines that fit, and a line saying how many do not.
fn cut(lines: &[Line<'static>], width: u16, content: u16, more: &str) -> Vec<Line<'static>> {
    let mut shown: Vec<Line<'static>> = Vec::new();
    let mut used = 0u16;
    for line in lines {
        let rows = rows_of(std::slice::from_ref(line), width);
        if used + rows + 1 > content {
            break;
        }
        used += rows;
        shown.push(line.clone());
    }
    let hidden = lines.len() - shown.len();
    if hidden > 0 {
        shown.push(Line::from(Span::styled(
            format!("+{hidden} more {more}"),
            DIM,
        )));
    }
    shown
}

/// The rows each block of a column gets.
///
/// Every block gets the rows it needs. The blocks that come last give up
/// their rows first — they are the ones the reader can do without. A block
/// gives up its last lines, then takes its dense form, and is left out only
/// when there is nothing left to give. A block that is left out says so,
/// so nothing disappears without a word.
fn layout_of(panels: &[Panel], width: u16, height: u16) -> Vec<u16> {
    let mut content: Vec<u16> = panels.iter().map(|panel| panel.rows(width)).collect();
    for index in (0..panels.len()).rev() {
        while total(&content) + dropped_markers(&content) > height {
            if !reduce(&panels[index], &mut content[index], width) {
                break;
            }
        }
        if total(&content) + dropped_markers(&content) <= height {
            break;
        }
    }
    content
}

/// Give a block fewer rows: first the lines it can do without, then its
/// dense form, and last the block itself.
fn reduce(panel: &Panel, content: &mut u16, width: u16) -> bool {
    let essential = panel.essential_rows(width);
    if *content > essential {
        *content = essential;
        return true;
    }
    if let Some(dense) = panel.dense_rows(width)
        && *content > dense
    {
        *content = dense;
        return true;
    }
    if *content > 0 {
        *content = 0;
        return true;
    }
    false
}

/// Draw the blocks of a column, top to bottom.
fn draw_panels(frame: &mut Frame, area: Rect, panels: &[Panel]) {
    let width = area.width.saturating_sub(BORDER);
    let content = layout_of(panels, width, area.height);
    let mut y = area.y;
    for (index, panel) in panels.iter().enumerate() {
        if content[index] == 0 {
            // The block was left out: say so, where it was.
            if y < area.y + area.height {
                frame.render_widget(
                    Paragraph::new(marker(panel, area.width)),
                    Rect {
                        x: area.x,
                        y,
                        width: area.width,
                        height: 1,
                    },
                );
            }
            y += 1;
            continue;
        }
        let lines = panel.shown(width, content[index]);
        if lines.is_empty() {
            // The block has nothing that fits: say so rather than show
            // nothing at all.
            if y < area.y + area.height {
                frame.render_widget(
                    Paragraph::new(marker(panel, area.width)),
                    Rect {
                        x: area.x,
                        y,
                        width: area.width,
                        height: 1,
                    },
                );
            }
            y += 1;
            continue;
        }
        let rows = rows_of(&lines, width);
        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: true })
                .block(Block::bordered().title(panel.title)),
            Rect {
                x: area.x,
                y,
                width: area.width,
                height: rows + BORDER,
            },
        );
        y += rows + BORDER;
    }
}

/// The rows the markers of the left out blocks take.
fn dropped_markers(content: &[u16]) -> u16 {
    content.iter().filter(|rows| **rows == 0).count() as u16
}

/// The line that says a block was left out.
fn marker(panel: &Panel, width: u16) -> Line<'static> {
    let text = panel
        .marker
        .clone()
        .unwrap_or_else(|| format!("{} not shown", panel.title));
    Line::from(Span::styled(cut_to(&text, usize::from(width)), DIM))
}

/// What the marker of the unreadable block says: the files it holds and
/// what is wrong with them.
fn hidden_files(work: &Work) -> String {
    let files: Vec<String> = work
        .errors
        .iter()
        .map(|error| format!("{}: {}", error.file, error.error))
        .collect();
    format!("unreadable not shown: {}", files.join(", "))
}

/// The rows the blocks take, borders included.
fn total(content: &[u16]) -> u16 {
    content
        .iter()
        .filter(|rows| **rows > 0)
        .map(|rows| rows + BORDER)
        .sum()
}

/// Tasks per status.
fn counts_panel(work: &Work, width: u16) -> Panel {
    let counts = work.status_counts();
    let total: usize = counts.iter().map(|(_, count)| count).sum();
    let heading = counts_heading(total);
    let inner = width.saturating_sub(BORDER);
    let label_width = counts
        .iter()
        .map(|(status, _)| status.as_str().chars().count())
        .max()
        .unwrap_or(0);
    let digits = count_width(counts.iter().map(|(_, count)| *count));
    // The count stands apart from its label: the two never touch.
    let cell = label_width + 1 + digits;
    let mut lines = vec![heading];
    // The dense form: the statuses in pairs, each count with its status.
    let dense = dense_counts(&counts, total, inner);
    // Two statuses per row, when the column is wide enough for it.
    if usize::from(inner) > 2 * cell {
        for pair in counts.chunks(2) {
            let mut spans = Vec::new();
            for (position, (status, count)) in pair.iter().enumerate() {
                if position > 0 {
                    spans.push(Span::raw(" "));
                }
                spans.push(Span::styled(
                    format!("{:<label_width$}", status.as_str()),
                    LABEL,
                ));
                spans.push(Span::styled(format!(" {count:>digits$}"), VALUE));
            }
            lines.push(Line::from(spans));
        }
    } else if label_width + 1 + digits <= usize::from(inner) {
        // One status per row: the label is padded to the width of the
        // longest status, and the row fits the column, so a count stays on
        // the row of its label.
        for (status, count) in &counts {
            lines.push(status_line(status.as_str(), *count, label_width, digits));
        }
    } else {
        // The padded label does not fit the column. The dense form keeps
        // every count with its status; when it does not fit either, the
        // block is left out whole.
        lines = dense.clone().unwrap_or_default();
    }
    Panel {
        title: "counts",
        lines,
        compact: dense,
        // The counts are read together: the dense form is used only when
        // every count stays with its status.
        compact_whole: true,
        optional: 0,
        atomic: false,
        more: "rows",
        marker: None,
    }
}

/// The heading of the counts block.
fn counts_heading(total: usize) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{total} "), VALUE),
        Span::styled(format!("{} by status", task_word(total)), HEADING),
    ])
}

/// The dense form of the counts: a status and its count on the same row.
///
/// The rows are built to fit the column, so a count is never parted from
/// its status. A status that does not fit on a row of its own leaves the
/// dense form out.
fn dense_counts(
    counts: &[(Status, usize)],
    total: usize,
    inner: u16,
) -> Option<Vec<Line<'static>>> {
    let inner = usize::from(inner);
    let mut rows: Vec<Vec<Span<'static>>> = Vec::new();
    let mut row: Vec<Span<'static>> = Vec::new();
    let mut used = 0usize;
    for (status, count) in counts {
        let label = status.as_str();
        let width = label.chars().count() + 1 + count.to_string().chars().count();
        if width > inner {
            return None;
        }
        if !row.is_empty() {
            if used + 1 + width > inner {
                rows.push(std::mem::take(&mut row));
                used = 0;
            } else {
                row.push(Span::raw(" "));
                used += 1;
            }
        }
        row.push(Span::styled(label.to_string(), LABEL));
        row.push(Span::raw(" "));
        row.push(Span::styled(count.to_string(), VALUE));
        used += width;
    }
    if !row.is_empty() {
        rows.push(row);
    }
    let mut lines = vec![counts_heading(total)];
    lines.extend(rows.into_iter().map(Line::from));
    Some(lines)
}

/// One status and its count.
///
/// The row is exactly as wide as the label column plus the count, so the
/// count stays on the row of its label.
fn status_line(status: &str, count: usize, label_width: usize, digits: usize) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{status:<label_width$}"), LABEL),
        Span::styled(format!(" {count:>digits$}"), VALUE),
    ])
}

/// Smaller tasks per kind.
///
/// The label is padded to the width of the column, so a count stays on the
/// row of its label. A kind whose label and count do not fit on one row
/// leaves the block out whole.
fn kinds_panel(work: &Work, width: u16) -> Panel {
    let kinds = work.kind_counts();
    let inner = usize::from(width.saturating_sub(BORDER));
    let digits = count_width(kinds.iter().map(|(_, count)| *count));
    let label_width = kinds
        .iter()
        .map(|(kind, _)| kind.as_str().chars().count())
        .max()
        .unwrap_or(0);
    // The label is padded as far as the column allows.
    let pad = LABEL_WIDTH.min(inner.saturating_sub(digits));
    let mut lines = vec![Line::from(Span::styled("smaller tasks by kind", HEADING))];
    if label_width <= pad {
        for (kind, count) in &kinds {
            lines.push(Line::from(vec![
                Span::styled(format!("{:<pad$}", kind.as_str()), LABEL),
                Span::styled(count.to_string(), VALUE),
            ]));
        }
    } else {
        // A kind and its count do not fit on one row: the block is left out
        // whole rather than showing a count on a row of its own.
        lines.clear();
    }
    Panel {
        title: "smaller tasks",
        lines,
        compact: None,
        compact_whole: false,
        optional: 0,
        atomic: false,
        more: "kinds",
        marker: None,
    }
}

/// What waits on a human.
fn needs_you_panel(work: &Work, width: u16) -> Panel {
    let (approve, review, escalated) = work.needs_you_counts();
    let total = approve + review + escalated;
    let headline = Line::from(vec![
        Span::styled(format!("{total} "), VALUE),
        Span::styled(
            format!("{} {} on a human", task_word(total), wait_word(total)),
            LABEL,
        ),
    ]);
    let inner = width.saturating_sub(BORDER);
    let counts = [approve, review, escalated];
    // The counts sit in a column when the column holds one. In a narrow
    // terminal they sit right after their reason.
    let column = usize::from(inner) >= 2 + LABEL_WIDTH + count_width(counts);
    let mut lines = vec![headline.clone()];
    let mut hidden = 0;
    for (reason, count) in [
        (Reason::Approve, approve),
        (Reason::Review, review),
        (Reason::Escalated, escalated),
    ] {
        let line = reason_line(reason, count, column);
        // The reason and its count share a row: when they do not fit, the
        // reason is left out rather than parted from its count.
        if rows_of(std::slice::from_ref(&line), inner) == 1 {
            lines.push(line);
        } else {
            hidden += 1;
        }
    }
    if hidden > 0 {
        lines.push(Line::from(Span::styled(
            format!("+{hidden} more reasons"),
            DIM,
        )));
    }
    // The dense form: the three reasons on one row.
    let mut spans = Vec::new();
    for (index, (reason, count)) in [
        (Reason::Approve, approve),
        (Reason::Review, review),
        (Reason::Escalated, escalated),
    ]
    .into_iter()
    .enumerate()
    {
        if index > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(format!("{} {}", reason.label(), count), LABEL));
    }
    Panel {
        title: "needs you",
        lines,
        compact: Some(vec![headline, Line::from(spans)]),
        // The three reasons are read together: the dense form is used only
        // when it keeps every count with its reason.
        compact_whole: true,
        optional: 0,
        atomic: true,
        more: "reasons",
        marker: None,
    }
}

/// One reason and its count.
fn reason_line(reason: Reason, count: usize, column: bool) -> Line<'static> {
    let label = if column {
        format!("  {:<LABEL_WIDTH$}", reason.label())
    } else {
        format!("  {} ", reason.label())
    };
    Line::from(vec![
        Span::styled(label, LABEL),
        Span::styled(count.to_string(), VALUE),
    ])
}

/// The columns the widest of the counts takes.
fn count_width(counts: impl IntoIterator<Item = usize>) -> usize {
    counts
        .into_iter()
        .map(|count| count.to_string().chars().count())
        .max()
        .unwrap_or(1)
}

/// How many tasks an agent could claim right now.
fn claimable_panel(work: &Work) -> Panel {
    let count = work.claimable().len();
    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{count} "), VALUE),
            Span::styled(
                format!("{} an agent could claim right now", task_word(count)),
                LABEL,
            ),
        ]),
        Line::from(Span::styled("  approved, free, dependencies done", DIM)),
    ];
    Panel {
        title: "claimable",
        lines,
        compact: None,
        compact_whole: false,
        optional: 1,
        atomic: false,
        more: "lines",
        marker: None,
    }
}

/// What happens next: the first claimable task.
fn next_panel(work: &Work) -> Panel {
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
    Panel {
        title: "next",
        lines,
        compact: None,
        compact_whole: false,
        optional: 1,
        atomic: false,
        more: "lines",
        marker: None,
    }
}

/// The files that could not be read.
fn errors_panel(work: &Work) -> Panel {
    let lines = work
        .errors
        .iter()
        .map(|error| {
            Line::from(vec![
                Span::styled(format!("{}: ", error.file), VALUE),
                Span::styled(error.error.clone(), DIM),
            ])
        })
        .collect();
    Panel {
        title: "unreadable",
        lines,
        compact: None,
        compact_whole: false,
        optional: 0,
        atomic: false,
        more: "files",
        marker: Some(hidden_files(work)),
    }
}

/// The rows the lines take when they are wrapped into `width` columns.
///
/// The height is ratatui's own: the paragraph that will be drawn is the one
/// that is measured, so a padded line is counted the way it is drawn.
fn rows_of(lines: &[Line<'static>], width: u16) -> u16 {
    if width == 0 {
        return 0;
    }
    Paragraph::new(lines.to_vec())
        .wrap(Wrap { trim: true })
        .line_count(width) as u16
}

/// The word for a number of tasks.
fn task_word(count: usize) -> &'static str {
    if count == 1 { "task" } else { "tasks" }
}

/// The verb for a number of tasks.
fn wait_word(count: usize) -> &'static str {
    if count == 1 { "waits" } else { "wait" }
}

/// The verb for a number of tasks that need a human.
fn need_word(count: usize) -> &'static str {
    if count == 1 { "needs" } else { "need" }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::*;

    fn mock_work() -> Work {
        Work::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../mock-data/coding-lanes/work"))
    }

    /// Draw the Overview and return the rows it shows.
    fn draw(work: &Work, width: u16, height: u16) -> Vec<String> {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test terminal");
        terminal
            .draw(|frame| render(frame, frame.area(), work))
            .expect("drawing the overview");
        let buffer = terminal.backend().buffer();
        let area = *buffer.area();
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buffer.cell((x, y)).map(|cell| cell.symbol()).unwrap_or(" "))
                    .collect::<String>()
            })
            .collect()
    }

    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("raul-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("a scratch folder");
        dir
    }

    #[test]
    fn the_overview_shows_the_numbers() {
        let work = mock_work();
        let text = draw(&work, 100, 40).join("\n");
        assert!(text.contains("8 tasks wait on a human"), "{text}");
        assert!(
            text.contains("1 task an agent could claim right now"),
            "{text}"
        );
        assert!(text.contains("approve"), "{text}");
        assert!(text.contains("escalated"), "{text}");
    }

    #[test]
    fn the_overview_shows_the_counts_and_the_next_task() {
        let work = mock_work();
        let text = draw(&work, 100, 40).join("\n");
        assert!(text.contains("29 tasks by status"), "{text}");
        assert!(text.contains("draft"), "{text}");
        assert!(text.contains("needs-human"), "{text}");
        assert!(text.contains("idea"), "{text}");
        assert!(text.contains("TSK29"), "{text}");
    }

    #[test]
    fn a_short_terminal_does_not_cut_a_count() {
        let work = mock_work();
        let text = draw(&work, 60, 16).join("\n");
        // What is shown is shown whole, and what is hidden is announced.
        assert!(text.contains("draft"), "{text}");
        assert!(text.contains("8 tasks wait on a human"), "{text}");
        assert!(
            text.contains("more statuses") || text.contains("needs-human"),
            "{text}"
        );
    }

    #[test]
    fn a_narrow_terminal_shows_the_key_numbers() {
        let work = mock_work();
        let text = draw(&work, 40, 12).join("\n");
        assert!(text.contains("8 tasks wait on a human"), "{text}");
        assert!(text.contains("1 claimable"), "{text}");
    }

    #[test]
    fn an_unreadable_file_shows_its_error() {
        let dir = scratch_dir("unreadable");
        fs::write(dir.join("TSK98-broken.json"), "{ not json").expect("writing a broken file");
        let work = Work::load(&dir);
        assert!(work.is_empty());
        let text = draw(&work, 80, 20).join("\n");
        assert!(text.contains("no tasks yet"), "{text}");
        assert!(text.contains("unreadable"), "{text}");
        assert!(text.contains("TSK98-broken.json"), "{text}");
    }

    #[test]
    fn a_long_text_is_wrapped_and_not_cut() {
        let dir = scratch_dir("wrapped");
        let mut task = coding_lanes_core::Task::new("TSK01", "a long title");
        task.title = "a long title that has to be wrapped over more than one line".to_string();
        task.status = coding_lanes_core::Status::Approved;
        fs::write(
            dir.join("TSK01-a-long-title.json"),
            serde_json::to_string_pretty(&task).expect("writing a task"),
        )
        .expect("writing a task");
        let work = Work::load(&dir);
        let text = draw(&work, 40, 20).join("\n");
        assert!(text.contains("more than one line"), "{text}");
    }

    #[test]
    fn a_word_longer_than_the_block_takes_the_rows_it_needs() {
        let dir = scratch_dir("long-word");
        let mut task = coding_lanes_core::Task::new("TSK01", "a long title");
        task.title = "X".repeat(90);
        task.status = coding_lanes_core::Status::Approved;
        fs::write(
            dir.join("TSK01-a-long-title.json"),
            serde_json::to_string_pretty(&task).expect("writing a task"),
        )
        .expect("writing a task");
        let work = Work::load(&dir);
        let text = draw(&work, 40, 20).join("\n");
        // The word is broken over rows; every letter of it is shown.
        assert_eq!(text.chars().filter(|ch| *ch == 'X').count(), 90, "{text}");
    }

    #[test]
    fn an_unreadable_file_outranks_the_smaller_tasks() {
        let dir = scratch_dir("unreadable-rank");
        let mut task = coding_lanes_core::Task::new("TSK01", "a good task");
        task.status = coding_lanes_core::Status::Approved;
        fs::write(
            dir.join("TSK01-a-good-task.json"),
            serde_json::to_string_pretty(&task).expect("writing a task"),
        )
        .expect("writing a task");
        fs::write(dir.join("TSK98-broken.json"), "{ not json").expect("writing a broken file");
        let work = Work::load(&dir);
        let text = draw(&work, 100, 12).join("\n");
        // The file that could not be read is shown; the block that is left
        // out says so.
        assert!(text.contains("unreadable"), "{text}");
        assert!(text.contains("TSK98-broken.json"), "{text}");
        assert!(text.contains("not shown"), "{text}");
    }

    #[test]
    fn a_narrow_summary_is_never_cut() {
        let work = mock_work();
        let text = draw(&work, 38, 12).join("\n");
        assert!(text.contains("1 claimable"), "{text}");
        // The next task is either shown whole or not at all.
        assert!(!text.contains("next TS"), "{text}");
    }

    #[test]
    fn a_count_stays_with_its_reason() {
        let work = mock_work();
        for (width, height) in [(64, 12), (66, 12), (100, 12), (100, 8)] {
            let rows = draw(&work, width, height);
            let row = rows
                .iter()
                .find(|row| row.contains("escalated"))
                .unwrap_or_else(|| panic!("no escalated row at {width}x{height}: {rows:?}"));
            // The count of the escalated tasks is on the row of its reason.
            let text = row.replace('│', " ");
            assert!(text.trim().ends_with('3'), "{width}x{height}: {row}");
        }
    }

    #[test]
    fn a_short_wide_terminal_shows_the_key_numbers() {
        let work = mock_work();
        let text = draw(&work, 100, 8).join("\n");
        // The blocks do not fit, so the numbers go on top of them.
        assert!(text.contains("8 tasks wait on a human"), "{text}");
        assert!(text.contains("1 claimable"), "{text}");
    }

    #[test]
    fn a_dropped_block_leaves_a_whole_marker() {
        let work = mock_work();
        let text = draw(&work, 20, 6).join("\n");
        // The marker is cut to the width of the column, not to the width of
        // the block inside it.
        assert!(text.contains("needs you not shown"), "{text}");
    }

    #[test]
    fn the_dense_counts_keep_a_count_with_its_status() {
        let work = mock_work();
        let counts = work.status_counts();
        let dense = dense_counts(&counts, 29, 38).expect("a dense form");
        for line in &dense {
            let text = text_of(std::slice::from_ref(line));
            assert!(text.chars().count() <= 38, "{text}");
        }
        for (status, count) in &counts {
            // The status and its count are on the same row.
            let found = dense.iter().any(|line| {
                text_of(std::slice::from_ref(line))
                    .contains(&format!("{} {count}", status.as_str()))
            });
            assert!(found, "{status:?} is not with its count");
        }
    }

    #[test]
    fn a_block_is_dropped_before_another_one_is_made_dense() {
        let work = mock_work();
        let rows = draw(&work, 80, 12);
        // The counts keep their full form: the labels are padded, so the
        // counts stand in a column.
        let column = |label: &str| -> usize {
            let row = rows
                .iter()
                .find(|row| row.contains(label))
                .unwrap_or_else(|| panic!("no row with {label}: {rows:?}"));
            let start = row.find(label).expect("the label");
            let rest = &row[start + label.len()..];
            start + label.len() + rest.find(|ch: char| ch.is_ascii_digit()).expect("a count")
        };
        assert_eq!(column("draft"), column("code-review"));
        // The block after the counts is the one that is left out.
        assert!(
            rows.iter()
                .any(|row| row.contains("smaller tasks not shown")),
            "{rows:?}"
        );
    }

    #[test]
    fn a_three_digit_count_stands_apart_from_its_label() {
        let dir = scratch_dir("many-tasks");
        for index in 1..=120 {
            let mut task = coding_lanes_core::Task::new(format!("TSK{index:03}"), "a task");
            task.status = coding_lanes_core::Status::CodeReview;
            fs::write(
                dir.join(format!("TSK{index:03}-a-task.json")),
                serde_json::to_string_pretty(&task).expect("writing a task"),
            )
            .expect("writing a task");
        }
        let work = Work::load(&dir);
        let text = draw(&work, 100, 40).join("\n");
        assert!(text.contains("code-review 120"), "{text}");
    }

    #[test]
    fn a_short_summary_keeps_the_qualifier_of_its_number() {
        let work = mock_work();
        for width in 8..=60 {
            let Some(line) = summary_line(&work, width) else {
                continue;
            };
            let text = text_of(std::slice::from_ref(&line));
            // The number is the count of the tasks that wait on a human:
            // it says so, so it is never read as the count of the tasks.
            assert!(
                text.contains("wait") || text.contains("human"),
                "{width}: {text}"
            );
        }
    }

    #[test]
    fn a_narrow_terminal_does_not_part_a_reason_from_its_count() {
        let work = mock_work();
        for width in 1..=40 {
            let panel = needs_you_panel(&work, width);
            for line in &panel.lines {
                let text = text_of(std::slice::from_ref(line));
                for (reason, count) in [("approve", "2"), ("review", "3"), ("escalated", "3")] {
                    if text.contains(reason) {
                        assert!(text.contains(count), "{width}: {text}");
                    }
                }
            }
        }
        // In a terminal too narrow for a reason and its count, the reason
        // is left out rather than parted from its count.
        let text = draw(&work, 9, 21).join("\n");
        assert!(!text.contains("approve"), "{text}");
        assert!(text.contains("+3 more"), "{text}");
    }

    #[test]
    fn an_unreadable_marker_carries_the_error() {
        let dir = scratch_dir("unreadable-marker");
        let mut task = coding_lanes_core::Task::new("TSK01", "a good task");
        task.status = coding_lanes_core::Status::Approved;
        fs::write(
            dir.join("TSK01-a-good-task.json"),
            serde_json::to_string_pretty(&task).expect("writing a task"),
        )
        .expect("writing a task");
        fs::write(dir.join("TSK98-broken.json"), "{ not json").expect("writing a broken file");
        let work = Work::load(&dir);
        let text = draw(&work, 100, 12).join("\n");
        let marker = text
            .lines()
            .find(|line| line.contains("unreadable not shown"))
            .unwrap_or_else(|| panic!("no marker: {text}"));
        // The name of the file and the error, not only the name.
        assert!(marker.contains("TSK98-broken.json"), "{marker}");
        assert!(marker.contains("key must"), "{marker}");
    }

    /// The text of some lines.
    fn text_of(lines: &[Line<'static>]) -> String {
        lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.to_string())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn an_unreadable_file_is_named_when_the_block_does_not_fit() {
        let dir = scratch_dir("unreadable-short");
        fs::write(dir.join("TSK98-broken.json"), "{ not json").expect("writing a broken file");
        let work = Work::load(&dir);
        let text = draw(&work, 80, 2).join("\n");
        // The block does not fit, but the file is named all the same.
        assert!(text.contains("TSK98-broken.json"), "{text}");
    }

    #[test]
    fn a_count_stays_with_its_status_at_every_width() {
        let work = mock_work();
        let counts = work.status_counts();
        for width in 1..=60 {
            let panel = counts_panel(&work, width);
            if panel.lines.is_empty() {
                // The block is left out whole: the marker says so.
                continue;
            }
            for (status, count) in &counts {
                let label = status.as_str();
                let row = panel
                    .lines
                    .iter()
                    .find(|line| text_of(std::slice::from_ref(line)).contains(label))
                    .unwrap_or_else(|| panic!("no row for {label} at {width}"));
                let text = text_of(std::slice::from_ref(row));
                // The count is on the row of its status, and the row is one
                // row: a count is never on a row of its own.
                assert!(text.contains(&count.to_string()), "{width}: {text}");
                assert_eq!(
                    rows_of(std::slice::from_ref(row), width.saturating_sub(BORDER)),
                    1,
                    "{width}: {text}"
                );
            }
        }
    }

    #[test]
    fn a_count_stays_with_its_kind_at_every_width() {
        let work = mock_work();
        for width in 1..=60 {
            let panel = kinds_panel(&work, width);
            if panel.lines.is_empty() {
                // The block is left out whole: the marker says so.
                continue;
            }
            for (kind, count) in work.kind_counts() {
                let label = kind.as_str();
                let row = panel
                    .lines
                    .iter()
                    .find(|line| text_of(std::slice::from_ref(line)).contains(label))
                    .unwrap_or_else(|| panic!("no row for {label} at {width}"));
                let text = text_of(std::slice::from_ref(row));
                assert!(text.contains(&count.to_string()), "{width}: {text}");
                assert_eq!(
                    rows_of(std::slice::from_ref(row), width.saturating_sub(BORDER)),
                    1,
                    "{width}: {text}"
                );
            }
        }
    }

    #[test]
    fn a_block_that_cannot_hold_a_count_is_left_out_whole() {
        let work = mock_work();
        // The counts do not fit a column of fourteen: the block is left
        // out, not shown with a count on a row of its own.
        let text = draw(&work, 14, 60).join("\n");
        assert!(text.contains("counts not sh"), "{text}");
        assert!(!text.contains("code-review"), "{text}");
        // At fifteen columns they fit, and every count is with its status.
        let text = draw(&work, 15, 60).join("\n");
        assert!(text.contains("code-review 3"), "{text}");
        assert!(!text.contains("counts not shown"), "{text}");
    }

    #[test]
    fn the_short_summary_agrees_with_its_count() {
        let dir = scratch_dir("one-human");
        let mut task = coding_lanes_core::Task::new("TSK01", "a task");
        task.status = coding_lanes_core::Status::Groomed;
        fs::write(
            dir.join("TSK01-a-task.json"),
            serde_json::to_string_pretty(&task).expect("writing a task"),
        )
        .expect("writing a task");
        let work = Work::load(&dir);
        let mut seen = false;
        for width in 1..=60 {
            let Some(line) = summary_line(&work, width) else {
                continue;
            };
            let text = text_of(std::slice::from_ref(&line));
            assert!(!text.contains("1 need a human"), "{width}: {text}");
            if text.contains("needs a human") {
                seen = true;
            }
        }
        assert!(seen, "the short form never said '1 needs a human'");
    }
}
