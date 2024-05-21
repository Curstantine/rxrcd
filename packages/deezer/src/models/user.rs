use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetUserDataResponse {
	pub results: UserData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct UserData {
	pub user: User,
	pub country: String,
	pub offer_name: String,

	#[serde(rename(deserialize = "checkForm"))]
	pub api_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
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
