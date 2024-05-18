use std::{
	fmt::Display,
	sync::{Arc, Mutex},
};

use reqwest::{
	header::{self, HeaderMap},
	Client,
};
use reqwest_cookie_store::CookieStoreMutex;
use tracing::debug;
use url::Url;

use crate::constants::{DEEZER_URL, PACKAGE_VERSION};

#[derive(Debug)]
pub struct DeezerClient {
	pub(crate) client: Client,
	pub(crate) cookie_store: Arc<CookieStoreMutex>,
	pub(crate) api_token: Mutex<Option<String>>,
}

impl DeezerClient {
	pub fn with_client_name(client_name: &'static str, client_version: &str) -> Self {
		let mut headers = HeaderMap::new();
		headers.insert(
			header::USER_AGENT,
			format!("{client_name} / {client_version} (Using deezer@{PACKAGE_VERSION})")
				.parse()
				.unwrap(),
		);

		let cookie_store = CookieStoreMutex::default();
		let cookie_store = Arc::new(cookie_store);

		let client = Client::builder()
			.default_headers(headers)
			.cookie_provider(Arc::clone(&cookie_store))
			.build()
			.unwrap();

		Self {
			client,
			cookie_store,
			api_token: Mutex::default(),
		}
	}

	pub fn is_authenticated(&self) -> bool {
		let data = self.api_token.lock().unwrap();
		data.is_some()
	}

	pub fn set_api_token(&self, api_token: String) {
		let mut data = self.api_token.lock().unwrap();
		*data = Some(api_token);
	}

	pub fn cookie_has_arl(&self) -> bool {
		let store = self.cookie_store.lock().unwrap();
		store.contains("deezer.com", "/", "arl")
	}

	pub fn cookie_set_arl(&self, arl: &str) {
		let cookie_str = format!("arl={arl}; Domain=.deezer.com");
		let mut store = self.cookie_store.lock().unwrap();
		store.parse(&cookie_str, &Url::parse(DEEZER_URL).unwrap()).unwrap();
	}

	pub fn clear_cookies(&self) {
		let mut store = self.cookie_store.lock().unwrap();
		store.clear()
	}

	pub fn get<U: reqwest::IntoUrl + Display>(&self, url: U) -> reqwest::RequestBuilder {
		debug!("GET request sent to {url}");
		self.client.get(url)
	}

	pub fn post<U: reqwest::IntoUrl + Display>(&self, url: U) -> reqwest::RequestBuilder {
		debug!("POST request sent to {url}");
		self.client.post(url)
	}

	#[cfg(test)]
	pub fn testing() -> Self {
		DeezerClient::with_client_name("rxrcd-deezer", "testing")
	}
}
