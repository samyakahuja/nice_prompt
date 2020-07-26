use git2::{
    Repository,
    Status,
};
use ansi_term::Color;
use std::path::{Path, PathBuf};
use itertools::{Itertools, Position};
use dirs::home_dir;

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn main() {
    let config = PromptConfig::default();

    let dir_prompt_out = DirPrompt.show(&config);

    let git_prompt_out = match Repository::open_from_env() {
        Ok(repo) => {
            //dbg!(repo.workdir().unwrap());
            let git_prompt = GitPrompt::new(repo).unwrap();
            git_prompt.show(&config)
        }
        Err(_) => {
            "".to_string()
        }
    };

    let out = format!("{} {}\n{} ", dir_prompt_out, git_prompt_out, config.prompt_symbol);

    println!("\n{}", out);
}

pub struct PromptConfig {
    git_status_clean_symbol: &'static str,
    git_status_unstaged_symbol: &'static str,
    git_status_staged_symbol: &'static str,
    prompt_symbol: &'static str,
    dir_style: DirStyle,
    dir_home_symbol: Option<&'static  str>,
    dir_color: Color,
    git_branch_color: Color,
    git_hash_color: Color,
    git_status_color: Color,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            git_status_clean_symbol: "✓",
            git_status_unstaged_symbol: "!",
            git_status_staged_symbol: "+",
            prompt_symbol: "❯",
            dir_style: DirStyle::FirstLetterFullPath,
            dir_home_symbol: Some("~"),
            dir_color: Color::White,
            git_branch_color: Color::RGB(69,133,136),
            git_hash_color: Color::RGB(250,189,47),
            git_status_color: Color::RGB(204,36,29),
        }
    }
}

pub struct GitPrompt {
    repo: Repository,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GitStatus {
    Clean,
    Unstaged,
    Staged,
}

impl GitPrompt {
    pub fn new(repo: Repository) -> Result<Self> {
        Ok(GitPrompt { repo })
    }

    pub fn branch(&self) -> Option<String> {
        let head = match self.repo.head() {
            Ok(reference) => reference,
            Err(err) => {
                //dbg!(err.code());
                return if err.code() == git2::ErrorCode::UnbornBranch {
                    // read the branch name from .git/HEAD
                    let head_path = self.repo.path().join("HEAD");
                    let file_contents = std::fs::read_to_string(head_path).ok()?;
                    Some(file_contents.lines().next()?.split("/").last()?.to_string())
                } else {
                    None
                }
            }
        };

        let shorthand = head.shorthand();
        shorthand.map(std::string::ToString::to_string)
    }

    pub fn head_reference_hash(&self) -> Option<String> {
        let oid = self.repo.head().ok()?.target()?;
        let oid_12 = oid.to_string()[0..13].to_string();
        Some(oid_12)
    }

    pub fn git_status(&self) -> Option<GitStatus> {
        let mut repo_status = GitStatus::Clean;

        for file_status in self.repo.statuses(None).ok()?.iter() {
            match file_status.status() {
                // changes in working dir relative to Index
                Status::WT_NEW
                | Status::WT_DELETED
                | Status::WT_RENAMED
                | Status::WT_MODIFIED
                | Status::WT_TYPECHANGE => {
                    repo_status = GitStatus::Unstaged;
                },
                // changes in index relative to the head
                Status::INDEX_NEW
                | Status::INDEX_DELETED
                | Status::INDEX_RENAMED
                | Status::INDEX_MODIFIED
                | Status::INDEX_TYPECHANGE => {
                    repo_status = GitStatus::Staged;
                }
                _ => {}
            }
        }

        Some(repo_status)
    }

    pub fn show(&self, config: &PromptConfig) -> String {
        let branch = self.branch();
        let status = self.git_status();

        let hash = match branch {
            Some(_) => self.head_reference_hash(),
            None => None
        };

        let status_symbol = match status {
            Some(status) => match status {
                GitStatus::Clean => config.git_status_clean_symbol,
                GitStatus::Unstaged => config.git_status_unstaged_symbol,
                GitStatus::Staged => config.git_status_staged_symbol,
            },
            None => ""
        };

        match branch {
            Some(branch) => match hash {
                Some(hash) => format!(
                    "[{}:{} {}]",
                    config.git_branch_color.paint(branch),
                    config.git_hash_color.paint(hash),
                    config.git_status_color.paint(status_symbol)
                ),
                None => format!(
                    "[{} {}]",
                    config.git_branch_color.paint(branch),
                    config.git_status_color.paint(status_symbol)
                ),
            },
            None => format!("[{}]", config.git_status_color.paint(status_symbol))
        }
    }
}

#[derive(Debug)]
enum DirStyle {
    FullPath,
    CurrentDir,
    FirstLetterFullPath,
    ShortestUniqueSymbol,
}

impl DirStyle {
    pub fn apply(&self, path: &Path, config: &PromptConfig) -> String {
        match self {
            Self::FullPath => full_path(path, config),
            Self::CurrentDir => current_dir(path, config),
            Self::FirstLetterFullPath => first_letter_full_path(path, config),
            Self::ShortestUniqueSymbol => shortest_unique_symbol(path, config)
        }
    }
}

// TODO: consider using a crate for this.
fn encode_home_symbol(path: &Path, symbol: &str) -> Option<String> {
    let home_dir = std::env::var("HOME").ok()?;
    let wd = format!("{}", path.display());

    if path.starts_with(&home_dir) {
        Some(wd.replacen(&home_dir, symbol, 1))
    } else {
        None
    }
}

fn full_path(path: &Path, config: &PromptConfig) -> String {
    match config.dir_home_symbol {
        Some(dir_home_symbol) => match encode_home_symbol(path, dir_home_symbol) {
            Some(encoded_path) => encoded_path,
            None => format!("{}", path.display()),
        }
        None => format!("{}", path.display()),
    }
}

/// Outputs only the current directory using `full_path` as a fallback.
fn current_dir(path: &Path, config: &PromptConfig) -> String {
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

fn first_letter_full_path(path: &Path, config: &PromptConfig) -> String {
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

fn path_file_name_to_string(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

fn shortest_unique_symbol(path: &Path, config: &PromptConfig) -> String {
    let mut ret_path = String::new();

    if let Some(dir_home_symbol) = config.dir_home_symbol {
        if let Some(_) = encode_home_symbol(path, dir_home_symbol) {
            ret_path = dir_home_symbol.to_string();
        }
    }

    let mut ancestors = path.ancestors().collect::<Vec<_>>();
    ancestors.reverse();

    for ancestor in ancestors.iter().take(ancestors.len() - 1) {
        dbg!(ancestor);
    }
    dbg!(ancestors.last().unwrap());

    dbg!(&ret_path);
    todo!()
}

#[derive(Debug)]
struct DirPrompt;

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
