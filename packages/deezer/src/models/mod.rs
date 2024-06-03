use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

pub(crate) mod ajax;

pub mod album;
pub mod artist;
pub mod generic;
pub mod genre;
pub mod search;
pub mod track;
pub mod user;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Language {
	#[serde(rename = "en")]
	English,

	#[serde(rename = "ja")]
	Japanese,

	#[serde(rename = "br")]
	BrazilianPortuguese,
}

impl Language {
	pub fn to_header_value(&self) -> HeaderValue {
		let val: &'static str = match self {
			Language::English => "en",
			Language::Japanese => "ja",
			Language::BrazilianPortuguese => "br",
		};

		HeaderValue::from_static(val)
	}
}
