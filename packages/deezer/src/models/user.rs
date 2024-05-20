use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct GetUserDataResponse {
	pub results: UserData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub struct UserData {
	pub user: User,
	pub country: String,

	#[serde(rename(deserialize = "checkForm"))]
	pub api_token: String,

	#[serde(rename(deserialize = "OFFER_NAME"), deserialize_with = "map_plan_to_bool")]
	pub is_premium: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub struct User {
	#[serde(rename(deserialize = "USER_ID"))]
	pub id: u64,

	#[serde(rename(deserialize = "BLOG_NAME"))]
	pub name: String,

	pub email: String,
	pub options: UserOptions,
}

#[derive(Debug, Deserialize)]
pub struct UserOptions {
	#[serde(rename(deserialize = "web_sound_quality"))]
	pub sound_quality: SoundQuality,
}

#[derive(Debug, Deserialize)]
pub struct SoundQuality {
	pub low: bool,
	pub standard: bool,
	pub high: bool,
	pub lossless: bool,
}

fn map_plan_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
	let value = String::deserialize(deserializer)?;
	Ok(value == "Deezer Premium")
}
