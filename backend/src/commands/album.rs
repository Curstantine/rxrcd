use anyhow::anyhow;
use deezer::models::album::AlbumSearch;

use {
	deezer::models::search::{SearchOptions, SearchOrder},
	tauri::State,
};

use crate::{
	errors::{CommandResult, PassiveError},
	state::NetworkClientState,
};

#[tauri::command]
#[tracing::instrument(skip(network_state), err(Debug))]
pub async fn search_albums(
	query: String,
	limit: Option<u32>,
	network_state: State<'_, NetworkClientState>,
) -> CommandResult<AlbumSearch> {
	let client_guard = network_state.get().await;
	let client = client_guard.as_ref().unwrap();

	let opts = SearchOptions::new(&query, Some(SearchOrder::AlbumAsc)).with_limit(limit.unwrap_or(6));

	deezer::album::search_albums(client, &opts)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}
