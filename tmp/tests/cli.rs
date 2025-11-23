//! CLI integration tests for template-rs

use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "CLI help command failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--help"),
        "Help output should contain help flag"
    );
    assert!(
        stdout.contains("--version"),
        "Help output should contain version flag"
    );
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "CLI version command failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("template-rs"),
        "Version output should contain program name"
    );
}

#[test]
fn test_cli_no_args() {
    let output = Command::new("cargo")
        .args(["run"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "CLI with no args should succeed");
}
