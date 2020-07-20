mod test_helpers;

use git2::{
    Repository,
    Index,
};
use temp_testdir::TempDir;
use std::path::{Path, PathBuf};
use std::fs::File;
use test_helpers::*;
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
    let file_name = "test_file_01";
    let temp_file_path = temp_dir_path.join(PathBuf::from(file_name));
    File::create(&temp_file_path).unwrap();
    assert_eq!(prompt.git_status().unwrap(), "?".to_string());

    // Add file
    let mut index = prompt.repo.index().unwrap();

    assert_eq!(index.len(), 0);
    index.add_path(PathBuf::from(file_name).as_path()).unwrap();
    assert_eq!(index.len(), 1);

    let index_entry_paths = index
        .iter()
        .map(|entry| {
            String::from_utf8(entry.path).unwrap()
        })
        .collect::<Vec<_>>();

    assert_eq!(index_entry_paths, [file_name]);
    assert_eq!(prompt.git_status(), None);
}
