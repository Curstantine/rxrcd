use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::directories;

use super::user::UserAuthType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
	pub download: ConfigurationDownload,
	pub appearance: ConfigurationAppearance,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum DownloadQuality {
	Flac,
	Mp3_320,
	Mp3_128,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum CoverQuality {
	Small,
	Medium,
	Big,
	Xl,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigurationDownload {
	pub concurrent: u8,
	pub quality: DownloadQuality,
	pub path: PathBuf,

	pub save_covers: bool,
	pub embed_covers: bool,
	pub cover_resolution: CoverQuality,
	pub cover_embed_resolution: CoverQuality,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigurationAppearance {
	pub theme: String,
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			download: ConfigurationDownload {
				concurrent: 3,
				path: directories::get_default_download_dir(),
				quality: DownloadQuality::Flac,

				save_covers: true,
				embed_covers: true,
				cover_resolution: CoverQuality::Xl,
				cover_embed_resolution: CoverQuality::Medium,
			},
			appearance: ConfigurationAppearance {
				theme: "system".to_string(),
			},
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfiguration {
	#[serde(flatten)]
	pub inner: UserAuthType,
}

impl AuthConfiguration {
	pub fn new(inner: UserAuthType) -> Self {
		Self { inner }
	}
}
