use tauri::State;

use crate::{
	errors::CommandResult,
	state::{ConfigurationState, DirectoriesState},
	utils::configuration::ConfigurationAppearance,
};

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_get_appearance(
	config_state: State<'_, ConfigurationState>,
) -> CommandResult<ConfigurationAppearance> {
	let guard = config_state.get();
	let object = guard.as_ref().unwrap();

	Ok(object.appearance.clone())
}

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_reload(
	dir_state: State<'_, DirectoriesState>,
	config_state: State<'_, ConfigurationState>,
) -> CommandResult<()> {
	let config_dir = {
		let dir_guard = dir_state.get();
		let directories = dir_guard.as_ref().unwrap();
		directories.config_dir.clone()
	};

	config_state.reload(&config_dir).await?;

	Ok(())
}
