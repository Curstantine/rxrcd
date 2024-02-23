use reqwest::Client;

use crate::{
	constants::DEEZER_API_URL,
	errors::DeezerResult,
	models::{
		artist::{Artist, ArtistSearch},
		search::SearchOptions,
	},
};

pub async fn get_artist(client: &Client, id: u64) -> DeezerResult<Artist> {
	let url = format!("{DEEZER_API_URL}/artist/{id}");
	let req = client.get(url).send().await?;
	let body = req.json::<Artist>().await?;

	Ok(body)
}

pub async fn search_artists(client: &Client, options: &SearchOptions<'_>) -> DeezerResult<ArtistSearch> {
	let url = options.create_url("search/artist")?;
	let req = client.get(url).send().await?;
	let body = req.json::<ArtistSearch>().await?;

	Ok(body)
}

#[cfg(test)]
mod tests {
	use reqwest::Client;

	use crate::{
		errors::DeezerResult,
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
	async fn test_get_artist() -> DeezerResult<()> {
		let client = Client::default();
		let out = super::get_artist(&client, 302127).await?;
		println!("{out:#?}");

		Ok(())
	}

	#[tokio::test]
	async fn test_search_artists() -> DeezerResult<()> {
		let client = Client::default();
		let options = SearchOptions::with_query("A", None, None, None);

		let out = super::search_artists(&client, &options).await?;
		let mut index = out.total.clamp(0, 25);

		println!("First fetch: {out:#?}");

		// We don't need to read through the whole tree. Checking if pagination actually works is enough.
		if out.total > index {
			let fetch = super::search_artists(&client, &options.with_index(index)).await?;
			println!("Second paginated test: {index} : {fetch:#?}");

			index += fetch.data.len() as u32;
		}

		println!("Index: {index},  total: {}", out.total);

		Ok(())
	}
}
