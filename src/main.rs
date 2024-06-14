mod utils;

use gtk::{prelude::*, Box, CssProvider, Label, StyleContext, Window, WindowType};
use lxinfo::info;
use utils::CommandUtils;

static mut LABELS: Vec<Label> = Vec::new();

fn main() {
    gtk::init()
        .expect("Unable to initialize About This Linux. Try again later... or maybe dont...");
    let box_vert = Box::new(gtk::Orientation::Vertical, 0);
    let window = Window::new(WindowType::Popup);
    window.set_title("About This Linux V3");
    window.set_default_size(300, 242);
    window.set_resizable(false);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    load_css(&window);
    create_labels();
    add_labels(&box_vert);
    window.add(&box_vert);
    window.show_all();
    gtk::main();
}

/// We are creating a label, then pushing it to our array which will then get added in another function
fn create_label(text: &str) {
    let label = Label::new(Some(text));
    unsafe { LABELS.push(label) }
}

/// Adding the labels to the window through unsafe code because it was much cleaner than the alternative
fn add_labels(box_vert: &Box) {
    let box_title = Box::new(gtk::Orientation::Horizontal, 6);
    let title = Label::new(Some(""));
    title.set_markup("<u>About This Linux V3</u>");
    box_title.pack_start(&title, true, true, 0);
    unsafe {
        for label in LABELS.iter_mut() {
            let box_hor = Box::new(gtk::Orientation::Horizontal, 0);
            box_hor.pack_start(label, false, false, 6);
            box_vert.pack_start(&box_title, false, false, 6);
            box_vert.pack_start(&box_hor, false, false, 2);
        }
    }
}

/// We're using css styling to allow for transparency, which can have blur if youre on hyprland
fn load_css(window: &Window) {
    let css_provider = CssProvider::new();
    css_provider
        .load_from_data(
            b"
            window{
                background-color: rgba(0, 0, 0, 0.5);
            }
        ",
        )
        .expect("Couldnt load CSS, please check your shit");
    let screen = window.get_screen().expect("Couldnt get screen");
    StyleContext::add_provider_for_screen(
        &screen,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Creating the labels in a separate function to cut down on the amount of code in our main function
fn create_labels() {
    let info = Info::default().system_info;
    create_label(&format!("{}@{}", info.username, info.hostname));
    create_label(&format!("Distro: {}", info.distro_name));
    create_label(&format!(
        "WM: {}",
        &std::env::var("XDG_CURRENT_DESKTOP").expect("Failed to get WM!")
    ));
    create_label(&format!("Shell: {}", info.shell));
    create_label(&format!("Kernel: {}", info.kernel));
    create_label(&format!("Uptime: {}", info.uptime_formatted));
    create_label(&format!(
        "Packages: {} (Pacman), {} (Flatpak)",
        CommandUtils::get_command_output("pacman -Q | wc -l"),
        CommandUtils::get_command_output("flatpak list | wc -l")
    ));
    create_label(&format!(
        "CPU: {}",
        CommandUtils::get_command_output("cpuid -1 | rg \'brand =\' | cut -d \'\"\' -f2")
    ));
    create_label(&format!(
        "GPU: {}",
        CommandUtils::get_command_output("lspci | rg \"VGA\" | cut -d'[' -f2 | cut -d']' -f1")
    ));
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
    pub(crate) system_info: info::SystemInfo,
}
