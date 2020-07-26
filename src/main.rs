#[cfg(test)]
mod tests;

mod prompts;
mod config;

use prompts::{
    git::{GitPrompt, Repository},
    dir::DirPrompt
};
use config::PromptConfig;

fn main() {
    let config = PromptConfig::default();

    let dir_prompt_out = DirPrompt.show(&config);

    let git_prompt_out = match Repository::open_from_env() {
        Ok(repo) => {
            //dbg!(repo.workdir().unwrap());
            let git_prompt = GitPrompt::new(repo);
            git_prompt.show(&config)
        }
        Err(_) => {
            "".to_string()
        }
    };

    let out = format!(
        "{} {}\n{} ",
        dir_prompt_out,
        git_prompt_out,
        config.prompt_symbol
    );

    println!("\n{}", out);
}
