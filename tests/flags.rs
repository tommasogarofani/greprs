mod common;
use common::{TestingEnvironment, build_config};
use greprs::run;

#[test]
fn test_ignore_case_flag() {
    let content = "Rust:\nSafe, Fast, Productive.";
    let env = TestingEnvironment::new_file(content);

    // Passiamo ignore_case = true
    let config = build_config(env.file_path.clone(), "safe", true, false, false);

    let result = run(config);
    assert!(result.is_ok());
}

#[test]
fn test_line_number_flag() {
    let content = "First line\nSecond line\nTarget line";
    let env = TestingEnvironment::new_file(content);

    // Passiamo line_number = true
    let config = build_config(env.file_path.clone(), "Target", false, false, true);

    let result = run(config);
    assert!(result.is_ok());
}
