use std::fs;

pub fn extract_and_delete_zip(zip_path: &str, output_dir: &str) -> klyx_extension_api::Result<()> {
    klyx_extension_api::unzip_file(zip_path, output_dir)?;
    fs::remove_file(zip_path).map_err(|e| e.to_string())?;
    Ok(())
}
