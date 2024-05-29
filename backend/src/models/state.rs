use std::{
	path::PathBuf,
	sync::{atomic::AtomicBool, Mutex},
};

use deezer::client::DeezerClient;
use tokio::sync::Mutex as AsyncMutex;

use crate::models::configuration::Configuration;

#[derive(Debug, Default)]
pub struct AppState {
	pub initialized: AtomicBool,
}

#[derive(Debug, Default)]
pub struct ConfigurationState(pub Mutex<Option<Configuration>>, pub Mutex<Option<PathBuf>>);

#[derive(Debug, Default)]
pub struct DeezerClientState(pub AsyncMutex<Option<DeezerClient>>);
