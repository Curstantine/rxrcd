use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct GetUserDataResponse {
	pub results: UserData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
	#[serde(rename = "USER")]
	pub user: User,

	#[serde(rename = "checkForm")]
	pub api_token: String,

	#[serde(rename = "OFFER_NAME", deserialize_with = "map_plan_to_bool")]
	pub is_premium: bool,
}

#[derive(Debug, Deserialize)]
pub struct User {
	#[serde(rename = "USER_ID")]
	pub id: u64,

	#[serde(rename = "BLOG_NAME")]
	pub name: String,

	#[serde(rename = "EMAIL")]
	pub email: String,
}

fn map_plan_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
	let value = String::deserialize(deserializer)?;
	Ok(value == "Deezer Premium")
}
