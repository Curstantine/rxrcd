use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeezerList<T> {
	data: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct DeezerPaginatedList<T> {
	data: Vec<T>,
	total: u64,
}
