use serde::Deserialize;

use super::{ajax::ResponseBody, Language};

pub type GetUserDataResponse = ResponseBody<UserData>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct UserData {
	pub user: User,
	pub country: String,
	pub offer_name: String,

	#[serde(rename(deserialize = "checkForm"))]
	pub api_token: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct User {
	#[serde(rename(deserialize = "USER_ID"))]
	pub id: u64,

	#[serde(rename(deserialize = "BLOG_NAME"))]
	pub name: String,

	pub email: String,
	pub options: UserOptions,
	pub setting: UserSetting,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserOptions {
	#[serde(rename(deserialize = "web_sound_quality"))]
	pub sound_quality: SoundQuality,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct SoundQuality {
	pub low: bool,
	pub standard: bool,
	pub high: bool,
	pub lossless: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct UserSetting {
	pub global: UserSettingGlobal,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct UserSettingGlobal {
	#[serde(with = "crate::serde::language_code")]
	pub language: Language,
}
