use std::{fmt::Display, sync::Arc};

use reqwest::{
	header::{self, HeaderMap, HeaderValue},
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
	pub(crate) language: Option<Language>,
	pub user_data: Option<UserData>,
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
			language: Option::default(),
			user_data: Option::default(),
		}
	}

	pub fn is_authenticated(&self) -> bool {
		self.user_data.is_some()
	}

	pub fn set_user_data(&mut self, user_data: Option<UserData>) {
		self.user_data = user_data;
	}

	pub fn set_language(&mut self, language: Option<Language>) {
		self.language = language;
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
		let request = self.client.get(url);
		let mut headers = HeaderMap::<HeaderValue>::with_capacity(1);

		if let Some(lang) = &self.language {
			headers.append(header::ACCEPT_LANGUAGE, lang.to_header_value());
		}

		request.headers(headers)
	}

	pub fn post<U: reqwest::IntoUrl + Display>(&self, url: U) -> reqwest::RequestBuilder {
		debug!("POST request sent to {url}");
		let request = self.client.post(url);
		let mut headers = HeaderMap::<HeaderValue>::with_capacity(1);

		if let Some(lang) = &self.language {
			headers.append(header::ACCEPT_LANGUAGE, lang.to_header_value());
		}

		request.headers(headers)
	}

	#[cfg(test)]
	pub fn testing() -> Self {
		DeezerClient::new("rxrcd-deezer", "testing")
	}
}
