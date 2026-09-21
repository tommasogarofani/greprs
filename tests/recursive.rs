mod common;
use common::{TestingEnvironment, build_config, run_binary};
use greprs::run;
use std::fs;

#[test]
fn test_recursive_search_in_subdirectories() {
    let (_env, sub_dir) = TestingEnvironment::new_directory_tree();

    // Passiamo la cartella come percorso e recursive = true
    let config = build_config(sub_dir.clone(), "target", false, true, false, false);

    assert!(run(config).is_ok());

    let output = run_binary(&["--recursive", "target", sub_dir.to_str().unwrap()]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("nested_file.txt"));
    assert!(stdout.contains("target query in nested folder"));
}

#[test]
fn test_recursive_search_continues_after_unreadable_content() {
    let temp_dir = tempfile::tempdir().unwrap();
    let valid_file = temp_dir.path().join("valid.txt");
    let binary_file = temp_dir.path().join("binary.dat");
    fs::write(&valid_file, "target valid content\n").unwrap();
    fs::write(&binary_file, [255, 0, 1]).unwrap();

    let output = run_binary(&["--recursive", "target", temp_dir.path().to_str().unwrap()]);

    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("target valid content")
    );
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("binary.dat")
    );
}
