use std::fs::{self, File};
use std::io;
use std::path::Path;
use zip::ZipArchive;

pub fn extract_and_delete_zip(zip_path: &str, output_dir: &str) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(output_dir).join(file.mangled_name());

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    fs::remove_file(zip_path)?;

    Ok(())
}
