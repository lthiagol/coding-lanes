//! The six tabs of the window.

/// A tab of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    NeedsYou,
    Path,
    Backlog,
    Autopilot,
    Settings,
}

impl Tab {
    pub const ALL: [Tab; 6] = [
        Tab::Overview,
        Tab::NeedsYou,
        Tab::Path,
        Tab::Backlog,
        Tab::Autopilot,
        Tab::Settings,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Tab::Overview => "Overview",
            Tab::NeedsYou => "Needs you",
            Tab::Path => "Path",
            Tab::Backlog => "Backlog",
            Tab::Autopilot => "Autopilot",
            Tab::Settings => "Settings",
        }
    }

    pub fn index(&self) -> usize {
        Tab::ALL
            .iter()
            .position(|tab| tab == self)
            .unwrap_or_default()
    }

    pub fn from_index(index: usize) -> Tab {
        Tab::ALL[index % Tab::ALL.len()]
    }

    pub fn next(&self) -> Tab {
        Tab::from_index(self.index() + 1)
    }

    pub fn previous(&self) -> Tab {
        Tab::from_index(self.index() + Tab::ALL.len() - 1)
    }
}
