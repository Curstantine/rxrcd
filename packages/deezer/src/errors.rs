use crate::models::ajax::AjaxRequestError;

pub type DeezerResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	HttpError(reqwest::Error),
	UrlParse(url::ParseError),
	NotLoggedIn,
	AlreadyLoggedIn,
	AjaxRequestError(AjaxRequestError),
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

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Error::HttpError(err) => write!(f, "HTTP error: {err}"),
			Error::UrlParse(err) => write!(f, "URL parse error: {err}"),
			Error::AjaxRequestError(err) => match err {
				AjaxRequestError::GatewayError(x) => write!(f, "AJAX gateway error: {x}"),
				AjaxRequestError::ValidTokenRequired(x) => write!(f, "AJAX CSRF token is invalid, returned: {x}"),
			},
			Error::NotLoggedIn => write!(f, "Client is not logged in"),
			Error::AlreadyLoggedIn => write!(f, "Client is already logged in"),
		}
	}
}

impl std::error::Error for Error {}
