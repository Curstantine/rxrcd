use serde::Deserialize;

use super::{ajax::ResponseBody, Language};

pub type GetUserDataResponse = ResponseBody<UserData>;

pub trait UserVariant {
	fn is_anon() -> bool;
	fn api_token(&self) -> String;
}

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
	#[serde(rename(deserialize = "USER_ID"), deserialize_with = "crate::serde::de_user_id")]
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

impl UserVariant for UserData {
	fn is_anon() -> bool {
		false
	}

	// TODO(Curstantine):
	// We definitely can wrap the str in a Box, Rc or and Arc to prevent the unnecessary clones.
	// Probably a good idea to look into this in the future when we are doing optimizations.
	fn api_token(&self) -> String {
		self.api_token.clone()
	}
}

pub mod anon {
	use serde::Deserialize;

	#[derive(Debug, Clone, Deserialize)]
	#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
	pub struct AnonUserData {
		pub country: String,

		#[serde(rename(deserialize = "checkForm"))]
		pub api_token: String,
	}

	impl super::UserVariant for AnonUserData {
		fn is_anon() -> bool {
			true
		}

		fn api_token(&self) -> String {
			self.api_token.clone()
		}
	}
}
