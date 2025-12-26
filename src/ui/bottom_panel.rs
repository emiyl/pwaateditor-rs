use eframe::egui;
use crate::{AppState, file, Tab, ui};

pub fn show_bottom_panel(ui: &mut egui::Ui, app: &mut AppState) {
    ui::apply_style(ui);
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::Frame::new()
            .fill(ui.visuals().code_bg_color)
            .stroke(ui.visuals().widgets.inactive.bg_stroke)
            .corner_radius(ui.visuals().widgets.inactive.corner_radius)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    ui.menu_button(egui_material_icons::icons::ICON_KEYBOARD_ARROW_DOWN, |ui| {
                        if ui.button("Save as").clicked() {
                            let previous_path = app.path.clone();
                            if let Some(save_file) = &app.save_file {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    app.path = Some(path.display().to_string());
                                    file::write_save(app.path.as_ref().unwrap(), save_file).expect("Failed to write save file");
                                    app.path = previous_path;
                                }
                            } else {
                                app.error = Some("No save file loaded".to_string());
                            }
                        }
                    });

                    if ui.button("Save").clicked() {
                        if let Some(save_file) = &app.save_file {
                            if let Some(path) = &app.path {
                                file::write_save(path, save_file).expect("Failed to write save file");
                            } else {
                                app.error = Some("No file path specified".to_string());
                            }
                        } else {
                            app.error = Some("No save file loaded".to_string());
                        }
                    }
                });
            });
            
            if ui.button("Load").clicked() {
                if let Some(path) = &app.path {
                    if file::does_file_exist(path) {
                        let save_file = file::open_save(path);
                        app.save_file = Some(save_file);
                        app.tab = Tab::Chapters;
                    } else {
                        app.save_file = None;
                    }
                }
            }

            egui::Frame::new()
            .fill(ui.visuals().extreme_bg_color)
            .stroke(ui.visuals().widgets.inactive.bg_stroke)
            .corner_radius(ui.visuals().widgets.inactive.corner_radius)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = ui.visuals().extreme_bg_color;

                    if ui.button("…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            app.path = Some(path.display().to_string());
                            app.save_file = Some(file::open_save(path.to_str().unwrap()));
                            app.tab = Tab::Chapters;
                        }
                    }

                    let button_height = ui.spacing().interact_size.y;
                    let path = app.path.get_or_insert_with(String::new);
                    let text_edit = egui::TextEdit::singleline(path)
                        .desired_width(f32::INFINITY)
                        .min_size(egui::vec2(0.0, button_height))
                        .frame(false);
                    let output = text_edit.show(ui);

                    if path.is_empty() {
                        let painter = ui.painter_at(output.response.rect);
                        let text_color = egui::Color32::from_rgba_premultiplied(100, 100, 100, 100);
                        let galley = painter.layout(
                            String::from("Path"),
                            egui::FontId::default(),
                            text_color,
                            f32::INFINITY
                        );
                        painter.galley(output.galley_pos, galley, text_color);
                    }
                });
            });
        });
    });
    ui.add_space(0.0);
}