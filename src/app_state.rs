use crate::save;
use crate::tabs::Tab;

pub struct AppState {
    pub path: Option<String>,
    pub save_file: Option<save::SaveFile>,
    pub error: Option<String>,
    pub tab: Tab,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            path: None,
            save_file: None,
            error: None,
            tab: Tab::About,
        }
    }
}