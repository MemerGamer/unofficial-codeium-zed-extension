use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};
use uuid::Uuid;

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
                // Generate a unique UUID for the auth request
                let uuid = Uuid::new_v4();
                
                // Construct the URL for authentication
                let url = format!(
                    "https://codeium.com/profile?response_type=token&redirect_uri=vim-show-auth-token&state={uuid}&scope=openid%20profile%20email&redirect_parameters_type=query"
                );

                let message = format!(
                    "Please open the following URL in your browser to authenticate with Codeium:\n\n{}",
                    url
                );

                Ok(SlashCommandOutput {
                    text: message.clone(),
                    sections: vec![SlashCommandOutputSection {
                        label: "Codeium Authentication Link".to_string(),
                        range: (0..message.len()).into(),
                    }],
                })
            }
            command => Err(format!("Unknown command: \"{command}\"")),
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
