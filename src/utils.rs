use std::process::Command;

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
}