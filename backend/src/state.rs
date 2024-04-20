use std::{
	path::{Path, PathBuf},
	sync::MutexGuard,
};

use tracing::debug;

use {
	anyhow::{ensure, Context, Result},
	reqwest::Client,
	tokio::sync::MutexGuard as AsyncMutexGuard,
};

use crate::{
	models::{
		configuration::Configuration,
		state::{AppState, ConfigurationState, NetworkClientState},
	},
	utils::{configuration, directories},
};

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

impl ConfigurationState {
	#[inline(always)]
	pub fn get_data(&self) -> MutexGuard<'_, Option<Configuration>> {
		self.0.lock().unwrap()
	}

	pub fn get_path(&self) -> MutexGuard<'_, Option<PathBuf>> {
		self.1.lock().unwrap()
	}

	pub fn get_owned_path(&self) -> PathBuf {
		self.get_path().clone().unwrap()
	}

	pub async fn initialize(&self, app_config_dir: &Path) -> Result<()> {
		let config_path = directories::get_config_path(app_config_dir);
		let conf = configuration::initialize(&config_path).await?;

		*self.get_data() = Some(conf);
		*self.get_path() = Some(config_path);

		Ok(())
	}

	pub async fn reload(&self) -> Result<()> {
		let config_path = self.get_owned_path();
		let conf = configuration::initialize_blindly(&config_path).await?;
		*self.get_data() = Some(conf);

		Ok(())
	}

	pub async fn write(&self) -> Result<()> {
		let toml_str = {
			let inner = self.get_data();
			toml::to_string_pretty(inner.as_ref().unwrap())
				.with_context(move || format!("Failed to serialized the config in its current state: {self:#?}"))?
		};

		let config_path = self.get_owned_path();
		debug!("Modified configuration is being written to {config_path:?}");

		tokio::fs::write(&config_path, toml_str)
			.await
			.with_context(move || format!("Failed to write modified configuration to {config_path:?}"))?;

		Ok(())
	}
}

impl NetworkClientState {
	#[inline(always)]
	pub async fn get(&self) -> AsyncMutexGuard<'_, Option<Client>> {
		self.0.lock().await
	}

	pub async fn initialize(&self) -> Result<()> {
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
}
