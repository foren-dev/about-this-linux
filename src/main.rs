mod utils;

use eframe::{
    egui::{self, RichText},
    epaint::Color32,
};
use lxinfo::info;
use utils::CommandUtils;

const FONT_SIZE: f32 = 13.;
const SPACING: f32 = 9.;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(340., 222.)),
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
        let packages = CommandUtils::get_command_output("pacman -Q | wc -l");
        let username = &self.system_info.username;
        let hostname = &self.system_info.hostname;
        let distro = &self.system_info.distro_name;
        let kernel = &self.system_info.kernel;
        let shell = &self.system_info.shell;
        let uptime = &self.system_info.uptime_formatted;
        let used_mem = &self.system_info.used_mem;
        let total_mem = &self.system_info.total_mem;
        let wm = std::env::var("XDG_CURRENT_DESKTOP").expect("Failed to get WM!");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|centered| {
                centered.heading(
                    RichText::new("About This Linux v3.0")
                        .color(Color32::WHITE)
                        .underline(),
                );
            });
            CommandUtils::create_label(
                ui,
                &format!("{username}@{hostname}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("Distro: {distro}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("WM: {wm}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("Kernel: {kernel}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("Shell: {shell}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("Packages: {packages}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("Uptime: {uptime}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
            CommandUtils::create_label(
                ui,
                &format!("Memory: {used_mem}/{total_mem}"),
                Color32::WHITE,
                FONT_SIZE,
                SPACING,
            );
        });
    }
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
