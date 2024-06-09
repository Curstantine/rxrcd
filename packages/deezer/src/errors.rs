use crate::models::ajax::AjaxRequestError;

pub type DeezerResult<T> = Result<T, Error>;

/// Error implementation used to compile all the errors that could happen within this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
	Network(String, Option<reqwest::StatusCode>, Option<reqwest::Url>),
	UrlParse(url::ParseError),

	AjaxRequestError(AjaxRequestError),
	NotLoggedIn,
	AlreadyLoggedIn,
	InvalidArl,
	InvalidCredentials,
}

impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		let url = value.url().cloned();
		let status_code = value.status();
		Self::Network(value.without_url().to_string(), status_code, url)
	}
}

impl From<url::ParseError> for Error {
	fn from(value: url::ParseError) -> Self {
		Self::UrlParse(value)
	}
}

impl From<AjaxRequestError> for Error {
	fn from(value: AjaxRequestError) -> Self {
		Error::AjaxRequestError(value)
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Error::Network(message, status, url) => {
				write!(f, "Network error: {message}")?;

				if let Some(x) = status {
					write!(f, " ({x:?})")?;
				}

				if let Some(x) = url {
					write!(f, " at url: {x:?}")?;
				}

				Ok(())
			}
			Error::UrlParse(err) => write!(f, "URL parse error: {err}"),
			Error::AjaxRequestError(err) => match err {
				AjaxRequestError::GatewayError(x) => write!(f, "AJAX gateway error: {x}"),
				AjaxRequestError::ValidTokenRequired(x) => write!(f, "AJAX CSRF token is invalid, returned: {x}"),
			},
			Error::NotLoggedIn => write!(f, "Client is not logged in"),
			Error::AlreadyLoggedIn => write!(f, "Client is already logged in"),
			Error::InvalidArl => write!(f, "The ARL provided was invalid"),
			Error::InvalidCredentials => write!(f, "The credentials provided are invalid"),
		}
	}
}

impl std::error::Error for Error {}
