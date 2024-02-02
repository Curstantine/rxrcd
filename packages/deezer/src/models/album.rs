use serde::{Deserialize, Serialize};

use super::{artist::AlbumRelArtist, generic::DeezerList, genre::Genre, track::Track};

#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
	pub id: u32,
	pub title: String,
	pub link: String,
	pub cover_small: String,
	pub cover_big: String,
	pub artist: AlbumRelArtist,
	pub genres: DeezerList<Genre>,
	pub tracks: DeezerList<Track>,
}
