use git2::{
    Repository,
    Status,
};

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn main() {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("Repo not found {}", e)
    };
    println!("{:?}", repo.workdir().unwrap());
    let prompt = GitPrompt::new(repo).unwrap();
    let config = PromptConfig::default();
    println!("{}", prompt.show(&config));
}

pub struct PromptConfig {
    git_status_clean_symbol: &'static str,
    git_status_unstaged_symbol: &'static str,
    git_status_staged_symbol: &'static str,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            git_status_clean_symbol: "✓",
            git_status_unstaged_symbol: "!",
            git_status_staged_symbol: "+",
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
                dbg!(err.code());
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
        let oid_12 = oid.to_string()[1..13].to_string();
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
                Some(hash) => format!("[{}:{} {}]", branch, hash, status_symbol),
                None => format!("[{} {}]", branch, status_symbol),
            },
            None => format!("[{}]", status_symbol)
        }
    }
}
