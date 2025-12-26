use eframe::egui::{self, Ui};

const SPACING: f32 = 8.0;

pub fn apply_style(ui: &mut Ui) {
    ui.style_mut().spacing.button_padding = eframe::egui::vec2(SPACING, 6.0);
    ui.spacing_mut().item_spacing.y = SPACING;
}

// pub fn get_line_height(ui: &&mut Ui) -> f32 {
//     let button_height = ui.spacing().interact_size.y;
//     button_height + SPACING
// }

pub fn apply_checkbox_style(ui: &mut Ui) {
    let fg_color = ui.visuals().text_color();
    let style = ui.style_mut();
    style.spacing.button_padding = egui::vec2(0.0, 0.0);
    style.visuals.button_frame = false;
    style.text_styles.get_mut(&egui::TextStyle::Button).unwrap().size = 24.0;
    style.visuals.widgets.inactive.fg_stroke.color = fg_color;
}