use eframe::egui;
use crate::AppState;
use crate::save;
use crate::ui::style;

fn format_volume_input(input: &str) -> Option<f64> {
    match input.trim() {
        "0%" => Some(1.0),
        "25%" => Some(2.0),
        "50%" => Some(3.0),
        "75%" => Some(4.0),
        "100%" => Some(5.0),
        other => other.parse::<f64>().ok().and_then(|n| {
            let rounded = (n / 25.0).round() * 25.0;
            if rounded >= 0.0 && rounded <= 100.0 {
                Some((rounded / 25.0) + 1.0)
            } else {
                None
            }
        }),
    }
}

fn parse_volume_output(value: f64) -> String {
    match value {
        1.0 => "0%",
        2.0 => "25%",
        3.0 => "50%",
        4.0 => "75%",
        5.0 => "100%",
        _ => "Invalid",
    }.to_string()
}

fn text_skip_to_str(text_skip: save::TextSkip) -> &'static str {
    match text_skip {
        save::TextSkip::Off => "Off",
        save::TextSkip::SingleBoxSkip => "Single Box",
        save::TextSkip::FullAutoSkip => "Full Auto",
    }
}

fn text_box_transparency_to_str(transparency: save::TextBoxTransparency) -> &'static str {
    match transparency {
        save::TextBoxTransparency::Off => "Off",
        save::TextBoxTransparency::Low => "Low",
        save::TextBoxTransparency::High => "High",
    }
}

fn language_to_str(language: save::Language) -> &'static str {
    match language {
        save::Language::Japanese => "Japanese",
        save::Language::English => "English",
        save::Language::French => "French",
        save::Language::German => "German",
        save::Language::Korean => "Korean",
        save::Language::ChineseSimplified => "Chinese (Simplified)",
        save::Language::ChineseTraditional => "Chinese (Traditional)",
    }
}

pub fn show_settings_tab(ui: &mut egui::Ui, app: &mut AppState) {
    let save_file = match &mut app.save_file {
        Some(save_file) => save_file,
        None => {
            ui.label("No save file loaded.");
            return;
        }
    };

    let mut bg_music_input = save_file.read_bg_music_volume() as u8;
    let mut se_music_input = save_file.read_se_volume() as u8;
    let text_skip_input = save_file.read_text_skip();
    let screen_shake_input = save_file.read_screen_shake();
    let vibration_input = save_file.read_vibration();
    let text_box_transparency_input = save_file.read_text_box_transparency();
    let language_input = save_file.read_language();
    let fullscreen_input = save_file.read_fullscreen();
    let vsync_input = save_file.read_vsync();

    egui::ScrollArea::vertical()
    .auto_shrink([false; 2])
    .show(ui, |ui| {
        egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([16.0, 8.0])
        .show(ui, |ui| {
            ui.label("BG Volume");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(
                    egui::DragValue::new(&mut bg_music_input)
                    .range(1..=5)
                    .custom_formatter(|n, _| { parse_volume_output(n) })
                    .custom_parser(|s| { format_volume_input(s) })
                ).changed() {
                    save_file.write_bg_music_volume(bg_music_input);
                }
            });
            ui.end_row();

            ui.label("SE Volume");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(
                    egui::DragValue::new(&mut se_music_input)
                    .range(1..=5)
                    .custom_formatter(|n, _| { parse_volume_output(n) })
                    .custom_parser(|s| { format_volume_input(s) })
                ).changed() {
                    save_file.write_se_volume(se_music_input);
                }
            });
            ui.end_row();

            ui.label("Text Skip");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.menu_button(format!("{}  {}", text_skip_to_str(text_skip_input), egui_material_icons::icons::ICON_KEYBOARD_ARROW_DOWN), |ui| {
                    for skip in [
                        save::TextSkip::Off,
                        save::TextSkip::SingleBoxSkip,
                        save::TextSkip::FullAutoSkip,
                    ] {
                        if ui.selectable_label(text_skip_input == skip, text_skip_to_str(skip)).clicked() {
                            save_file.write_text_skip(skip);
                            ui.close();
                        }
                    }
                });
            });
            ui.end_row();

            ui.label("Screen Shake");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                style::apply_checkbox_style(ui);
                
                let button;
                if screen_shake_input {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX);
                } else {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX_OUTLINE_BLANK);
                }
                if button.clicked() {
                    save_file.write_screen_shake(!screen_shake_input);
                }
            });
            ui.end_row();

            ui.label("Vibration");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                style::apply_checkbox_style(ui);

                let button;
                if vibration_input {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX);
                } else {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX_OUTLINE_BLANK);
                }
                if button.clicked() {
                    save_file.write_vibration(!vibration_input);
                }
            });
            ui.end_row();

            ui.label("Text Box Transparency");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.menu_button(format!("{}  {}", text_box_transparency_to_str(text_box_transparency_input), egui_material_icons::icons::ICON_KEYBOARD_ARROW_DOWN), |ui| {
                    for transparency in [
                        save::TextBoxTransparency::Off,
                        save::TextBoxTransparency::Low,
                        save::TextBoxTransparency::High,
                    ] {
                        if ui.selectable_label(text_box_transparency_input == transparency, text_box_transparency_to_str(transparency)).clicked() {
                            save_file.write_text_box_transparency(transparency);
                            ui.close();
                        }
                    }
                });
            });
            ui.end_row();

            ui.label("Language");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.menu_button(format!("{}  {}", language_to_str(language_input), egui_material_icons::icons::ICON_KEYBOARD_ARROW_DOWN), |ui| {
                    for lang in [
                        save::Language::Japanese, save::Language::English, save::Language::French, save::Language::German,
                        save::Language::Korean, save::Language::ChineseSimplified, save::Language::ChineseTraditional,
                    ] {
                        if ui.selectable_label(language_input == lang, language_to_str(lang)).clicked() {
                            save_file.write_language(lang);
                            ui.close();
                        }
                    }
                });
            });
            ui.end_row();

            ui.label("Resolution Width");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut width = save_file.read_resolution_width() / 16;
                if ui.add(
                    egui::DragValue::new(&mut width)
                    .range(1..=480)
                    .clamp_existing_to_range(false)
                    .custom_formatter(|n, _| {
                        format!("{}", n as u32 * 16)
                    })
                    .custom_parser(|s| {
                        s.trim().parse::<u32>().ok().map(|n| (n / 16) as f64)
                    })
                ).changed() {
                    let width = width as u32 * 16;
                    let height = (width * 9) / 16;
                    save_file.write_resolution_width(width as u16);
                    save_file.write_resolution_height(height as u16);
                }
            });
            ui.end_row();

            ui.label("Resolution Height");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut height = save_file.read_resolution_height() / 9;
                if ui.add(
                    egui::DragValue::new(&mut height)
                    .range(1..=270)
                    .clamp_existing_to_range(false)
                    .custom_formatter(|n, _| {
                        format!("{}", n as u32 * 9)
                    })
                    .custom_parser(|s| {
                        s.trim().parse::<u32>().ok().map(|n| (n / 9) as f64)
                    })
                ).changed() {
                    let height = height as u32 * 9;
                    let width = (height * 16) / 9;
                    save_file.write_resolution_height(height as u16);
                    save_file.write_resolution_width(width as u16);
                }
            });
            ui.end_row();

            ui.label("Fullscreen");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                style::apply_checkbox_style(ui);

                let button;
                if fullscreen_input {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX);
                } else {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX_OUTLINE_BLANK);
                }
                if button.clicked() {
                    save_file.write_fullscreen(!fullscreen_input);
                }
            });
            ui.end_row();

            ui.label("VSync");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                style::apply_checkbox_style(ui);

                let button;
                if vsync_input {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX);
                } else {
                    button = ui.button(egui_material_icons::icons::ICON_CHECK_BOX_OUTLINE_BLANK);
                }
                if button.clicked() {
                    save_file.write_vsync(!vsync_input);
                }
            });
            ui.end_row();
        });
    });
}