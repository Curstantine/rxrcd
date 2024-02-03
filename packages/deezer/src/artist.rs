use reqwest::Client;

use crate::{
	constants::DEEZER_API_URL,
	errors::Result,
	models::{
		artist::{Artist, ArtistSearch},
		search::SearchOptions,
	},
};

pub async fn get_artist(client: &Client, id: u64) -> Result<Artist> {
	let url = format!("{DEEZER_API_URL}/artist/{id}");
	let req = client.get(url).send().await?;
	let body = req.json::<Artist>().await?;

	Ok(body)
}

pub async fn search_artists(client: &Client, options: SearchOptions<'_>) -> Result<ArtistSearch> {
	let url = options.make_url("search/artist");
	let req = client.get(url).send().await?;
	let body = req.json::<ArtistSearch>().await?;

	Ok(body)
}

#[cfg(test)]
mod tests {
	use reqwest::Client;

	use crate::{
		errors::Result,
		models::{
			artist::{Artist, ArtistSearch},
			search::SearchOptions,
		},
	};

	#[test]
	fn test_deserialize_artist() {
		let file = std::fs::read_to_string("./samples/artist.json").unwrap();
		let out: Artist = serde_json::from_str(&file).expect("Failed to deserialize JSON");

		println!("{out:#?}");
	}

	#[test]
	fn test_deserialize_artists_search() {
		let file = std::fs::read_to_string("./samples/search-artists.json").unwrap();
		let out: ArtistSearch = serde_json::from_str(&file).expect("Failed to deserialize JSON");

		println!("{out:#?}");
	}

	#[tokio::test]
	async fn test_get_artist() -> Result<()> {
		let client = Client::default();
		let out = super::get_artist(&client, 302127).await?;
		println!("{out:#?}");

		Ok(())
	}

	#[tokio::test]
	async fn test_search_artists() -> Result<()> {
		let client = Client::default();
		let out = super::search_artists(&client, SearchOptions::new("Draft Punk", None, None)).await?;
		println!("{out:#?}");

		Ok(())
	}
}
