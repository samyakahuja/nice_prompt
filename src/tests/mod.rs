mod test_helpers;

use git2::Repository;
use temp_testdir::TempDir;
use std::path::PathBuf;
use std::fs::{File, remove_file};
use std::io::Write;
//use test_helpers::*;
use crate::Prompt;

#[test]
fn prompt_git_status() {
    let temp_dir = TempDir::default();
    let temp_dir_path = PathBuf::from(temp_dir.as_ref());

    // Init repo
    let repo = Repository::init(&temp_dir_path).unwrap();
    let prompt = Prompt::new(repo).unwrap();
    assert_eq!(prompt.git_status(), None);

    // Create file
    let file_name_01 = "test_file_01";
    let temp_file_path_01 = temp_dir_path.join(PathBuf::from(file_name_01));
    let mut file_01 = File::create(&temp_file_path_01).unwrap();
    assert_eq!(prompt.git_status().unwrap(), "?".to_string());

    // Add file
    let mut index = prompt.repo.index().unwrap();

    assert_eq!(index.len(), 0);
    index.add_path(PathBuf::from(file_name_01).as_path()).unwrap();
    assert_eq!(index.len(), 1);

    let index_entry_paths = index
        .iter()
        .map(|entry| {
            String::from_utf8(entry.path).unwrap()
        })
        .collect::<Vec<_>>();

    assert_eq!(index_entry_paths, [file_name_01]);
    assert_eq!(prompt.git_status(), None);

    // Modify existing file
    file_01.write_all(b"Hello, World!").unwrap();
    assert_eq!(prompt.git_status().unwrap(), "!".to_string());

    // Create new file
    let file_name_02 = "test_file_02";
    let temp_file_path_02 = temp_dir_path.join(PathBuf::from(file_name_02));
    File::create(&temp_file_path_02).unwrap();
    assert!(prompt.git_status().unwrap().contains("?"));
    assert!(prompt.git_status().unwrap().contains("!"));

    // Add both files
    index.add_path(PathBuf::from(file_name_01).as_path()).unwrap();
    assert!(!prompt.git_status().unwrap().contains("!"));
    index.add_path(PathBuf::from(file_name_02).as_path()).unwrap();
    assert_eq!(prompt.git_status(), None);

    // Delete file
    remove_file(temp_file_path_02).unwrap();
    assert!(prompt.git_status().unwrap().contains("✘"));
    index.remove_path(PathBuf::from(file_name_02).as_path()).unwrap();
    assert_eq!(prompt.git_status(), None);
}
