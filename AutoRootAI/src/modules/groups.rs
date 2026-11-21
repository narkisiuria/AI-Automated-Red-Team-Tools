#![allow(unused_parens)]
#![allow(unused_must_use)]
use std::process::{Command, Stdio, Output};

pub fn run_initial_groups_check() -> String {
    let output: Output = Command::new("id")
        .args([""])
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run initial group check");

    if !output.status.success() || output.stdout.is_empty() {
        return String::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    return stdout.to_string();
}