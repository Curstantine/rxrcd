use reqwest::Client;

use crate::{
	constants::DEEZER_API_URL,
	errors::Result,
	models::{
		album::{Album, AlbumSearch},
		search::SearchOptions,
	},
};

pub async fn get_album(client: &Client, id: u64) -> Result<Album> {
	let url = format!("{DEEZER_API_URL}/album/{id}");
	let req = client.get(url).send().await?;
	let body = req.json::<Album>().await?;

	Ok(body)
}

pub async fn search_albums(client: &Client, options: SearchOptions<'_>) -> Result<AlbumSearch> {
	let url = options.make_url("search/album");
	let req = client.get(url).send().await?;
	let body = req.json::<AlbumSearch>().await?;

	Ok(body)
}

#[cfg(test)]
mod tests {
	use reqwest::Client;

	use crate::{
		errors::Result,
		models::{
			album::{Album, AlbumSearch},
			search::SearchOptions,
		},
	};

	#[test]
	fn test_deserialize_album() {
		let file = std::fs::read_to_string("./samples/album.json").unwrap();
		let album: Album = serde_json::from_str(&file).expect("Failed to deserialize JSON");

		println!("{album:#?}");
	}

	#[test]
	fn test_deserialize_albums_search() {
		let file = std::fs::read_to_string("./samples/search-albums.json").unwrap();
		let search: AlbumSearch = serde_json::from_str(&file).expect("Failed to deserialize JSON");

		println!("{search:#?}");
	}

	#[tokio::test]
	async fn test_get_album() -> Result<()> {
		let client = Client::default();
		let out = super::get_album(&client, 302127).await?;
		println!("{out:#?}");

		Ok(())
	}

	#[tokio::test]
	async fn test_search_albums() -> Result<()> {
		let client = Client::default();
		let out = super::search_albums(&client, SearchOptions::new("Draft Punk", None, None)).await?;
		println!("{out:#?}");

		Ok(())
	}
}
