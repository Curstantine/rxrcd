use serde::{Deserialize, Serialize};

use super::{
	artist::AlbumRelArtist,
	generic::{DeezerList, DeezerPaginatedList},
	genre::Genre,
	track::Track,
};

pub type AlbumSearch = DeezerPaginatedList<SearchRelAlbum>;
pub type ArtistAlbumList = DeezerPaginatedList<ArtistRelAlbum>;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AlbumRecordType {
	Album,
	Ep,
	Single,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
	pub id: u32,
	pub title: String,
	pub link: String,
	pub cover_small: String,
	pub cover_medium: String,
	pub cover_big: String,
	pub artist: AlbumRelArtist,
	pub genres: DeezerList<Genre>,
	pub tracks: DeezerList<Track>,
	pub release_date: String,
	pub record_type: AlbumRecordType,
	pub explicit_lyrics: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRelAlbum {
	pub id: u32,
	pub title: String,
	pub link: String,
	pub cover_small: String,
	pub cover_medium: String,
	pub cover_big: String,
	pub artist: AlbumRelArtist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRelAlbum {
	pub id: u32,
	pub title: String,
	pub link: String,
	pub cover_small: String,
	pub cover_medium: String,
	pub cover_big: String,
	pub release_date: String,
	pub record_type: AlbumRecordType,
	pub explicit_lyrics: bool,
}
