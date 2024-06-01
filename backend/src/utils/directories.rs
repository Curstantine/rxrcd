use std::path::{Path, PathBuf};

use tauri::api::path::download_dir;

#[inline(always)]
pub fn get_config_path(config_dir: &Path) -> PathBuf {
	config_dir.join("config.toml")
}
#[inline(always)]
pub fn get_auth_config_path(config_dir: &Path) -> PathBuf {
	config_dir.join("auth.toml")
}

#[inline(always)]
pub fn get_default_download_dir() -> PathBuf {
	download_dir().unwrap().join("rxrcd")
}

/// re-implementations of [directories] using a [tauri::PathResolver] for convenience.
pub mod resolved {
	use std::path::PathBuf;

	use tauri::PathResolver;

	use crate::constants;

	#[inline]
	pub fn get_auth_config_path(path_resolver: PathResolver) -> PathBuf {
		let app_config_dir = path_resolver
			.app_config_dir()
			.expect(constants::ERR_MSG_NO_APP_CONFIG_DIR);
		super::get_auth_config_path(&app_config_dir)
	}
}
