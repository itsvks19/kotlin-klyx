use std::fs;

use klyx_extension_api::{self as klyx, Result};

use crate::language_servers::archive_utils;

pub struct KotlinLanguageServer {
    cached_binary_path: Option<String>,
}

impl KotlinLanguageServer {
    pub const LANGUAGE_SERVER_ID: &'static str = "kotlin-language-server";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }
}

impl KotlinLanguageServer {
    pub fn language_server_binary_path(
        &mut self,
        language_server_id: &klyx::LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = self.cached_binary_path.as_ref() {
            return Ok(path.clone());
        }

        klyx::set_language_server_installation_status(
            language_server_id,
            &klyx::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = klyx::latest_github_release(
            "fwcd/kotlin-language-server",
            klyx::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = "server.zip";
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| "no asset found")?;

        let user_home =
            std::env::var("USER_HOME").map_err(|_| "Failed to get user home directory")?;
        let version_dir = format!("{user_home}/kotlin-language-server-{}", release.version);
        let binary_path = format!("{version_dir}/server/bin/kotlin-language-server");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            klyx::set_language_server_installation_status(
                language_server_id,
                &klyx::LanguageServerInstallationStatus::Downloading,
            );

            let zip_path = format!("{version_dir}/{asset_name}");
            klyx::download_file(&asset.download_url, &zip_path)
                .map_err(|e| format!("failed to download file error: {e}"))?;

            archive_utils::extract_and_delete_zip(&zip_path, &version_dir)
                .map_err(|e| format!("failed to extract archive: {e}"))?;

            klyx::make_file_executable(&binary_path)
                .map_err(|e| format!("failed to make binary executable: {e}"))?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}
