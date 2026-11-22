#![allow(unused_parens)]
#![allow(unused_must_use)]
use std::process::{Command, Stdio, Output};

pub fn run_initial_kernel_check() -> String {
    let output: Output = Command::new("uname")
        .args(["-a"])
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run initial kernel check");

    if !output.status.success() || output.stdout.is_empty() {
        return String::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    return stdout.to_string();
}