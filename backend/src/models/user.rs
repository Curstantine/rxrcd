use serde::{Deserialize, Serialize};

use super::configuration::DownloadQuality;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserAuthType {
	Arl { arl: String },
	Credentials { email: String, password: String },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum UserAuthState {
	LoggedIn(UserAuthType),
	Incomplete(UserAuthType),
	NotLoggedIn,
}

#[derive(Debug, Serialize)]
pub struct User {
	pub id: u64,
	pub name: String,
	pub email: String,
	pub country: String,
	pub offer_name: String,

	pub sound_quality: Vec<DownloadQuality>,
}

impl From<deezer::models::user::UserData> for User {
	fn from(value: deezer::models::user::UserData) -> Self {
		let mut sound_quality = Vec::with_capacity(3);

		if value.user.options.sound_quality.low || value.user.options.sound_quality.standard {
			sound_quality.push(DownloadQuality::Mp3_128);
		}

		if value.user.options.sound_quality.high {
			sound_quality.push(DownloadQuality::Mp3_320);
		}

		if value.user.options.sound_quality.lossless {
			sound_quality.push(DownloadQuality::Flac);
		}

		sound_quality.shrink_to_fit();
		Self {
			id: value.user.id,
			name: value.user.name,
			email: value.user.email,
			country: value.country,
			offer_name: value.offer_name,
			sound_quality,
		}
	}
}
