use serde::Deserialize;

use super::{
	artist::AlbumRelArtist,
	generic::{DeezerList, DeezerPaginatedList},
	genre::Genre,
	track::Track,
};

pub type AlbumSearch = DeezerPaginatedList<RelSearchAlbum>;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct RelSearchAlbum {
	pub id: u32,
	pub title: String,
	pub link: String,
	pub cover_small: String,
	pub cover_big: String,
	pub artist: AlbumRelArtist,
}
