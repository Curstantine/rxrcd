use std::{
	io::ErrorKind,
	path::{Path, PathBuf},
};

use {
	anyhow::Result,
	serde::{Deserialize, Serialize},
};

use super::directories;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
	pub download: ConfigurationDownload,
	pub covers: ConfigurationCovers,
	pub appearance: ConfigurationAppearance,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum DownloadQuality {
	Flac,
	Mp3_320,
	Mp3_128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigurationDownload {
	pub concurrent: u8,
	pub path: PathBuf,
	pub quality: DownloadQuality,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigurationCovers {
	pub save_covers: bool,
	pub embed_covers: bool,
	pub cover_resolution: u16,
	pub cover_embed_resolution: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigurationAppearance {
	pub theme: String,
}

impl Configuration {
	pub async fn initialize(config_dir: &Path) -> Result<Self> {
		let config_path = directories::get_config_path(config_dir);

		let config = match tokio::fs::read_to_string(&config_path).await {
			Ok(string) => toml::from_str::<Configuration>(&string)?,
			Err(e) if e.kind() == ErrorKind::NotFound => {
				let config = Configuration::default();
				let config_str = toml::to_string_pretty(&config)?;

				tokio::fs::write(&config_path, config_str).await?;

				config
			}
			Err(e) => return Err(e.into()),
		};

		Ok(config)
	}
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			download: ConfigurationDownload {
				concurrent: 3,
				path: PathBuf::from("~/Downloads/rxrcd/"),
				quality: DownloadQuality::Mp3_128,
			},
			covers: ConfigurationCovers {
				save_covers: true,
				embed_covers: true,
				cover_resolution: 1200,
				cover_embed_resolution: 1000,
			},
			appearance: ConfigurationAppearance {
				theme: "system".to_string(),
			},
		}
	}
}
