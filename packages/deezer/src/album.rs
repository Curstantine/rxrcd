use reqwest::Client;

use crate::{constants::DEEZER_API_URL, errors::Error, models::album::Album};

pub async fn get_album(client: &Client, id: u64) -> Result<Album, Error> {
	let url = format!("{DEEZER_API_URL}/album/{id}");
	let reqw = client.get(url).send().await?;
	let body = reqw.json::<Album>().await?;

	Ok(body)
}

#[cfg(test)]
mod tests {
	use crate::models::album::Album;

	use super::get_album;

	#[test]
	fn test_deserialize_album() {
		let file = std::fs::read_to_string("./samples/album.json").unwrap();
		let album: Album = serde_json::from_str(&file).expect("Failed to deserialize JSON");

		println!("{album:#?}");
	}

	#[tokio::test]
	async fn test_get_album() {
		let client = reqwest::Client::default();
		let out = get_album(&client, 302127).await;
		println!("{out:#?}");
	}
}
