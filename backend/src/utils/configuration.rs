use std::{borrow::Borrow, io::ErrorKind, path::Path};

use {
	anyhow::{bail, Context, Result},
	tracing::debug,
};

use crate::models::configuration::{AuthConfiguration, Configuration};

pub async fn initialize(config_path: &Path) -> Result<Configuration> {
	debug!("Reading configuration file from {config_path:?}");

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

			debug!("Configuration files with defaults were created as part of the initialization process");
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

pub async fn read_auth_config(path: &Path) -> Result<Option<AuthConfiguration>> {
	match tokio::fs::read_to_string(path).await {
		Ok(string) => Ok(Some(toml::from_str::<AuthConfiguration>(&string)?)),
		Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
		Err(e) => Err(e.into()),
	}
}

pub async fn write_auth_config<C>(path: &Path, auth_config: C) -> Result<()>
where
	C: Borrow<AuthConfiguration> + std::fmt::Debug + serde::Serialize,
{
	let data = toml::to_string_pretty(&auth_config)
		.with_context(|| format!("Failed to TOML serialize auth_config: {auth_config:?}"))?;

	tokio::fs::write(path, data).await?;

	Ok(())
}
