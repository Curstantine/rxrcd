use reqwest::Client;

use crate::{constants::DEEZER_API_URL, errors::Result, models::search::SearchOrder};

/// Deezer documentation: https://developers.deezer.com/api/search
pub async fn search(client: &Client, query: &str, order: Option<SearchOrder>, strict: Option<bool>) -> Result<()> {
	let mut url = format!("{DEEZER_API_URL}/search?q={query}");

	if let Some(ord) = order {
		url.push_str(&format!("&order={}", ord.as_api_value()));
	}

	if strict.unwrap_or(false) {
		url.push_str("&strict=on");
	}

	let req = client.get(url).send().await?;
	let body = req.text().await?;

	println!("{body}");
	unimplemented!()
}

#[cfg(test)]
mod test {
	use reqwest::Client;

	use crate::models::search::SearchOrder;

	#[tokio::test]
	pub async fn test_search() {
		let client = Client::default();
		let query = super::search(&client, "Draft Punk", Some(SearchOrder::ArtistDesc), Some(true)).await;

		println!("{query:#?}");
	}
}
