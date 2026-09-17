mod common;
use common::{TestingEnvironment, build_config, run_binary};
use greprs::run;
use std::fs;

#[test]
fn test_search_finds_single_match() {
    let content = "Rust:\nsafe, fast, productive.\nPick three.";
    let env = TestingEnvironment::new_file(content);

    // build_config(file_path, query, ignore_case, recursive, line_number)
    let config = build_config(
        env.file_path.clone(),
        "productive",
        false,
        false,
        false,
        false,
    );

    assert!(run(config).is_ok());

    let output = run_binary(&["productive", env.file_path.to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("1 occurrences found:"));
    assert!(stdout.contains("safe, fast, productive."));
}

#[test]
fn test_search_returns_ok_when_no_match() {
    let content = "Rust:\nsafe, fast, productive.";
    let env = TestingEnvironment::new_file(content);
    let config = build_config(
        env.file_path.clone(),
        "nonexistent",
        false,
        false,
        false,
        false,
    );

    assert!(run(config).is_ok());

    let output = run_binary(&["nonexistent", env.file_path.to_str().unwrap()]);
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("0 occurrences found:")
    );
}

#[test]
fn test_binary_file_is_reported_without_stopping_search() {
    let temp_dir = tempfile::tempdir().unwrap();
    let valid_file = temp_dir.path().join("valid.txt");
    let binary_file = temp_dir.path().join("binary.dat");
    fs::write(&valid_file, "needle in text\n").unwrap();
    fs::write(&binary_file, [0, 159, 146, 150]).unwrap();

    let output = run_binary(&["-r", "needle", temp_dir.path().to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stdout.contains("1 occurrences found:"));
    assert!(stdout.contains("needle in text"));
    assert!(stderr.contains("binary.dat"));
}

#[test]
fn test_missing_file_is_reported_without_failing_process() {
    let temp_dir = tempfile::tempdir().unwrap();
    let missing_file = temp_dir.path().join("missing.txt");

    let output = run_binary(&["needle", missing_file.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("missing.txt")
    );
}
