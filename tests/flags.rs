mod common;
use common::{TestingEnvironment, build_config, run_binary};
use greprs::run;

#[test]
fn test_ignore_case_flag() {
    let content = "Rust:\nSafe, Fast, Productive.";
    let env = TestingEnvironment::new_file(content);

    // Passiamo ignore_case = true
    let config = build_config(env.file_path.clone(), "safe", true, false, false, false);

    assert!(run(config).is_ok());

    let output = run_binary(&["--ignore-case", "safe", env.file_path.to_str().unwrap()]);
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("Safe, Fast, Productive.")
    );
}

#[test]
fn test_line_number_flag() {
    let content = "First line\nSecond line\nTarget line";
    let env = TestingEnvironment::new_file(content);

    // Passiamo line_number = true
    let config = build_config(env.file_path.clone(), "Target", false, false, true, false);

    assert!(run(config).is_ok());

    let output = run_binary(&["--line-number", "Target", env.file_path.to_str().unwrap()]);
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("3: Target line")
    );
}

#[test]
fn test_invert_match_flag() {
    let content = "Rust:\nSafe, Fast, Productive.";
    let env = TestingEnvironment::new_file(content);

    let config = build_config(env.file_path.clone(), "Safe", false, false, false, true);

    assert!(run(config).is_ok());

    let output = run_binary(&["--invert-match", "Safe", env.file_path.to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Rust:"));
    assert!(!stdout.contains("Safe, Fast, Productive."));
}

#[test]
fn test_directory_without_recursive_flag_returns_error() {
    let temp_dir = tempfile::tempdir().unwrap();

    let output = run_binary(&["needle", temp_dir.path().to_str().unwrap()]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("--recursive")
    );
}
