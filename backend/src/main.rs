// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};

use {tauri::WindowEvent, tracing::info};

#[cfg(any(windows, target_os = "macos"))]
use {tauri::Manager, window_shadows::set_shadow};

use crate::state::{AppState, DirectoriesState};

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
		.invoke_handler(tauri::generate_handler![
			commands::setup,
			commands::config::config_get_appearance
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
