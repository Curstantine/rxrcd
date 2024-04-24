use tauri::State;

use crate::{
	errors::CommandResult,
	models::{
		configuration::{ConfigurationAppearance, ConfigurationDownload},
		state::ConfigurationState,
	},
};

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_get_appearance(
	config_state: State<'_, ConfigurationState>,
) -> CommandResult<ConfigurationAppearance> {
	let guard = config_state.get_data();
	let object = guard.as_ref().unwrap();

	Ok(object.appearance.clone())
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_set_appearance(
	config_state: State<'_, ConfigurationState>,
	appearance: ConfigurationAppearance,
) -> CommandResult<()> {
	{
		let mut guard = config_state.get_data();
		let inner = guard.as_mut().unwrap();
		inner.appearance = appearance;
	}

	config_state.write().await?;

	Ok(())
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_get_download(config_state: State<'_, ConfigurationState>) -> CommandResult<ConfigurationDownload> {
	let guard = config_state.get_data();
	let object = guard.as_ref().unwrap();

	Ok(object.download.clone())
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_set_download(
	config_state: State<'_, ConfigurationState>,
	download: ConfigurationDownload,
) -> CommandResult<()> {
	{
		let mut guard = config_state.get_data();
		let inner = guard.as_mut().unwrap();
		inner.download = download;
	}

	config_state.write().await?;
	Ok(())
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_reload(config_state: State<'_, ConfigurationState>) -> CommandResult<()> {
	config_state.reload().await?;

	Ok(())
}
