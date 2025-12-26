use eframe::egui;
use crate::AppState;
use crate::ui;

pub fn show_error_window(ctx: &egui::Context, app: &mut AppState) {
    if let Some(error_msg) = app.error.clone() {
        egui::Window::new("Error")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui::apply_style(ui);
                ui.label(&error_msg);
                if ui.button("Close").clicked() {
                    app.error = None;
                }
            });
    }
}