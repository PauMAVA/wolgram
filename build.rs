use std::process::Command;

fn main() {
    Command::new("cargo")
        .arg("fmt")
        .spawn()
        .expect("Failed to format code to meet rust standards!");
}
