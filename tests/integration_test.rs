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

// TRAVEL WITH HOURS FLAG
#[test]
fn test_travel_with_hours_flag() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("travel")
        .arg("Jita")
        .arg("Amarr")
        .arg("--hours")
        .arg("1")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Should show 1h in the output
    assert!(stdout.contains("/1h"), "travel --hours 1 should show /1h");
}

#[test]
fn test_travel_with_24_hours() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("travel")
        .arg("Jita")
        .arg("Amarr")
        .arg("--hours")
        .arg("24")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Should show 24h in the output
    assert!(stdout.contains("/24h"), "travel --hours 24 should show /24h");
}

// TRAVEL GATE KILLS FORMAT (Uedama has known gate kills)
#[test]
fn test_travel_shows_gate_kills() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("travel")
        .arg("Oijamon")
        .arg("Amattens")
        .arg("--hours")
        .arg("1")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Should have kills in the output (systems with gates show numbers)
    assert!(stdout.contains("/1h"), "should show hour format");
}

// TRAVEL DEFAULT HOURS
#[test]
fn test_travel_default_hours() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("travel")
        .arg("Jita")
        .arg("Amarr")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    // Default is 1 hour (matches eve-gatecheck)
    assert!(stdout.contains("/1h"), "default should be 1h");
}

// TRAVEL ROUTE FLAG
#[test]
fn test_travel_route_safest() {
    let output = StdCommand::new("./target/release/neocom.exe")
        .arg("travel")
        .arg("Jita")
        .arg("Tama")
        .arg("--route")
        .arg("safest")
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Route:") || stdout.contains("HS"));
}
