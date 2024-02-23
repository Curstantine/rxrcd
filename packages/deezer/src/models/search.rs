use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::constants::DEEZER_API_URL;
use crate::errors::DeezerResult;
use crate::models::{album::Album, artist::Artist, generic::DeezerPaginatedList, track::Track};

pub type Search = DeezerPaginatedList<SearchData>;

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptions<'a> {
	query: Option<&'a str>,
	order: Option<SearchOrder>,
	strict: Option<bool>,
	limit: Option<u32>,
	index: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
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
	pub fn new(order: Option<SearchOrder>, limit: Option<u32>, index: Option<u32>) -> Self {
		Self {
			order,
			limit,
			index,
			query: None,
			strict: None,
		}
	}

	pub fn with_query(query: &'a str, order: Option<SearchOrder>, limit: Option<u32>, index: Option<u32>) -> Self {
		Self {
			query: Some(query),
			order,
			limit,
			index,
			strict: None,
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

	pub fn create_url(&self, path: &'_ str) -> DeezerResult<Url> {
		let mut params: Vec<(&'static str, Cow<'_, str>)> = vec![];

		if let Some(q) = self.query {
			params.push(("q", Cow::Borrowed(q)));
		}

		if let Some(ord) = self.order {
			params.push(("order", Cow::Borrowed(ord.as_api_value())))
		}

		if self.strict.unwrap_or(false) {
			params.push(("strict", Cow::Borrowed("on")))
		}

		if let Some(limit) = &self.limit {
			params.push(("limit", Cow::Owned(limit.to_string())))
		}

		if let Some(index) = &self.index {
			params.push(("index", Cow::Owned(index.to_string())))
		}

		let url = Url::parse_with_params(&format!("{DEEZER_API_URL}/{path}"), params)?;
		Ok(url)
	}
}

impl SearchOrder {
	pub fn as_api_value(&self) -> &'static str {
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
