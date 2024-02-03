use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeezerList<T> {
	pub data: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct DeezerPaginatedList<T> {
	pub data: Vec<T>,
	pub total: u32,
	pub next: Option<String>,
}
