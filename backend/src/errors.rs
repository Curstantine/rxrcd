use serde::Serialize;

pub type CommandResult<T> = Result<T, PassiveError>;

#[derive(Debug, Serialize)]
pub struct PassiveError(pub String);

impl From<anyhow::Error> for PassiveError {
	fn from(value: anyhow::Error) -> Self {
		Self(value.to_string())
	}
}

impl From<deezer::errors::Error> for PassiveError {
	fn from(value: deezer::errors::Error) -> Self {
		Self(value.to_string())
	}
}
