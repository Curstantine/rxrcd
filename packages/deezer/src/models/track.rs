use serde::{Deserialize, Serialize};

use super::artist::TrackRelArtist;

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
	pub id: u32,
	pub title: String,
	pub explicit_lyrics: bool,
	pub preview: String,
	pub artist: TrackRelArtist,
}
