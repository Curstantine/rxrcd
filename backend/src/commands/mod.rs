use std::sync::atomic;

use {
	tauri::{AppHandle, Manager, Runtime},
	tracing::{debug, info},
};

use crate::{
	constants,
	errors::CommandResult,
	models::{
		commands::SetupReturnFlags,
		state::{AppState, ConfigurationState, DeezerClientState},
		user::UserAuthType,
	},
	utils::{configuration, directories},
};

pub mod album;
pub mod artist;
pub mod config;
pub mod user;

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn setup<R: Runtime>(handle: AppHandle<R>) -> CommandResult<SetupReturnFlags> {
	let app_state = handle.state::<AppState>();
	let path_resolver = handle.path_resolver();
	let config_state = handle.state::<ConfigurationState>();
	let deezer_state = handle.state::<DeezerClientState>();

	let mut flags = SetupReturnFlags::default();

	if app_state.initialized.load(atomic::Ordering::Relaxed) {
		flags.is_re_run = true;

		// Note(Curstantine): Checks whether auth needs to be resumed in a HMR re-mounted state.
		// We handle whether to actually call deezer or use the local-storage cache in the frontend.
		let deezer_lock = deezer_state.get().await;
		let deezer = deezer_lock.as_ref().unwrap();
		flags.resume_auth = deezer.cookie_has_arl();

		debug!("Setup hook reran while the app was initialized. Ignored with flags: {flags:?}");
		return Ok(flags);
	} else {
		app_state.initialized.store(true, atomic::Ordering::Relaxed);
	}

	let app_config_dir = path_resolver
		.app_config_dir()
		.expect(constants::ERR_MSG_NO_APP_CONFIG_DIR);
	config_state.initialize(&app_config_dir).await?;
	deezer_state.initialize().await?;

	let auth_config_path = directories::get_auth_config_path(&app_config_dir);
	if let Some(auth_config) = configuration::read_auth_config(&auth_config_path).await? {
		let deezer_lock = deezer_state.get().await;
		let deezer = deezer_lock.as_ref().unwrap();

		match &auth_config.inner {
			UserAuthType::Arl { arl } => {
				deezer.cookie_set_arl(arl);
				flags.resume_auth = true;
			}
			_ => todo!("Credential auth is not supported yet"),
		}
	}

	info!("Setup hook completed successfully with flags: {flags:?}");

	Ok(flags)
}
