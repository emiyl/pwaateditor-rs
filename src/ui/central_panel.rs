use eframe::egui;
use crate::tabs::Tab;
use crate::ui;
use crate::ui::tabs::{chapters::show_chapters_tab, about::show_about_tab, settings::show_settings_tab};
use crate::AppState;

pub fn show_central_panel(ui: &mut egui::Ui, app: &mut AppState) {
    ui::apply_style(ui);
    
    match app.tab {
        Tab::Chapters => show_chapters_tab(ui, app),
        Tab::Settings => show_settings_tab(ui, app),
        Tab::About => show_about_tab(ui, app),
    }
}