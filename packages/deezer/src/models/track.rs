use serde::Deserialize;

use super::artist::TrackRelArtist;

#[derive(Debug, Deserialize)]
pub struct Track {
	pub id: u32,
	pub title: String,
	pub explicit_lyrics: bool,
	pub preview: String,
	pub artist: TrackRelArtist,
}
