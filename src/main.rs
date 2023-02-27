use eframe::{
    egui::{self, RichText, Ui},
    epaint::Color32,
};
use lxinfo::info;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(340., 206.)),
        resizable: false,
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "About This Linux",
        options,
        Box::new(|_cc| Box::<Info>::default()),
    )
}

impl eframe::App for Info {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|centered| {
                centered.heading(
                    RichText::new("About This Linux v3.0")
                        .color(Color32::WHITE)
                        .underline(),
                );
            });
            create_label(
                ui,
                RichText::new(format!(
                    "{}@{}",
                    self.system_info.username, self.system_info.hostname
                ))
                .color(Color32::WHITE)
                .size(13.),
                10.,
            );
            create_label(
                ui,
                RichText::new(format!("Distribution: {}", self.system_info.distro_name))
                    .color(Color32::WHITE)
                    .size(13.),
                10.,
            );
            create_label(
                ui,
                RichText::new(format!("Kernel: {}", self.system_info.kernel))
                    .color(Color32::WHITE)
                    .size(13.),
                10.,
            );
            create_label(
                ui,
                RichText::new(format!("Shell: {}", self.system_info.shell))
                    .color(Color32::WHITE)
                    .size(13.),
                10.,
            );
            create_label(
                ui,
                RichText::new(format!("Uptime: {}", self.system_info.uptime_formatted))
                    .color(Color32::WHITE)
                    .size(13.),
                10.,
            );
            create_label(
                ui,
                RichText::new(format!(
                    "Memory: {}/{}",
                    self.system_info.used_mem, self.system_info.total_mem
                ))
                .color(Color32::WHITE)
                .size(13.),
                10.,
            );
        });
    }
}

/// Creates a new label with a space to keep code cleaner
fn create_label(ui: &mut Ui, text: RichText, space: f32) {
    ui.add_space(space);
    ui.label(text);
}

/// Implement to give system_info a default value
impl Default for Info {
    fn default() -> Self {
        Self {
            system_info: info::get_system_information().expect("You did sum ting wong"),
        }
    }
}

struct Info {
    system_info: info::SystemInfo,
}
