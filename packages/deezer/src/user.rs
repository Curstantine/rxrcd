use crate::client::DeezerClient;

pub async fn login_with_arl(client: &DeezerClient) {
	let lock = client.cookie_store.lock().unwrap();
}
