use crate::models::ajax::AjaxRequestError;

pub type DeezerResult<T> = Result<T, Error>;

#[derive(Debug)]
/// Error implementation used to compile all the errors that could happen within this crate.
///
/// Note on eq:
/// While [`PartialEq`] is supported for this enum, variants like [`Error::HttpError`] does not support any form of equality due to how they are structured internally.
/// These variants return `false` in the cases.
pub enum Error {
	HttpError(reqwest::Error),
	UrlParse(url::ParseError),
	NotLoggedIn,
	AlreadyLoggedIn,
	AjaxRequestError(AjaxRequestError),
	InvalidArl,
}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::HttpError(_), Self::HttpError(_)) => false,
			(Self::UrlParse(l0), Self::UrlParse(r0)) => l0 == r0,
			(Self::AjaxRequestError(l0), Self::AjaxRequestError(r0)) => l0 == r0,
			_ => core::mem::discriminant(self) == core::mem::discriminant(other),
		}
	}
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

impl From<AjaxRequestError> for Error {
	fn from(value: AjaxRequestError) -> Self {
		Error::AjaxRequestError(value)
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
			Error::InvalidArl => write!(f, "The ARL provided was invalid"),
		}
	}
}

impl std::error::Error for Error {}
