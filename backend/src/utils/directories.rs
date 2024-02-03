use std::path::{Path, PathBuf};

use tauri::PathResolver;

#[derive(Debug)]
pub struct Directories {
	pub data_dir: PathBuf,
	pub config_dir: PathBuf,
}

impl Directories {
	pub fn initialize(path_resolver: &PathResolver) -> Self {
		Self {
			data_dir: path_resolver.app_data_dir().expect("Couldn't read the app_data_dir"),
			config_dir: path_resolver
				.app_config_dir()
				.expect("Couldn't read the app_config_dir"),
		}
	}
}

#[inline(always)]
pub fn get_config_path(config_dir: &Path) -> PathBuf {
	config_dir.join("config.toml")
}
