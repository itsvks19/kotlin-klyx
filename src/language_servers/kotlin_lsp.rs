use std::path::Path;

use klyx_extension_api::{self as klyx, Result, make_file_executable};

use crate::language_servers::archive_utils;

pub struct KotlinLSP {
    cached_binary_path: Option<String>,
}

impl KotlinLSP {
    pub const LANGUAGE_SERVER_ID: &'static str = "kotlin-lsp";

    pub fn new() -> Self {
        KotlinLSP {
            cached_binary_path: None,
        }
    }

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
        let version = get_version()?;

        klyx::set_language_server_installation_status(
            language_server_id,
            &klyx::LanguageServerInstallationStatus::Downloading,
        );

        let binary_path = download_from_teamcity(&version)?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

fn extract_version_from_markdown(contents: &str) -> Option<String> {
    contents
        .lines()
        .find_map(|line| line.strip_prefix("### v"))
        .map(|version| version.to_string())
}

/// Return URL to the kotlin-lsp package on TeamCity servers
fn get_version() -> Result<String> {
    let url = "https://raw.githubusercontent.com/Kotlin/kotlin-lsp/refs/heads/main/RELEASES.md"
        .to_string();
    let result = klyx::http_client::fetch(&klyx::http_client::HttpRequest {
        method: klyx::http_client::HttpMethod::Get,
        url,
        headers: vec![],
        body: None,
        redirect_policy: klyx::http_client::RedirectPolicy::NoFollow,
    })?;
    let body =
        String::from_utf8(result.body).map_err(|_| "Failed to fetch RELEASES.md".to_owned())?;
    extract_version_from_markdown(&body)
        .ok_or_else(|| "Failed to extract version from RELEASES.md".into())
}

fn download_from_teamcity(version: &str) -> Result<String> {
    let url =
        format!("https://download-cdn.jetbrains.com/kotlin-lsp/{version}/kotlin-{version}.zip");
    let user_home = std::env::var("USER_HOME").map_err(|_| "Failed to get user home directory")?;
    let target_dir = format!("{}/kotlin-lsp-{version}", user_home);
    let zip_path = format!("{}/kotlin-{version}.zip", target_dir);
    let script_path = format!("{}/kotlin-lsp.sh", target_dir);

    if !Path::new(&target_dir).exists() {
        klyx::download_file(&url, &zip_path).map_err(|e| format!("failed to download zip: {e}"))?;

        archive_utils::extract_and_delete_zip(&zip_path, &target_dir)
            .map_err(|e| format!("failed to extract zip: {e}"))?;

        make_file_executable(&script_path)
            .map_err(|e| format!("failed to make script executable: {e}"))?;
    }

    Ok(script_path)
}
