use zed_extension_api::{self as zed, serde_json::json};

#[derive(Default)]
struct TexpressoExtension {}

impl zed::Extension for TexpressoExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = worktree
            .which("texpresso-lsp")
            .ok_or("texpresso-lsp not on PATH")?;
        let args = vec!["--stdio".to_string()];
        let env = vec![];

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let mut init_opts = zed::settings::LspSettings::for_worktree("texpresso-lsp", worktree)
            .unwrap_or_default()
            .initialization_options
            .unwrap_or_default();

        if init_opts.get("inverse_search").is_none() {
            init_opts["inverse_search"] = json!({
                "command": "zed",
                "arguments": ["%f:%l"]
            });
        }
        // FUTURE: if root_tex not specified, search worktree for a suitable .tex file
        // if !init_opts.get("root_tex").is_none() {
        // }

        Ok(Some(init_opts))
    }
}

zed::register_extension!(TexpressoExtension);
