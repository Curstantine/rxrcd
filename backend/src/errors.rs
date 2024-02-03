use serde::Serialize;

pub type CommandResult<T> = Result<T, PassiveError>;

#[derive(Debug, Serialize)]
pub struct PassiveError(String);

impl From<anyhow::Error> for PassiveError {
	fn from(value: anyhow::Error) -> Self {
		PassiveError(value.to_string())
	}
}
