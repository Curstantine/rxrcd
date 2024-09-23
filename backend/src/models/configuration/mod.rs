use std::path::PathBuf;

use data_language::DataLanguage;
use deezer::models::Language;
use serde::{Deserialize, Serialize};

use crate::utils::directories;

use super::user::UserAuthType;

pub mod data_language;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
	pub account: ConfigurationAccount,
	pub appearance: ConfigurationAppearance,
	pub download: ConfigurationDownload,
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
pub struct ConfigurationAccount {
	/// The language used in the Accept-Language header for deezer specific requests.
	pub data_language: DataLanguage,

	// --------------------------------------------------------------------------------------------------------
	// TODO(Curstantine):
	// I have no idea how to implement this, but I very much need this.
	// I did look around on the API, but there doesn't seem to be a way to get the "country" or the "language"
	// of an artist/album/song to guess which locale to pass to the Accept-Language header.
	// --------------------------------------------------------------------------------------------------------
	/// Whether to use the best-case locale for entries.
	///
	/// Such that, english entries use "en", japanese entries use "ja", and so forth.
	pub use_native_locale: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigurationAppearance {
	pub theme: String,
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

impl DataLanguage {
	pub fn into_deezer_language(self, account_language: Option<Language>) -> Language {
		match self {
			DataLanguage::Account => account_language.unwrap_or(Language::English),
			Self::Defined(lang) => lang,
		}
	}
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			account: ConfigurationAccount {
				data_language: DataLanguage::Account,
				use_native_locale: false,
			},
			appearance: ConfigurationAppearance {
				theme: "system".to_string(),
			},
			download: ConfigurationDownload {
				concurrent: 3,
				path: directories::get_default_download_dir(),
				quality: DownloadQuality::Flac,

				save_covers: true,
				embed_covers: true,
				cover_resolution: CoverQuality::Xl,
				cover_embed_resolution: CoverQuality::Medium,
			},
		}
	}
}
