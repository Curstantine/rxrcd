use serde::{Serialize, Serializer};

pub type CommandResult<T> = Result<T, PassiveError>;

#[derive(Debug)]
pub struct PassiveError(pub anyhow::Error);

impl Serialize for PassiveError {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.0.to_string().serialize(serializer)
	}
}

impl From<anyhow::Error> for PassiveError {
	fn from(value: anyhow::Error) -> Self {
		Self(value)
	}
}
