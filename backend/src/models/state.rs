use std::{path::PathBuf, sync::Mutex};

use tokio::sync::Mutex as AsyncMutex;

use crate::models::configuration::Configuration;

#[derive(Debug, Default)]
pub struct AppState(pub Mutex<App>);

#[derive(Debug, Default)]
pub struct ConfigurationState(pub Mutex<Option<Configuration>>, pub Mutex<Option<PathBuf>>);

#[derive(Debug, Default)]
pub struct NetworkClientState(pub AsyncMutex<Option<reqwest::Client>>);

#[derive(Debug, Default)]
pub struct App {
	pub initialized: bool,
}
