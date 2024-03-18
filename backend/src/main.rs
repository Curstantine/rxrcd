// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

use {tauri::WindowEvent, tracing::info};

#[cfg(any(windows, target_os = "macos"))]
use {tauri::Manager, window_shadows::set_shadow};

use crate::state::{AppState, ConfigurationState, DirectoriesState, NetworkClientState};

mod commands;
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
			// To alleviate the resize perf bugs mentioned in https://github.com/tauri-apps/tauri/issues/6322
			if let WindowEvent::Resized(_) = e.event() {
				thread::sleep(Duration::from_nanos(1));
			}
		})
		.manage(AppState::default())
		.manage(DirectoriesState::default())
		.manage(ConfigurationState::default())
		.manage(NetworkClientState::default())
		.invoke_handler(tauri::generate_handler![
			commands::setup,
			commands::config::config_get_appearance,
			commands::album::get_album,
			commands::album::search_albums,
			commands::album::get_artist_albums,
			commands::artist::get_artist,
			commands::artist::search_artists,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
