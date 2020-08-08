#[cfg(test)]
mod tests;

mod prompts;
mod config;
mod logging;
mod util;
mod cli;

use prompts::{
    git::{GitPrompt, Repository},
    dir::DirPrompt
};
use config::PromptConfig;
use logging::setup_logging;
use cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();

    if matches.is_present("logging") {
        setup_logging().expect("Could not setup logging");
    }

    log::trace!("starting application");

    let config = PromptConfig::default();

    let dir_prompt_out = DirPrompt.show(&config);

    let git_prompt_out = match Repository::open_from_env() {
        Ok(repo) => {
            log::debug!("Found git repo: {}", &repo.workdir().unwrap().to_string_lossy());
            let git_prompt = GitPrompt::new(repo);
            git_prompt.show(&config)
        }
        Err(e) => {
            log::error!("No git repo found: {}", e);
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
