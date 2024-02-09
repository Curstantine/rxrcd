use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeezerList<T> {
	pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeezerPaginatedList<T> {
	pub data: Vec<T>,
	pub total: u32,
	pub next: Option<String>,
}
