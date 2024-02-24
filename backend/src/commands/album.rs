use anyhow::anyhow;
use deezer::models::album::{AlbumSearch, ArtistAlbumList};
use tracing::info;

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
	index: Option<u32>,
	network_state: State<'_, NetworkClientState>,
) -> CommandResult<AlbumSearch> {
	info!("search_albums called");

	let client_guard = network_state.get().await;
	let client = client_guard.as_ref().unwrap();

	let options = SearchOptions::with_query(&query, Some(SearchOrder::AlbumAsc), limit.or(Some(8)), index);

	deezer::album::search_albums(client, &options)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}

#[tauri::command]
#[tracing::instrument(skip(network_state), err(Debug))]
pub async fn get_artist_albums(
	artist_id: u64,
	limit: Option<u32>,
	index: Option<u32>,
	network_state: State<'_, NetworkClientState>,
) -> CommandResult<ArtistAlbumList> {
	info!("get_artist_albums called");

	let client_guard = network_state.get().await;
	let client = client_guard.as_ref().unwrap();

	let options = SearchOptions::new(Some(SearchOrder::AlbumAsc), limit.or(Some(100)), index);

	deezer::album::get_artist_albums(client, artist_id, &options)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}
