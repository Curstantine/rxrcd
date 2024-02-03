use tauri::State;

use crate::{errors::CommandResult, state::ConfigurationState, utils::configuration::ConfigurationAppearance};

#[tauri::command]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn config_get_appearance(
	config_state: State<'_, ConfigurationState>,
) -> CommandResult<ConfigurationAppearance> {
	let guard = config_state.get();
	let object = guard.as_ref().unwrap();

	Ok(object.appearance.clone())
}
