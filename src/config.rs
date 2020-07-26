use ansi_term::Color;
use crate::prompts::dir::DirStyle;

pub struct PromptConfig {
    pub git_status_clean_symbol: &'static str,
    pub git_status_unstaged_symbol: &'static str,
    pub git_status_staged_symbol: &'static str,
    pub prompt_symbol: &'static str,
    pub dir_style: DirStyle,
    pub dir_home_symbol: Option<&'static  str>,
    pub dir_color: Color,
    pub git_branch_color: Color,
    pub git_hash_color: Color,
    pub git_status_color: Color,
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
