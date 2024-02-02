pub struct Search {}

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
