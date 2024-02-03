use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Artist {
	pub id: u32,
	pub name: String,
	pub tracklist: String,
	pub picture_small: String,
	pub picture_big: String,
	pub nb_album: u32,
	pub nb_fan: u64,
}

#[derive(Debug, Deserialize)]
pub struct AlbumRelArtist {
	pub id: u32,
	pub name: String,
	pub picture_small: String,
	pub picture_big: String,
}

#[derive(Debug, Deserialize)]
pub struct TrackRelArtist {
	pub id: u32,
	pub name: String,
}
