use std::env;
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
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        const PACKAGE_NAME: &str = "texpresso-lsp";
        const SERVER_PATH: &str = "node_modules/texpresso-lsp/dist/server.js";

        // FUTURE allow specifying path directly

        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if let Ok(Some(current_version)) = zed::npm_package_installed_version(PACKAGE_NAME) {
            if latest_version != current_version {
                zed::npm_install_package(PACKAGE_NAME, latest_version.as_str()).ok();
            }
        } else {
            zed::npm_install_package(PACKAGE_NAME, latest_version.as_str()).ok();
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .ok()
                    .ok_or("Failed to get current directory")?
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
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

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let config = zed::settings::LspSettings::for_worktree("texpresso-lsp", worktree)
            .unwrap_or_default()
            .settings
            .unwrap_or_default();

        Ok(Some(config))
    }
}

zed::register_extension!(TexpressoExtension);
