use std::fmt::{self, write};

pub type DeezerResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	HttpError(reqwest::Error),
	UrlParse(url::ParseError),
	NotLoggedIn,
	AlreadyLoggedIn,
}

impl From<reqwest::Error> for Error {
	fn from(v: reqwest::Error) -> Self {
		Self::HttpError(v)
	}
}

impl From<url::ParseError> for Error {
	fn from(value: url::ParseError) -> Self {
		Self::UrlParse(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::HttpError(err) => write!(f, "HTTP error: {err}"),
			Error::UrlParse(err) => write!(f, "URL parse error: {err}"),
			Error::NotLoggedIn => write!(f, "Client is not logged in"),
			Error::AlreadyLoggedIn => write!(f, "Client is already logged in"),
		}
	}
}

impl std::error::Error for Error {}
