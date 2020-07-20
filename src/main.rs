use git2::{
    Repository,
    Delta,
};
//use std::path::Path;

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn main() {
    let repo = match Repository::open_from_env() {
        Ok(repo) => repo,
        Err(e) => panic!("Repo not found {}", e)
    };
    println!("{:?}", repo.workdir().unwrap());
    let prompt = Prompt::new(repo).unwrap();
    println!("{}", prompt.git_status().unwrap());

}

pub struct Prompt {
    repo: Repository,
}

impl Prompt {
    pub fn new(repo: Repository) -> Result<Self> {
        Ok(Prompt { repo })
    }

    pub fn git_status(&self) -> Option<String> {
        let mut out = String::new();
        let mut seen = Vec::new();

        for status in self.repo.statuses(None).unwrap().iter() {
            if let Some(status) = status.index_to_workdir() {
                let delta = status.status();

                if seen.contains(&delta) {
                    continue;
                }
                seen.push(delta);

                match delta {
                    Delta::Unmodified | Delta::Ignored | Delta::Copied | Delta:: Unreadable => {},
                    Delta::Untracked => out.push('?'),
                    Delta::Added => out.push('+'),
                    Delta::Deleted => out.push('✘'),
                    Delta::Modified | Delta::Renamed | Delta::Typechange => out.push('!'),
                    Delta::Conflicted => out.push('#'),
                }
            }
        }

        if out.is_empty() { None } else { Some(out) }
    }
}
