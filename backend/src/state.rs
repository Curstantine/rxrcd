use std::{
	path::{Path, PathBuf},
	sync::MutexGuard,
};

use anyhow::{Context, Result};
use deezer::client::DeezerClient;
use tokio::sync::MutexGuard as AsyncMutexGuard;
use tracing::debug;

use crate::{
	models::{
		configuration::Configuration,
		state::{ConfigurationState, DeezerClientState},
	},
	utils::{configuration, directories},
};

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

impl DeezerClientState {
	#[inline(always)]
	pub async fn get(&self) -> AsyncMutexGuard<'_, Option<DeezerClient>> {
		self.0.lock().await
	}

	pub async fn initialize(&self) -> Result<()> {
		let client = DeezerClient::new("rxrcd", env!("CARGO_PKG_VERSION"));
		let mut lock = self.0.lock().await;
		*lock = Some(client);

		Ok(())
	}
}
