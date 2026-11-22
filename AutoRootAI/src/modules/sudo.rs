#![allow(unused_parens)]
#![allow(unused_must_use)]
use std::process::{Command, Stdio, Output};

pub fn run_initial_sudo() -> String {
    let output: Output = Command::new("sudo")
        .args(["-l"])
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run sudo -l");

    if !output.status.success() || output.stdout.is_empty() {
        return String::new();
    }

    let stdout= String::from_utf8_lossy(&output.stdout);
    return stdout.to_string();
}