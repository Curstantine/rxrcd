use serde::Deserialize;

use crate::{
	constants::DEEZER_API_URL,
	models::{album::Album, artist::Artist, generic::DeezerPaginatedList, track::Track},
};

pub type Search = DeezerPaginatedList<SearchData>;

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptions<'a> {
	query: &'a str,
	order: Option<SearchOrder>,
	strict: Option<bool>,
	limit: Option<u32>,
	index: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SearchData {
	Track(Track),
	Artist(Artist),
	Album(Album),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchOrder {
	Ranking,
	TrackAsc,
	TrackDesc,
	ArtistAsc,
	ArtistDesc,
	AlbumAsc,
	AlbumDesc,
	RatingAsc,
	RatingDesc,
	DurationAsc,
	DurationDesc,
}

impl<'a> SearchOptions<'a> {
	pub fn new(query: &'a str, order: Option<SearchOrder>, strict: Option<bool>) -> Self {
		Self {
			query,
			order,
			strict,
			index: Some(0),
			limit: Some(25),
		}
	}

	pub fn with_limit(mut self, limit: u32) -> Self {
		self.limit = Some(limit);
		self
	}

	pub fn with_index(mut self, index: u32) -> Self {
		self.index = Some(index);
		self
	}

	pub fn create_url(&self, path: &'static str) -> String {
		let mut url = format!("{DEEZER_API_URL}/{path}?q={}", self.query);

		if let Some(ord) = self.order {
			url.push_str(&format!("&order={}", ord.as_api_value()));
		}

		if self.strict.unwrap_or(false) {
			url.push_str("&strict=on");
		}

		if let Some(limit) = self.limit {
			url.push_str(&format!("&limit={limit}"));
		}

		if let Some(index) = self.index {
			url.push_str(&format!("&index={index}"));
		}

		url
	}
}

impl SearchOrder {
	pub fn as_api_value(&self) -> &str {
		match self {
			SearchOrder::Ranking => "RANKING",
			SearchOrder::TrackAsc => "TRACK_ASC",
			SearchOrder::TrackDesc => "TRACK_DESC",
			SearchOrder::ArtistAsc => "ARTIST_ASC",
			SearchOrder::ArtistDesc => "ARTIST_DESC",
			SearchOrder::AlbumAsc => "ALBUM_ASC",
			SearchOrder::AlbumDesc => "ALBUM_DESC",
			SearchOrder::RatingAsc => "RATING_ASC",
			SearchOrder::RatingDesc => "RATING_DESC",
			SearchOrder::DurationAsc => "DURATION_ASC",
			SearchOrder::DurationDesc => "DURATION_DESC",
		}
	}
}
