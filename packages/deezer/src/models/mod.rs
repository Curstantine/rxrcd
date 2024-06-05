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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Language {
	English,
	Japanese,
	BrazilianPortuguese,
}

impl Language {
	pub fn from_language_code(value: &str) -> Option<Self> {
		match value {
			"en" => Some(Language::English),
			"ja" => Some(Language::Japanese),
			"br" => Some(Language::BrazilianPortuguese),
			_ => None,
		}
	}

	pub fn as_language_code(&self) -> &'static str {
		match self {
			Language::English => "en",
			Language::Japanese => "ja",
			Language::BrazilianPortuguese => "br",
		}
	}

	pub fn to_header_value(&self) -> HeaderValue {
		HeaderValue::from_static(self.as_language_code())
	}
}
