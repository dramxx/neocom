use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;
use std::str;

// STATUS COMMAND
#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("status").assert().success();
}

#[test]
fn test_status_output() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("status")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("ONLINE") || stdout.contains("Players:"));
}

// WH COMMAND
#[test]
fn test_wh_c3() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("wh").arg("c3").assert().success();
}

#[test]
fn test_wh_classes() {
    for class in &["c1", "c2", "c3", "c4", "c5", "c6"] {
        let output = StdCommand::new("./target/release/neocom.exe")
            .arg("wh")
            .arg(class)
            .output()
            .unwrap();
        assert!(output.status.success(), "wh {} failed", class);
    }
}

// SYSTEM COMMAND
#[test]
fn test_system_jita() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("system").arg("Jita").assert().success();
}

#[test]
fn test_system_output() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("system")
        .arg("Jita")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("System:") && stdout.contains("Security:"));
}

// TRAVEL COMMAND
#[test]
fn test_travel_jita_amarr() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("travel")
        .arg("Jita")
        .arg("Amarr")
        .assert()
        .success();
}

#[test]
fn test_travel_output() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("travel")
        .arg("Jita")
        .arg("Amarr")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Route:") && stdout.contains("jumps"));
}

// PRICE COMMAND
#[test]
fn test_price_tritanium() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("price").arg("Tritanium").assert().success();
}

#[test]
fn test_price_output() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("price")
        .arg("Tritanium")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("ISK"));
}

// INTEL COMMAND
#[test]
fn test_intel_requires_arg() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("intel").assert().failure();
}

#[test]
fn test_intel_unknown_pilot() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("intel")
        .arg("NobodyKnown12345")
        .output()
        .unwrap();
    // Should handle gracefully
    let text = format!(
        "{}{}",
        str::from_utf8(&output.stdout).unwrap(),
        str::from_utf8(&output.stderr).unwrap()
    );
    assert!(text.contains("Unknown") || output.status.success());
}

// HELP
#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("help").assert().success();
}

#[test]
fn test_help_all_commands() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("help")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    for cmd in &["travel", "price", "intel", "system", "wh", "status"] {
        assert!(stdout.contains(cmd), "help missing {}", cmd);
    }
}

// ERROR HANDLING
#[test]
fn test_unknown_command_fails() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("foobar").assert().failure();
}

#[test]
fn test_price_no_args_fails() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("price").assert().failure();
}

// VERSION
#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("neocom").unwrap();
    cmd.arg("--version").assert().success();
}
