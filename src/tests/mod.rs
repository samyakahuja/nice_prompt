mod test_helpers;

use git2::Repository;
use temp_testdir::TempDir;
use std::path::{PathBuf, Path};
use std::fs::File;
//use std::io::Write;
//use test_helpers::*;
use crate::{
    prompts::git::{GitPrompt, GitStatus},
    config::PromptConfig,
    prompts::dir::full_path,
    prompts::dir::current_dir,
    prompts::dir::first_letter_full_path,
};

#[test]
fn git_prompt() {
    let temp_dir = TempDir::default();
    let temp_dir_path = PathBuf::from(temp_dir.as_ref());

    // Init repo
    let repo = Repository::init(&temp_dir_path).unwrap();
    let prompt = GitPrompt::new(repo);
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

#[test]
fn dir_style_full_path() {
    let config = PromptConfig::default();

    let path_str = "/home/samyak/Desktop/foo/bar/baz";
    let expected = "~/Desktop/foo/bar/baz";
    assert_eq!(full_path(Path::new(&path_str), &config), expected);

    let path_str = "/home/samyak";
    let expected = "~";
    assert_eq!(full_path(Path::new(&path_str), &config), expected);

    let path_str = "/usr/share";
    let expected = path_str;
    assert_eq!(full_path(Path::new(&path_str), &config), expected);

    let config = PromptConfig {
        dir_home_symbol: None,
        ..PromptConfig::default()
    };

    let path_str = "/home/samyak/Desktop/foo/bar/baz";
    let expected = path_str;
    assert_eq!(full_path(Path::new(&path_str), &config), expected);
}

#[test]
fn dir_style_current_dir() {
    let config = PromptConfig::default();

    let path_str = "/home/samyak/Desktop/foo/bar/baz";
    let expected = "baz";
    assert_eq!(current_dir(Path::new(&path_str), &config), expected);

    let path_str = "/home/samyak";
    let expected = "~";
    assert_eq!(current_dir(Path::new(&path_str), &config), expected);

    let path_str = "/usr/share";
    let expected = "share";
    assert_eq!(current_dir(Path::new(&path_str), &config), expected);

    let config = PromptConfig {
        dir_home_symbol: None,
        ..PromptConfig::default()
    };

    let path_str = "/home/samyak";
    let expected = "samyak";
    assert_eq!(current_dir(Path::new(&path_str), &config), expected);
}

#[test]
fn dir_style_first_letter_full_path() {
    let config = PromptConfig::default();

    let path_str = "/home/samyak/Desktop/foo/bar/baz";
    let expected = "~/D/f/b/baz";
    assert_eq!(first_letter_full_path(Path::new(&path_str), &config), expected);

    let path_str = "/";
    let expected = "/";
    assert_eq!(first_letter_full_path(Path::new(&path_str), &config), expected);

    let path_str = "/home";
    let expected = "/home";
    assert_eq!(first_letter_full_path(Path::new(&path_str), &config), expected);

    let path_str = "/home/samyak";
    let expected = "~";
    assert_eq!(first_letter_full_path(Path::new(&path_str), &config), expected);

    let path_str = "/usr/share";
    let expected = "/u/share";
    assert_eq!(first_letter_full_path(Path::new(&path_str), &config), expected);

    let config = PromptConfig {
        dir_home_symbol: None,
        ..PromptConfig::default()
    };

    let path_str = "/home/samyak";
    let expected = "/h/samyak";
    assert_eq!(first_letter_full_path(Path::new(&path_str), &config), expected);
}
