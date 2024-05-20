use deezer::models::user::GetUserDataResponse;
use serde::Serialize;

use super::configuration::DownloadQuality;

#[derive(Debug, Serialize)]
pub struct UserData {
	pub id: u64,
	pub name: String,
	pub email: String,
	pub country: String,
	pub offer_name: String,

	pub sound_quality: Vec<DownloadQuality>,
}

impl From<GetUserDataResponse> for UserData {
	fn from(value: GetUserDataResponse) -> Self {
		let mut sound_quality = Vec::with_capacity(4);

		if value.results.user.options.sound_quality.low || value.results.user.options.sound_quality.standard {
			sound_quality.push(DownloadQuality::Mp3_128);
		}

		if value.results.user.options.sound_quality.high {
			sound_quality.push(DownloadQuality::Mp3_320);
		}

		if value.results.user.options.sound_quality.lossless {
			sound_quality.push(DownloadQuality::Flac);
		}

		Self {
			id: value.results.user.id,
			name: value.results.user.name,
			email: value.results.user.email,
			country: value.results.country,
			offer_name: value.results.offer_name,
			sound_quality,
		}
	}
}
