use eframe::egui;
use crate::AppState;
use crate::save;

const GAME_TITLES: [&str; 3] = [
    "Phoenix Wright: Ace Attorney",
    "Phoenix Wright: Justice for All",
    "Phoenix Wright: Trials and Tribulations",
];

pub fn show_chapters_tab(ui: &mut egui::Ui, app: &mut AppState) {
    if let Some(save_file) = &mut app.save_file {
        egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            egui::Grid::new("settings_grid")
            .num_columns(2)
            .spacing([16.0, 8.0])
            .show(ui, |ui| {
                for i in 0..save::GAME_COUNT {
                    let chapter = save_file.read_chapter_from_game(i);
                    let mut chapter_input = chapter as u32;

                    ui.label(format!("{}", GAME_TITLES[i]));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let high_chapter = if i == 1 { 4 } else { 5 };
                        if ui.add(egui::DragValue::new(&mut chapter_input).range(1..=high_chapter)).changed() {
                            let chapter_u8 = chapter_input as u8;
                            save_file.write_chapter_to_game(i, chapter_u8);
                        }
                    });
                    ui.end_row();
                }
            });
        });
    } else {
        app.error = Some("No save file loaded".to_string());
    }
}