use tauri::State;

use crate::{
	errors::CommandResult,
	models::{configuration::ConfigurationAppearance, state::ConfigurationState},
};

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_get_appearance(
	config_state: State<'_, ConfigurationState>,
) -> CommandResult<ConfigurationAppearance> {
	let guard = config_state.get_data();
	let object = guard.as_ref().unwrap();

	Ok(object.appearance.clone())
}

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_set_theme(config_state: State<'_, ConfigurationState>, theme: String) -> CommandResult<()> {
	{
		let mut guard = config_state.get_data();
		let inner = guard.as_mut().unwrap();
		inner.appearance.theme = theme;
	}

	config_state.write().await?;

	Ok(())
}

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_reload(config_state: State<'_, ConfigurationState>) -> CommandResult<()> {
	config_state.reload().await?;

	Ok(())
}
