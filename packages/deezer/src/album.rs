use crate::{
	client::DeezerClient,
	constants::DEEZER_API_URL,
	errors::DeezerResult,
	models::{
		album::{Album, AlbumSearch, ArtistAlbumList},
		search::SearchOptions,
	},
};

pub async fn get_album(client: &DeezerClient, id: u64) -> DeezerResult<Album> {
	let url = format!("{DEEZER_API_URL}/album/{id}");
	let req = client.get(url).send().await?;
	let body = req.json::<Album>().await?;

	Ok(body)
}

#[tracing::instrument(skip(client))]
pub async fn search_albums(client: &DeezerClient, options: &SearchOptions<'_>) -> DeezerResult<AlbumSearch> {
	let url = options.create_url("search/album")?;
	let req = client.get(url).send().await?;
	let body = req.json::<AlbumSearch>().await?;

	Ok(body)
}

pub async fn get_artist_albums(
	client: &DeezerClient,
	artist_id: u64,
	options: &SearchOptions<'_>,
) -> DeezerResult<ArtistAlbumList> {
	let url = options.create_url(&format!("artist/{artist_id}/albums"))?;
	let req = client.get(url).send().await?;
	let body = req.json::<ArtistAlbumList>().await?;

	Ok(body)
}

#[cfg(test)]
mod tests {
	use crate::{
		client::DeezerClient,
		errors::DeezerResult,
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
	async fn test_get_album() -> DeezerResult<()> {
		let client = DeezerClient::testing();
		let out = super::get_album(&client, 302127).await?;
		println!("{out:#?}");

		Ok(())
	}

	#[tokio::test]
	async fn test_search_albums() -> DeezerResult<()> {
		let client = DeezerClient::testing();
		let options = SearchOptions::with_query("Draft Punk", None, None, None);

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

	#[tokio::test]
	async fn test_get_artist_albums() -> DeezerResult<()> {
		let client = DeezerClient::testing();
		let options = SearchOptions::new(None, None, None);

		let out = super::get_artist_albums(&client, 27, &options).await?;
		let mut index = out.total.clamp(0, 25);
		println!("First fetch: {out:#?}");

		if out.total > index {
			let fetch = super::get_artist_albums(&client, 27, &options.with_index(index)).await?;
			println!("Second paginated test: {index} : {fetch:#?}");

			index += fetch.data.len() as u32;
		}

		println!("Index: {index},  total: {}", out.total);

		Ok(())
	}
}
