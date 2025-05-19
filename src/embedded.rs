use rust_embed::RustEmbed;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

pub fn extract_asset_to_file(file_name: &str, dest_path: &PathBuf) -> std::io::Result<()> {
    if let Some(content) = Asset::get(file_name) {
        let data = content.data.into_owned();
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(dest_path)?;
        file.write_all(&data)?;
    }
    Ok(())
}
