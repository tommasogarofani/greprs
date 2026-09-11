mod common;
use common::{TestingEnvironment, build_config};
use greprs::run;

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

    let result = run(config);
    assert!(result.is_ok());
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

    let result = run(config);
    assert!(result.is_ok());
}
