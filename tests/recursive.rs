mod common;
use common::{TestingEnvironment, build_config};
use greprs::run;

#[test]
fn test_recursive_search_in_subdirectories() {
    let (_env, sub_dir) = TestingEnvironment::new_directory_tree();

    // Passiamo la cartella come percorso e recursive = true
    let config = build_config(sub_dir, "target", false, true, false);

    let result = run(config);
    assert!(result.is_ok());
}
