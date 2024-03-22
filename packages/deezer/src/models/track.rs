use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
	pub id: u32,
	pub title: String,
	pub explicit_lyrics: bool,
	pub preview: String,
	pub duration: u32,
}
