use eframe::egui;
use crate::AppState;

pub fn show_about_tab(ui: &mut egui::Ui, app: &mut AppState) {
    if app.save_file.is_none() {
        ui.heading("PWAATeditor");
    }
    ui.label("PWAATeditor is a simple editor for Phoenix Wright: Ace Attorney Trilogy save files.");
    ui.label("Written by emiyl.");
    if app.save_file.is_none() {
        ui.label("No save file loaded.");
    }
}