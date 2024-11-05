use webbrowser;
use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

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
                        sections: vec![SlashCommandOutputSection {
                            range: (0..url.len()).into(),
                            label: "Codeium".to_string(),
                        }],
                    })
                } else {
                    Err("Failed to open Codeium".to_string())
                }
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        CodeiumExtension {}
    }
}

// Register the extension
zed_extension_api::register_extension!(CodeiumExtension);
