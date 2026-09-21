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
fn test_count_flag_prints_only_the_number_of_matches() {
    let content = "needle one\nother line\nneedle two\n";
    let env = TestingEnvironment::new_file(content);

    let output = run_binary(&["--count", "needle", env.file_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "2");
}

#[test]
fn test_count_short_flag_prints_only_the_number_of_matches() {
    let content = "needle one\nother line\nneedle two\n";
    let env = TestingEnvironment::new_file(content);

    let output = run_binary(&["-c", "needle", env.file_path.to_str().unwrap()]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "2");
}

#[test]
fn test_count_flag_prints_only_the_total_for_recursive_search() {
    let temp_dir = tempfile::tempdir().unwrap();
    let first_file = temp_dir.path().join("first.txt");
    let second_file = temp_dir.path().join("second.txt");
    std::fs::write(&first_file, "needle\nother\n").unwrap();
    std::fs::write(&second_file, "needle\nneedle\n").unwrap();

    let output = run_binary(&[
        "--count",
        "--recursive",
        "needle",
        temp_dir.path().to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "3");
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
