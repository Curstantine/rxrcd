use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
	errors::{CommandResult, PassiveError},
	models::{
		configuration::AuthConfiguration,
		state::DeezerClientState,
		user::{User, UserAuthState, UserAuthType},
	},
	utils::{configuration, directories},
};

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(app_handle), err(Debug))]
pub async fn get_auth_state<R: Runtime>(app_handle: AppHandle<R>) -> CommandResult<UserAuthState> {
	let is_logged_in = {
		let deezer_state = app_handle.state::<DeezerClientState>();
		let deezer_guard = deezer_state.get().await;
		let client = deezer_guard.as_ref().unwrap();

		client.is_authenticated()
	};

	let auth_config_path = directories::resolved::get_auth_config_path(app_handle.path_resolver());
	match configuration::read_auth_config(&auth_config_path).await? {
		Some(conf) if is_logged_in => Ok(UserAuthState::LoggedIn(conf.inner)),
		Some(conf) => Ok(UserAuthState::Incomplete(conf.inner)),
		_ => Ok(UserAuthState::NotLoggedIn),
	}
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
			UserAuthType::Arl { arl } => deezer::user::login_with_arl(client, arl).await.map(User::from)?,
			_ => todo!("Credential login is not implemented in the deezer crate yet"),
		}
	};

	let auth_config_path = directories::resolved::get_auth_config_path(app_handle.path_resolver());
	configuration::write_auth_config(&auth_config_path, AuthConfiguration::new(data)).await?;

	Ok(login)
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(app_handle), err(Debug))]
pub async fn logout<R: Runtime>(app_handle: AppHandle<R>) -> CommandResult<()> {
	{
		let deezer_state = app_handle.state::<DeezerClientState>();
		let deezer_guard = deezer_state.get().await;
		let client = deezer_guard.as_ref().unwrap();
		deezer::user::logout(client).map_err(PassiveError::from)?;
	}

	let auth_config_path = directories::resolved::get_auth_config_path(app_handle.path_resolver());
	configuration::remove_auth_config(&auth_config_path).await?;

	Ok(())
}
