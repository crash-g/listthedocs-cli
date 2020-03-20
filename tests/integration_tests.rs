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
    let project_title = "test-project";

    let project = Project {
        title: project_title.to_owned(),
        code: project_title.to_owned(),
        description: "testing project".to_owned(),
        logo: None,
        versions: vec![],
    };

    let exe = find_exe();

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "project",
        "remove",
        project_title,
    ]);
    let _ = cmd.output().expect("listthedocs output");

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "project", "get", project_title]);
    let output = cmd.output().expect("listthedocs output");
    assert!(String::from_utf8_lossy(&output.stdout).contains("not found"));

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "project",
        "add",
        project_title,
        &project.description,
    ]);
    let output = cmd.output().expect("listthedocs output");
    let result: Project = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
    assert_eq!(result, project);

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "project",
        "update",
        project_title,
        "--logo",
        "mylogo",
    ]);
    let output = cmd.output().expect("listthedocs output");
    let result: Project = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
    assert_eq!(result.logo, Some("mylogo".to_owned()));

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "project",
        "remove",
        project_title,
    ]);
    let output = cmd.output().expect("listthedocs output");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        project_title
    );

    let mut cmd = process::Command::new(&exe);
    cmd.args(&["-j", "-u", URL, "project", "get", project_title]);
    let output = cmd.output().expect("listthedocs output");
    assert!(String::from_utf8_lossy(&output.stdout).contains("not found"));

    Ok(())
}

#[test]
fn add_update_delete_version() -> Result<(), serde_json::Error> {
    let project_title = "test-version";
    let version_name = "1.0.0";
    let version_url = "http://example.com";

    let project = Project {
        title: project_title.to_owned(),
        code: project_title.to_owned(),
        description: "testing versions".to_owned(),
        logo: None,
        versions: vec![Version {
            name: version_name.to_owned(),
            url: version_url.to_owned(),
        }],
    };

    let exe = find_exe();

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "project",
        "remove",
        project_title,
    ]);
    let _ = cmd.output().expect("listthedocs output");

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "project",
        "add",
        project_title,
        "testing versions",
    ]);
    let _ = cmd.output().expect("listthedocs output");

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "version",
        "add",
        project_title,
        version_name,
        version_url,
    ]);
    let output = cmd.output().expect("listthedocs output");
    let result: Project = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
    assert_eq!(result, project);

    let updated_url = "http://updated.com";
    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "version",
        "update",
        project_title,
        version_name,
        updated_url,
    ]);
    let output = cmd.output().expect("listthedocs output");
    let result: Project = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
    assert_eq!(result.versions[0].url, updated_url);

    let mut cmd = process::Command::new(&exe);
    cmd.args(&[
        "-j",
        "-u",
        URL,
        "-a",
        API_KEY,
        "version",
        "remove",
        project_title,
        version_name,
    ]);
    let output = cmd.output().expect("listthedocs output");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        version_name
    );

    Ok(())
}
