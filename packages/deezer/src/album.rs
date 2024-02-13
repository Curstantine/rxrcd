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

pub async fn search_albums(client: &Client, options: &SearchOptions<'_>) -> Result<AlbumSearch> {
	let url = options.create_url("search/album");
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
		let options = SearchOptions::new("Draft Punk", None, None, None);

		let out = super::search_albums(&client, &options).await?;
		let mut index = out.total.clamp(0, 25);
		println!("First fetch: {out:#?}");

		if out.total > index {
			let fetch = super::search_albums(&client, &options.with_index(index)).await?;
			println!("Second paginated test: {index} : {fetch:#?}");

			index += fetch.data.len() as u32;
		}

		println!("Index: {index},  total: {}", out.total);

		Ok(())
	}
}
