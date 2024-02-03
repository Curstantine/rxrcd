use serde::Deserialize;

use crate::utils::configuration::ConfigurationAppearance;

#[derive(Debug, Deserialize)]
pub struct SetupData {
	pub config_appearance: ConfigurationAppearance,
}
