use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::env;

use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};
use uuid::Uuid;

struct CodeiumExtension {}

impl zed::Extension for CodeiumExtension {
    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "codeium-auth" => {
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
            "codeium-save-token" => {
                if args.is_empty() {
                    return Err("No token provided. Usage: codeium-save-token <auth_token>".to_string());
                }

                let token = &args[0];
                
                // Define the path to save the token
                let home_dir = env::var("HOME").map_err(|e| format!("Failed to get home directory: {}", e))?;
                let auth_file_path = PathBuf::from(home_dir).join(".zed-codeium/.authkey");

                // Ensure the directory exists
                if let Some(parent) = auth_file_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
                }

                // Save the token to the file
                let mut file = fs::File::create(&auth_file_path).map_err(|e| format!("Failed to create auth file: {}", e))?;
                file.write_all(token.as_bytes()).map_err(|e| format!("Failed to write token: {}", e))?;

                let message = format!("Token saved successfully to {}", auth_file_path.display());

                Ok(SlashCommandOutput {
                    text: message.clone(),
                    sections: vec![SlashCommandOutputSection {
                        label: "Token Save Status".to_string(),
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
