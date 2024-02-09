use serde::{Deserialize, Serialize};

use super::artist::TrackRelArtist;

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
	pub id: u32,
	pub title: String,
	pub explicit_lyrics: bool,
	pub preview: String,
	pub artist: TrackRelArtist,
}
