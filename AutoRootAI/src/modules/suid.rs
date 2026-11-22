#![allow(unused_parens)]
#![allow(unused_must_use)]
use std::process::{Command, Stdio};

pub fn run_initial_suid() -> String {
    let output: std::process::Output = Command::new("find")
        .args(["/", "-perm", "-4000"])
        .stderr(Stdio::null())
        .output()
        .expect("Failed to execute find");

    if !output.status.success() && output.stdout.is_empty() {
        return String::new();
    }

    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    return stdout.to_string();
}
