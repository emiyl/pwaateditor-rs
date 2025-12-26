#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Chapters,
    Settings,
    About
}

pub const TABS: [(Tab, &str); 3] = [
    (Tab::Chapters, "Chapters"),
    (Tab::Settings, "Settings"),
    (Tab::About, "About"),
];