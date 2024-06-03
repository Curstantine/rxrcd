use std::{
	fmt::Display,
	sync::{Arc, Mutex, MutexGuard},
};

use reqwest::{
	header::{self, HeaderMap},
	Client,
};
use reqwest_cookie_store::CookieStoreMutex;
use tracing::debug;
use url::Url;

use crate::{
	constants::{DEEZER_URL, PACKAGE_VERSION},
	models::{user::UserData, Language},
};

#[derive(Debug)]
pub struct DeezerClient {
	pub(crate) client: Client,
	pub(crate) cookie_store: Arc<CookieStoreMutex>,
	pub(crate) language: Mutex<Option<Language>>,
	pub(crate) user_data: Mutex<Option<UserData>>,
}

impl DeezerClient {
	pub fn new(client_name: &'static str, client_version: &str) -> Self {
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
			language: Mutex::default(),
			user_data: Mutex::default(),
		}
	}

	pub fn is_authenticated(&self) -> bool {
		let data = self.user_data.lock().unwrap();
		data.is_some()
	}

	pub fn get_user_data(&self) -> MutexGuard<'_, Option<UserData>> {
		self.user_data.lock().unwrap()
	}

	pub fn set_user_data(&self, user_data: Option<UserData>) {
		let mut data = self.user_data.lock().unwrap();
		*data = user_data;
	}

	pub fn set_language(&self, language: Option<Language>) {
		let mut data = self.language.lock().unwrap();
		*data = language;
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
		self.client.get(url).header(
			header::ACCEPT_LANGUAGE,
			self.language
				.lock()
				.unwrap()
				.map_or_else(|| Language::English.to_header_value(), |e| e.to_header_value()),
		)
	}

	pub fn post<U: reqwest::IntoUrl + Display>(&self, url: U) -> reqwest::RequestBuilder {
		debug!("POST request sent to {url}");
		self.client.post(url).header(
			header::ACCEPT_LANGUAGE,
			self.language
				.lock()
				.unwrap()
				.map_or_else(|| Language::English.to_header_value(), |e| e.to_header_value()),
		)
	}

	#[cfg(test)]
	pub fn testing() -> Self {
		DeezerClient::new("rxrcd-deezer", "testing")
	}
}
