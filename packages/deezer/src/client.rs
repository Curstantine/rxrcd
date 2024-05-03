use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

#[derive(Debug)]
pub struct DeezerClient {
	pub(crate) inner: Client,
	pub(crate) cookie_store: CookieStoreMutex,
}
