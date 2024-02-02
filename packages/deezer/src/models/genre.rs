use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
	pub id: u32,
	pub name: String,
}
