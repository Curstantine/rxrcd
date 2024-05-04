use {
	anyhow::anyhow,
	deezer::models::{
		album::{Album, AlbumSearch, ArtistAlbumList},
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
pub async fn get_album(id: u64, deezer_state: State<'_, DeezerClientState>) -> CommandResult<Album> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	deezer::album::get_album(client, id)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn search_albums(
	query: String,
	limit: Option<u32>,
	index: Option<u32>,
	deezer_state: State<'_, DeezerClientState>,
) -> CommandResult<AlbumSearch> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	let options = SearchOptions::with_query(&query, Some(SearchOrder::AlbumAsc), limit.or(Some(8)), index);

	deezer::album::search_albums(client, &options)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}

#[tauri::command(rename_all = "snake_case")]
#[tracing::instrument(skip(deezer_state), err(Debug))]
pub async fn get_artist_albums(
	artist_id: u64,
	limit: Option<u32>,
	index: Option<u32>,
	deezer_state: State<'_, DeezerClientState>,
) -> CommandResult<ArtistAlbumList> {
	let deezer_guard = deezer_state.get().await;
	let client = deezer_guard.as_ref().unwrap();

	let options = SearchOptions::new(Some(SearchOrder::AlbumAsc), limit.or(Some(100)), index);

	deezer::album::get_artist_albums(client, artist_id, &options)
		.await
		.map_err(|e| PassiveError::from(anyhow!(e)))
}
