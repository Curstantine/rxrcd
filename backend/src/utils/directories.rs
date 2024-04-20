use std::path::{Path, PathBuf};

use tauri::api::path::download_dir;

#[inline(always)]
pub fn get_config_path(config_dir: &Path) -> PathBuf {
	config_dir.join("config.toml")
}

#[inline(always)]
pub fn get_default_download_dir() -> PathBuf {
	download_dir().unwrap().join("rxrcd/")
}
