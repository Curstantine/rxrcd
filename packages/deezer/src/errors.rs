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
