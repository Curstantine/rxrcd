use {
	anyhow::anyhow,
	deezer::models::{
		artist::{Artist, ArtistSearch},
		search::{SearchOptions, SearchOrder},
	},
	tauri::State,
};

use crate::{
	errors::{CommandResult, PassiveError},
	models::state::DeezerClientState,
};

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn get_artist(id: u64, deezer_state: State<'_, DeezerClientState>) -> CommandResult<Artist> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	deezer::artist::get_artist(client, id)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn search_artists(
	query: String,
	limit: Option<u32>,
	index: Option<u32>,
	deezer_state: State<'_, DeezerClientState>,
) -> CommandResult<ArtistSearch> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	let opts = SearchOptions::with_query(&query, Some(SearchOrder::ArtistDesc), limit.or(Some(6)), index);

	deezer::artist::search_artists(client, &opts)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}
