use std::process::Command;

pub fn clear_screen() {
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");
}
