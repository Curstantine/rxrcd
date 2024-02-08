use {
	tauri::{AppHandle, Manager, Runtime},
	tracing::{debug, info},
};

use crate::{
	errors::CommandResult,
	state::{AppState, ConfigurationState, DirectoriesState, NetworkClientState},
};

pub mod album;
pub mod config;

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn setup<R: Runtime>(handle: AppHandle<R>) -> CommandResult<()> {
	let app_state = handle.state::<AppState>();
	let path_resolver = handle.path_resolver();
	let dir_state = handle.state::<DirectoriesState>();
	let config_state = handle.state::<ConfigurationState>();
	let network_state = handle.state::<NetworkClientState>();

	if app_state.initialize().is_none() {
		debug!("AppState::initialize hook reran while the app is initialized. Ignoring...");
		return Ok(());
	}

	dir_state.initialize(&path_resolver);
	network_state.initialize()?;

	let config_dir = {
		let dir_guard = dir_state.get();
		let directories = dir_guard.as_ref().unwrap();
		directories.config_dir.clone()
	};
	config_state.initialize(&config_dir).await?;

	info!("Setup hook completed successfully!");

	Ok(())
}
