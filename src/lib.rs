use std::result::Result;
use zed_extension_api::{lsp::{Completion, Symbol}, Command as ZedCommand, Extension, Worktree};

struct OpenCodeiumSite;

impl OpenCodeiumSite {
    fn execute(&self) -> Result<(), String>{
        // Open codeium site in the browser
        webbrowser::open("https://codeium.com")
        .map_err(|_| "Failed to open the browser.".to_string())
    }
}

struct CodeiumExtension;

impl CodeiumExtension{
    fn new() -> Self {
        // Triggers the codeium extension
        OpenCodeiumSite.execute().unwrap();
        CodeiumExtension
    }
}

impl Extension for CodeiumExtension {
    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<ZedCommand> {
        Err("`language_server_command` not implemented".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _completion: Completion,
    ) -> Option<zed_extension_api::CodeLabel> {
        None
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _symbol: Symbol,
    ) -> Option<zed_extension_api::CodeLabel> {
        None
    }

    fn complete_slash_command_argument(
        &self,
        _command: zed_extension_api::SlashCommand,
        _args: Vec<String>,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        Ok(Vec::new())
    }

    fn run_slash_command(
        &self,
        _command: zed_extension_api::SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> zed_extension_api::Result<zed_extension_api::SlashCommandOutput, String> {
        Err("`run_slash_command` not implemented".to_string())
    }

    fn suggest_docs_packages(&self, _provider: String) -> zed_extension_api::Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    fn index_docs(
        &self,
        _provider: String,
        _package: String,
        _database: &zed_extension_api::KeyValueStore,
    ) -> zed_extension_api::Result<(), String> {
        Err("`index_docs` not implemented".to_string())
    }
    
    fn new() -> Self
    where
        Self: Sized {
        todo!()
    }
}