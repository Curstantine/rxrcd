// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

use {tauri::WindowEvent, tracing::info};

#[cfg(any(windows, target_os = "macos"))]
use {tauri::Manager, window_shadows::set_shadow};

use crate::models::state::{AppState, ConfigurationState, DeezerClientState};

mod commands;
mod constants;
mod errors;
mod models;
mod state;
mod utils;

fn main() {
	tracing_subscriber::fmt::init();
	info!("Starting application");

	#[allow(unused_variables)]
	tauri::Builder::default()
		.setup(|app| {
			#[cfg(any(windows, target_os = "macos"))]
			{
				let window = app.get_window("main").unwrap();
				set_shadow(&window, true).unwrap();
			}

			Ok(())
		})
		.on_window_event(|e| {
			// Note(Curstantine):
			// To temporarily fix the resize perf bugs mentioned in https://github.com/tauri-apps/tauri/issues/6322
			if let WindowEvent::Resized(_) = e.event() {
				thread::sleep(Duration::from_nanos(1));
			}
		})
		.manage(AppState::default())
		.manage(ConfigurationState::default())
		.manage(DeezerClientState::default())
		.invoke_handler(tauri::generate_handler![
			commands::setup,
			// commands/config
			commands::config::config_get_account,
			commands::config::config_set_account,
			commands::config::config_get_appearance,
			commands::config::config_set_appearance,
			commands::config::config_get_download,
			commands::config::config_set_download,
			commands::config::config_reload,
			// commands/album
			commands::album::get_album,
			commands::album::search_albums,
			commands::album::get_artist_albums,
			// commands/artist
			commands::artist::get_artist,
			commands::artist::search_artists,
			// commands/user
			commands::user::get_auth_state,
			commands::user::refresh_login,
			commands::user::login,
			commands::user::logout,
			commands::user::change_data_language,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
