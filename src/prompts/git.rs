pub use git2::{
    Repository,
    Status,
};
use crate::config::PromptConfig;

pub struct GitPrompt {
    pub repo: Repository,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GitStatus {
    Clean,
    Unstaged,
    Staged,
}

impl GitPrompt {
    pub fn new(repo: Repository) -> Self {
        log::trace!("Creating a new `GitPrompt`");

        GitPrompt { repo }
    }

    pub fn branch(&self) -> Option<String> {
        log::trace!("Finding git branch");

        let head = match self.repo.head() {
            Ok(reference) => {
                log::debug!("Successfully found reference to HEAD");
                reference
            },
            Err(err) => {
                log::error!("Did not find head reference: {}", err);

                return if err.code() == git2::ErrorCode::UnbornBranch {
                    log::debug!("Trying to read head reference from file `.git/HEAD`");

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
        log::trace!("Calculating reference hash for HEAD");

        let oid = self.repo.head().ok()?.target()?;
        let oid_12 = oid.to_string()[0..13].to_string();
        Some(oid_12)
    }

    pub fn git_status(&self) -> Option<GitStatus> {
        log::trace!("Calculating git status");

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
        log::trace!("Show git prompt");

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

