mod save;
mod file;
mod ui;
mod app_state;
mod tabs;
mod offsets;

use eframe::egui;
use app_state::AppState;
use tabs::Tab;

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::show_error_window(ctx, self);
        
        if !self.save_file.is_none() {
            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui::show_tab_list(ui, self);
            });
        }

        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui::show_bottom_panel(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::show_central_panel(ui, self);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 285.0]),
        ..Default::default()
    };

    eframe::run_native(
        "pwaateditor-rs",
        options,
        Box::new(|cc| {
            egui_material_icons::initialize(&cc.egui_ctx);
            Ok(Box::new(AppState::default()))
        }),
    )
}
