use std::{
	path::Path,
	sync::{Mutex, MutexGuard},
};

use tauri::PathResolver;

use crate::utils::{configuration::Configuration, directories::Directories};

#[derive(Debug, Default)]
pub struct AppState(pub Mutex<App>);

#[derive(Debug, Default)]
pub struct DirectoriesState(pub Mutex<Option<Directories>>);

#[derive(Debug, Default)]
pub struct ConfigurationState(pub Mutex<Option<Configuration>>);

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
		let db = Configuration::initialize(config_dir).await?;
		self.get().replace(db);

		Ok(())
	}

	#[inline(always)]
	pub fn get(&self) -> MutexGuard<'_, Option<Configuration>> {
		self.0.lock().unwrap()
	}
}
