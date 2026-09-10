#![allow(dead_code)]

use greprs::Config;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestingEnvironment {
    _temp_dir: TempDir,
    pub file_path: PathBuf,
}

impl TestingEnvironment {
    /// Create a new temporary testing environment with a file containing the specified content.
    pub fn new_file(content: &str) -> Self {
        let temp_dir = tempfile::tempdir().expect("Error during creating a temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");

        fs::write(&file_path, content).expect("Error during writing to the temporary file");

        Self {
            _temp_dir: temp_dir,
            file_path,
        }
    }

    /// Create a new temporary testing environment with a directory tree containing a file in a subdirectory.
    pub fn new_directory_tree() -> (Self, PathBuf) {
        let temp_dir = tempfile::tempdir().expect("Error creating temp dir");
        let sub_dir = temp_dir.path().join("sub_folder");
        fs::create_dir(&sub_dir).expect("Error creating subfolder");

        let file_path = sub_dir.join("nested_file.txt");
        fs::write(&file_path, "target query in nested folder").expect("Error writing file");

        let env = Self {
            _temp_dir: temp_dir,
            file_path: file_path.clone(),
        };

        (env, sub_dir)
    }
}

/// Build a Config object for testing purposes.
pub fn build_config(
    file_path: PathBuf,
    query: &str,
    ignore_case: bool,
    recursive: bool,
    line_number: bool,
) -> Config {
    Config {
        query: query.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
        ignore_case: ignore_case,
        recursive: recursive,
        line_number: line_number,
    }
}
