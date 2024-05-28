use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
	constants,
	errors::{CommandResult, PassiveError},
	models::{
		configuration::AuthConfiguration,
		state::DeezerClientState,
		user::{User, UserAuthType},
	},
	utils::{configuration, directories},
};

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn get_auth_state(deezer_state: State<'_, DeezerClientState>) -> CommandResult<UserAuthType> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	todo!()
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn refresh_login(deezer_state: State<'_, DeezerClientState>) -> CommandResult<User> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();
	Ok(deezer::user::refresh_login(client).await.map(User::from)?)
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(app_handle), err(Debug))]
pub async fn login<R: Runtime>(app_handle: AppHandle<R>, data: UserAuthType) -> CommandResult<User> {
	let login = {
		let deezer_state = app_handle.state::<DeezerClientState>();
		let deezer_guard = deezer_state.get().await;
		let client = deezer_guard.as_ref().unwrap();

		match &data {
			UserAuthType::ARL { arl } => deezer::user::login_with_arl(client, &arl).await.map(User::from)?,
			_ => todo!("Credential login is not implemented in the deezer crate yet"),
		}
	};

	let app_config_dir = app_handle
		.path_resolver()
		.app_config_dir()
		.expect(constants::ERR_MSG_NO_APP_CONFIG_DIR);

	let auth_config_path = directories::get_auth_path(&app_config_dir);
	configuration::write_auth_config(&auth_config_path, AuthConfiguration::new(data)).await?;

	Ok(login)
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn logout(deezer_state: State<'_, DeezerClientState>) -> CommandResult<()> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	deezer::user::logout(client).map_err(PassiveError::from)
}
