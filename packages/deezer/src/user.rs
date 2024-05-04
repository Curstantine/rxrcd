use crate::{
	client::DeezerClient,
	constants::DEEZER_AJAX_URL,
	errors::DeezerResult,
	models::{
		ajax::{RequestPOSTBody, RequestPOSTMethod},
		user::GetUserDataResponse,
	},
};

/// Logins with a provided ARL and sets the api_token to the [DeezerClient].
pub async fn login_with_arl(client: &DeezerClient, arl: &str) -> DeezerResult<GetUserDataResponse> {
	client.cookie_set_arl(arl);

	let body = RequestPOSTBody::with_defaults(RequestPOSTMethod::GetUserData);
	let response = client.post(DEEZER_AJAX_URL).json(&body).send().await?;
	let data = response.json::<GetUserDataResponse>().await?;
	client.set_api_token(data.results.api_token.clone());

	Ok(data)
}

#[cfg(test)]
mod tests {
	use std::env::var;

	use crate::{client::DeezerClient, errors::DeezerResult};

	#[tokio::test]
	async fn test_login_with_arl() -> DeezerResult<()> {
		let client = DeezerClient::with_client_name("rxrcd-deezer", "testing");
		let arl = var("ARL").expect("ARL env needs to be set for this test to run!");
		super::login_with_arl(&client, &arl).await?;

		Ok(())
	}
}
