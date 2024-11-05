use zed_extension_api::{
    self as zed,
    SlashCommand, SlashCommandOutput, Worktree,
};
use webbrowser;

struct CodeiumExtension {}

impl zed::Extension for CodeiumExtension {
    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "open-codeium" => {
                let url = "https://codeium.com";
                if webbrowser::open(url).is_ok() {
                    Ok(SlashCommandOutput {
                        text: "Codeium opened".to_string(),
                        sections: vec![],
                    })
                } else {
                    Err("Failed to open Codeium".to_string())
                }
            }
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        CodeiumExtension {}
    }
}