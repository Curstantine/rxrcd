use std::{io::ErrorKind, path::Path};

use anyhow::{bail, Context, Result};

use crate::models::configuration::Configuration;

pub async fn initialize(config_path: &Path) -> Result<Configuration> {
	let config = match tokio::fs::read_to_string(&config_path).await {
		Ok(string) => toml::from_str::<Configuration>(&string)?,
		Err(e) if e.kind() == ErrorKind::NotFound => {
			let config = Configuration::default();
			let config_str = toml::to_string_pretty(&config)?;

			let parent = config_path.parent().unwrap();

			tokio::fs::create_dir_all(parent)
				.await
				.with_context(move || format!("Failed to create configuration directory at {parent:?}"))?;

			tokio::fs::write(&config_path, config_str)
				.await
				.with_context(move || format!("Failed to create configuration at {config_path:?}"))?;

			config
		}
		Err(e) => return Err(e.into()),
	};

	Ok(config)
}

pub async fn initialize_blindly(config_path: &Path) -> Result<Configuration> {
	let config = match tokio::fs::read_to_string(&config_path).await {
		Ok(string) => toml::from_str::<Configuration>(&string)?,
		Err(e) if e.kind() == ErrorKind::NotFound => {
			bail!("Config was blindly initialized, but the file doesn't exist at {config_path:?}");
		}
		Err(e) => return Err(e.into()),
	};

	Ok(config)
}
