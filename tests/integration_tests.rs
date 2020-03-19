mod entities;
use entities::*;

use std::env;
use std::path::PathBuf;
use std::process;

const URL: &str = "http://localhost:5000";
const API_KEY: &str = "ROOT-API-KEY";

/// Find the *listthedocs* executable.
fn find_exe() -> PathBuf {
    // Tests exe is in target/debug/deps, the *listthedocs* exe is in target/debug
    let root = env::current_exe()
        .expect("tests executable")
        .parent()
        .expect("tests executable directory")
        .parent()
        .expect("listthedocs executable directory")
        .to_path_buf();

    let exe_name = if cfg!(windows) {
        "listthedocs.exe"
    } else {
        "listthedocs"
    };

    root.join(exe_name)
}

#[test]
fn add_update_delete_project() -> Result<(), serde_json::Error> {
    let exe = find_exe();

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "-a", API_KEY, "project", "remove", "test"]);
    let _ = cmd.output().expect("listthedocs output");

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "project", "get", "test"]);
    let output = cmd.output().expect("listthedocs output");
    assert!(String::from_utf8_lossy(&output.stdout).contains("not found"));

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "-a", API_KEY, "project", "add", "test", "desc"]);
    let output = cmd.output().expect("listthedocs output");
    let result: Project = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
    assert_eq!(result.title, "test");
    assert_eq!(result.code, "test");
    assert_eq!(result.description, "desc");
    assert_eq!(result.logo, None);
    assert_eq!(result.versions, Vec::new());

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "-a", API_KEY, "project", "update", "test", "--logo", "mylogo"]);
    let output = cmd.output().expect("listthedocs output");
    let result: Project = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
    assert_eq!(result.logo, Some("mylogo".to_owned()));

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "-a", API_KEY, "project", "remove", "test"]);
    let output = cmd.output().expect("listthedocs output");
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "test");

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "project", "get", "test"]);
    let output = cmd.output().expect("listthedocs output");
    assert!(String::from_utf8_lossy(&output.stdout).contains("not found"));

    Ok(())
}
