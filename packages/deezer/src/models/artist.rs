use serde::{Deserialize, Serialize};

use super::generic::DeezerPaginatedList;

pub type ArtistSearch = DeezerPaginatedList<Artist>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
	pub id: u32,
	pub name: String,
	pub tracklist: String,
	pub picture_small: Option<String>,
	pub picture_big: Option<String>,
	pub nb_album: u32,
	pub nb_fan: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRelArtist {
	pub id: u32,
	pub name: String,
	pub picture_small: Option<String>,
	pub picture_big: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackRelArtist {
	pub id: u32,
	pub name: String,
}
