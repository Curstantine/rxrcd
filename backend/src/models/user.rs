use deezer::models::user::GetUserDataResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserData {
	pub id: u64,
	pub name: String,
	pub email: String,
	pub is_premium: bool,
}

impl From<GetUserDataResponse> for UserData {
	fn from(value: GetUserDataResponse) -> Self {
		Self {
			id: value.results.user.id,
			name: value.results.user.name,
			email: value.results.user.email,
			is_premium: value.results.is_premium,
		}
	}
}
