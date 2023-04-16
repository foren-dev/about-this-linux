use std::process::Command;

use eframe::{
    egui::{RichText, Ui},
    epaint::Color32,
};

pub struct CommandUtils;

impl CommandUtils {
    ///Executing a command and converting it to a String. The pop removes the extra line in the output
    pub fn get_command_output(command: &str) -> String {
        let mut packages = String::from_utf8(
            Command::new("sh")
                .args(["-c", command])
                .output()
                .unwrap()
                .stdout,
        )
        .expect("Failed to execute command");
        packages.pop();
        packages
    }

    /// Creates an explicit label
    pub fn create_explicit_label(ui: &mut Ui, text: RichText, space: f32) {
        ui.add_space(space);
        ui.label(text);
    }

    /// A cleaner create label that has the rich text to make the main code cleaner
    pub fn create_label(ui: &mut Ui, text: &str, color: Color32, size: f32, space: f32) {
        Self::create_explicit_label(ui, RichText::new(text).color(color).size(size), space);
    }
}
