use {
	tauri::{AppHandle, Manager, Runtime},
	tracing::{debug, info},
};

use crate::{
	errors::CommandResult,
	models::state::{AppState, ConfigurationState, NetworkClientState},
};

pub mod album;
pub mod artist;
pub mod config;

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn setup<R: Runtime>(handle: AppHandle<R>) -> CommandResult<()> {
	let app_state = handle.state::<AppState>();
	let path_resolver = handle.path_resolver();
	let config_state = handle.state::<ConfigurationState>();
	let network_state = handle.state::<NetworkClientState>();

	if app_state.initialize().is_none() {
		debug!("AppState::initialize hook reran while the app is initialized. Ignoring...");
		return Ok(());
	}

	network_state.initialize().await?;

	let app_config_dir = path_resolver
		.app_config_dir()
		.expect("Couldn't read the app_config_dir");

	config_state.initialize(&app_config_dir).await?;

	info!("Setup hook completed successfully!");

	Ok(())
}
