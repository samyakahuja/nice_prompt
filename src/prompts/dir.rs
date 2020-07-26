use crate::config::PromptConfig;
use std::path::{Path, PathBuf};
use itertools::{Itertools, Position};
use dirs::home_dir;

#[derive(Debug)]
pub struct DirPrompt;

impl DirPrompt {
    pub fn working_dir(&self) -> Option<PathBuf> {
        std::env::current_dir().ok()
    }

    pub fn styled_working_dir(&self, config: &PromptConfig) -> Option<String> {
        let wd = match self.working_dir() {
            Some(x) => x,
            None => return None,
        };

        let wd = config.dir_style.apply(&wd, config);

        Some(wd)
    }

    pub fn show(&self, config: &PromptConfig) -> String {
        let s = self.styled_working_dir(config).unwrap_or("".to_string());
        config.dir_color.paint(s).to_string()
    }
}

#[derive(Debug)]
pub enum DirStyle {
    FullPath,
    CurrentDir,
    FirstLetterFullPath,
}

impl DirStyle {
    pub fn apply(&self, path: &Path, config: &PromptConfig) -> String {
        match self {
            Self::FullPath => full_path(path, config),
            Self::CurrentDir => current_dir(path, config),
            Self::FirstLetterFullPath => first_letter_full_path(path, config),
        }
    }
}

fn encode_home_symbol(path: &Path, symbol: &str) -> Option<String> {
    let home_dir = home_dir()?;
    let wd = format!("{}", path.display());

    if path.starts_with(&home_dir) {
        Some(wd.replacen(&home_dir.to_str()?, symbol, 1))
    } else {
        None
    }
}

fn path_file_name_to_string(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

pub fn full_path(path: &Path, config: &PromptConfig) -> String {
    match config.dir_home_symbol {
        Some(dir_home_symbol) => match encode_home_symbol(path, dir_home_symbol) {
            Some(encoded_path) => encoded_path,
            None => format!("{}", path.display()),
        }
        None => format!("{}", path.display()),
    }
}

/// Outputs only the current directory using `full_path` as a fallback.
pub fn current_dir(path: &Path, config: &PromptConfig) -> String {
    match config.dir_home_symbol {
        Some(dir_home_symbol) => match encode_home_symbol(path, dir_home_symbol) {
            Some(wd) => {
                if dir_home_symbol == wd {
                    return wd;
                }
            }
            None => {}
        },
        None => {}
    }

    match path.components().last() {
        Some(item) => {
            match item.as_os_str().to_str() {
                Some(s) => s.to_string(),
                None => full_path(path, config)
            }
        },
        None => full_path(path, config)
    }
}

pub fn first_letter_full_path(path: &Path, config: &PromptConfig) -> String {
    let mut ret_path = String::new();

    let mut ancestors = path.ancestors().collect::<Vec<_>>();
    ancestors.reverse();

    let home_dir_pos = ancestors
        .iter()
        .position(|&i| i == home_dir().unwrap().as_path());

    if let Some(dir_home_symbol) = config.dir_home_symbol {
        if let Some(index) = home_dir_pos {
                ancestors.drain(..index);
                ret_path.push_str(dir_home_symbol);
        }
    }

    for pos in ancestors.iter().with_position() {
        match pos {
            Position::First(_) => ret_path.push('/'),
            Position::Only(_) => {
                if !home_dir_pos.is_some() {
                    ret_path.push('/');
                }
            },
            _ => {}
        }

        match pos {
            Position::Last(inner) => {
                ret_path.push_str(&path_file_name_to_string(inner).unwrap())
            },
            Position::Only(inner) => {
                if let Some("/") = inner.to_str() {
                    continue;
                }
                if !home_dir_pos.is_some() {
                    ret_path.push_str(&path_file_name_to_string(inner).unwrap())
                }
            },
            Position::First(inner) => {
                if let Some("/") = inner.to_str() {
                    continue;
                }
                if !home_dir_pos.is_some() {
                    dbg!(inner);
                    let first_letter = path_file_name_to_string(inner)
                        .unwrap()
                        .chars()
                        .next()
                        .unwrap();
                    ret_path.push(first_letter);
                }
            },
            Position::Middle(inner) => {
                let first_letter = path_file_name_to_string(inner)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                ret_path.push(first_letter);
            }
        }

        match pos {
            Position::Middle(_) => ret_path.push('/'),
            _ => {}
        }
    }

    ret_path
}
