use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Genre {
	pub id: u32,
	pub name: String,
}
