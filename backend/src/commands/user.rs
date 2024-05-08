use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
	constants,
	errors::CommandResult,
	models::{configuration::AuthConfiguration, state::DeezerClientState, user::UserData},
	utils::{configuration, directories},
};

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn refresh_login(deezer_state: State<'_, DeezerClientState>) -> CommandResult<UserData> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();
	Ok(deezer::user::refresh_login(client).await.map(UserData::from)?)
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(app_handle), err(Debug))]
pub async fn login_with_arl<R: Runtime>(app_handle: AppHandle<R>, arl: String) -> CommandResult<UserData> {
	let login = {
		let deezer_state = app_handle.state::<DeezerClientState>();
		let deezer_guard = deezer_state.get().await;
		let client = deezer_guard.as_ref().unwrap();
		deezer::user::login_with_arl(client, &arl).await.map(UserData::from)?
	};

	let app_config_dir = app_handle
		.path_resolver()
		.app_config_dir()
		.expect(constants::ERR_MSG_NO_APP_CONFIG_DIR);

	let auth_config_path = directories::get_auth_path(&app_config_dir);
	configuration::write_auth_config(&auth_config_path, AuthConfiguration::new(arl)).await?;

	Ok(login)
}
