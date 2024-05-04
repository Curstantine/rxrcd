use anyhow::anyhow;
use tauri::State;

use crate::{
	errors::{CommandResult, PassiveError},
	models::{state::DeezerClientState, user::UserData},
};

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn login_with_arl(deezer_state: State<'_, DeezerClientState>, arl: String) -> CommandResult<UserData> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	deezer::user::login_with_arl(client, &arl)
		.await
		.map(UserData::from)
		.map_err(|e| PassiveError::from(anyhow!(e)))
}
