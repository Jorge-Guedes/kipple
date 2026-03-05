use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_dry_run() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path();

    fs::write(test_dir.join("test.jpg"), "contenido").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "--dry-run", "-d", test_dir.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("📁 Pictures"));
    assert!(stdout.contains("test.jpg"));
}

#[test]
fn test_cli_verbose() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path();

    fs::write(test_dir.join("test.jpg"), "contenido").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "-v", "-d", test_dir.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("EVALUATING:"));
    assert!(stdout.contains("CLASSIFIED:"));
}

#[test]
fn test_cli_include_dirs() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path();

    fs::create_dir(test_dir.join("subdir")).unwrap();
    fs::write(test_dir.join("subdir").join("test.jpg"), "contenido").unwrap();

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--include-dirs",
            "-v",
            "-d",
            test_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DIR:") || stdout.contains("subdir"));
    assert!(stdout.contains("test.jpg"));
}

#[test]
fn test_cli_force_flag() {
    let dir = tempdir().unwrap();
    let test_dir = dir.path();

    fs::write(test_dir.join("test.jpg"), "original").unwrap();
    let output1 = Command::new("cargo")
        .args(["run", "--", "-d", test_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output1.status.success());

    fs::write(test_dir.join("test.jpg"), "nuevo").unwrap();
    let output2 = Command::new("cargo")
        .args(["run", "--", "--force", "-d", test_dir.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output2.status.success());
    let stdout = String::from_utf8_lossy(&output2.stdout);

    assert!(!stdout.contains("DUPLICATE:"));
    assert!(!stdout.contains("RENAMED:"));
}
