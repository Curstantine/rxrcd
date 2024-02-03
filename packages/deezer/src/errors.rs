use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	HttpError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
	fn from(v: reqwest::Error) -> Self {
		Self::HttpError(v)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::HttpError(err) => write!(f, "HTTP error: {}", err),
		}
	}
}

impl std::error::Error for Error {}
