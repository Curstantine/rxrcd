use std::{
	path::Path,
	sync::{Mutex, MutexGuard},
};

use anyhow::ensure;

use {
	reqwest::Client,
	tauri::PathResolver,
	tokio::sync::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard},
};

use crate::utils::{configuration::Configuration, directories::Directories};

#[derive(Debug, Default)]
pub struct AppState(pub Mutex<App>);

#[derive(Debug, Default)]
pub struct DirectoriesState(pub Mutex<Option<Directories>>);

#[derive(Debug, Default)]
pub struct ConfigurationState(pub Mutex<Option<Configuration>>);

#[derive(Debug, Default)]
pub struct NetworkClientState(pub AsyncMutex<Option<reqwest::Client>>);

#[derive(Debug, Default)]
pub struct App {
	pub initialized: bool,
}

impl AppState {
	pub fn initialize(&self) -> Option<()> {
		let mut self_lock = self.0.lock().unwrap();

		if self_lock.initialized {
			return None;
		}

		self_lock.initialized = true;

		Some(())
	}
}

impl DirectoriesState {
	pub fn initialize(&self, path_resolver: &PathResolver) {
		let db = Directories::initialize(path_resolver);
		self.get().replace(db);
	}

	#[inline(always)]
	pub fn get(&self) -> MutexGuard<'_, Option<Directories>> {
		self.0.lock().unwrap()
	}
}

impl ConfigurationState {
	pub async fn initialize(&self, config_dir: &Path) -> anyhow::Result<()> {
		let conf = Configuration::initialize(config_dir).await?;
		self.get().replace(conf);

		Ok(())
	}

	#[inline(always)]
	pub fn get(&self) -> MutexGuard<'_, Option<Configuration>> {
		self.0.lock().unwrap()
	}
}

impl NetworkClientState {
	pub async fn initialize(&self) -> anyhow::Result<()> {
		let mut headers = reqwest::header::HeaderMap::new();

		ensure!(
			headers
				.insert(reqwest::header::CONTENT_TYPE, "json".parse().unwrap())
				.is_none(),
			"Content-Type header should not be populated in this Client"
		);

		let client = reqwest::ClientBuilder::new()
			.https_only(true)
			.default_headers(headers)
			.build()?;

		self.get().await.replace(client);

		Ok(())
	}

	#[inline(always)]
	pub async fn get(&self) -> AsyncMutexGuard<'_, Option<Client>> {
		self.0.lock().await
	}
}
