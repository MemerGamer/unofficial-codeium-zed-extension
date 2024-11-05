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
                #[cfg(target_arch = "wasm32")]
                {
                    let window = web_sys::window().unwrap();
                    if let Err(e) = window.open_with_url("https://codeium.com") {
                        return Err(format!("Failed to open Codeium: {:?}", e));
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if let Err(e) = webbrowser::open("https://codeium.com") {
                        return Err(format!("Failed to open Codeium: {:?}", e));
                    }
                }
                let text = "Opening Codeium website...".to_string();
                Ok(SlashCommandOutput {
                    text: text.clone(),
                    sections: vec![SlashCommandOutputSection {
                        label: "Success".to_string(),
                        range: (0..text.len()).into(),
                    }],
                })
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
