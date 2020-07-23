mod test_helpers;

use git2::Repository;
use temp_testdir::TempDir;
use std::path::PathBuf;
use std::fs::{File, remove_file};
use std::io::Write;
//use test_helpers::*;
use crate::{
    GitPrompt,
    GitStatus,
    PromptConfig
};

#[test]
fn git_prompt() {
    let temp_dir = TempDir::default();
    let temp_dir_path = PathBuf::from(temp_dir.as_ref());

    // Init repo
    let repo = Repository::init(&temp_dir_path).unwrap();
    let prompt = GitPrompt::new(repo).unwrap();
    assert_eq!(prompt.git_status().unwrap(), GitStatus::Clean);

    // Create file
    let file_name_01 = "test_file_01";
    let temp_file_path_01 = temp_dir_path.join(PathBuf::from(file_name_01));
    File::create(&temp_file_path_01).unwrap();
    assert_eq!(prompt.git_status().unwrap(), GitStatus::Unstaged);

    // Add file
    let mut index = prompt.repo.index().unwrap();
    index.add_path(PathBuf::from(file_name_01).as_path()).unwrap();

    let index_entry_paths = index
        .iter()
        .map(|entry| {
            String::from_utf8(entry.path).unwrap()
        })
        .collect::<Vec<_>>();

    assert_eq!(index_entry_paths, [file_name_01]);
    assert_eq!(prompt.git_status().unwrap(), GitStatus::Staged);
}
