use tauri::State;

use crate::{errors::CommandResult, state::NetworkClientState};

#[tauri::command]
pub async fn query_albums(query: String, network_state: State<'_, NetworkClientState>) -> CommandResult<()> {
	let client = network_state.get();

	Ok(())
}
