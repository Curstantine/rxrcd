use crate::{
	client::DeezerClient,
	constants::DEEZER_AJAX_URL,
	errors::DeezerResult,
	models::{
		ajax::{RequestPOSTBody, RequestPOSTMethod},
		user::GetUserDataResponse,
	},
};

/// Refreshes a login with the ARL stored in the cookies and sets the fresh api_token to the [DeezerClient]
pub async fn refresh_login(client: &DeezerClient) -> DeezerResult<GetUserDataResponse> {
	let body = RequestPOSTBody::with_defaults(RequestPOSTMethod::GetUserData);
	let response = client.post(DEEZER_AJAX_URL).json(&body).send().await?;
	let data = response.json::<GetUserDataResponse>().await?;
	client.set_api_token(data.results.api_token.clone());

	Ok(data)
}

/// Logins with a provided ARL and sets the api_token to the [DeezerClient].
pub async fn login_with_arl(client: &DeezerClient, arl: &str) -> DeezerResult<GetUserDataResponse> {
	client.cookie_set_arl(arl);
	refresh_login(client).await
}

pub async fn login_with_credentials(client: &DeezerClient, username: &str, password: &str) -> DeezerResult<()> {
	todo!("Add login with credentials")
}

#[cfg(test)]
mod tests {
	use std::env::var;

	use crate::{client::DeezerClient, errors::DeezerResult};

	#[tokio::test]
	async fn test_refresh_login() -> DeezerResult<()> {
		let arl = var("ARL").expect("ARL env needs to be set for this test to run!");
		let client = DeezerClient::testing();
		client.cookie_set_arl(&arl);

		let data = super::refresh_login(&client).await?;
		assert_ne!(data.results.user.id, 0);

		Ok(())
	}

	#[tokio::test]
	async fn test_login_with_arl() -> DeezerResult<()> {
		let arl = var("ARL").expect("ARL env needs to be set for this test to run!");
		let client = DeezerClient::testing();

		let data = super::login_with_arl(&client, &arl).await?;
		assert_ne!(data.results.user.id, 0);

		Ok(())
	}
}
