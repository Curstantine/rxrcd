use anyhow::anyhow;
use deezer::models::artist::{Artist, ArtistSearch};

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
pub async fn get_artist(id: u64, network_state: State<'_, NetworkClientState>) -> CommandResult<Artist> {
	let client_guard = network_state.get().await;
	let client = client_guard.as_ref().unwrap();

	deezer::artist::get_artist(client, id)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}

#[tauri::command]
#[tracing::instrument(skip(network_state), err(Debug))]
pub async fn search_artists(
	query: String,
	limit: Option<u32>,
	index: Option<u32>,
	network_state: State<'_, NetworkClientState>,
) -> CommandResult<ArtistSearch> {
	let client_guard = network_state.get().await;
	let client = client_guard.as_ref().unwrap();

	let opts = SearchOptions::with_query(&query, Some(SearchOrder::ArtistDesc), limit.or(Some(6)), index);

	deezer::artist::search_artists(client, &opts)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}
