use crate::tabs::TABS;
use eframe::egui;

pub fn show_tab_list(ui: &mut egui::Ui, app: &mut crate::AppState) {
    ui.style_mut().spacing.button_padding = eframe::egui::vec2(8.0, 6.0);
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        for (tab, label) in TABS {
            if ui.selectable_label(app.tab == tab, label).clicked() {
                app.tab = tab;
            }
        }
    });
    ui.add_space(5.0);
}