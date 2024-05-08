use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub(crate) enum RequestPOSTMethod {
	#[serde(rename = "deezer.getUserData")]
	GetUserData,
}

#[derive(Debug, Serialize)]
pub(crate) struct RequestPOSTBody {
	api_version: String,
	method: RequestPOSTMethod,
	#[serde(serialize_with = "serialize_none_as_str_null")]
	api_token: Option<String>,
}

fn serialize_none_as_str_null<S: Serializer>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error> {
	match value {
		Some(inner) => serializer.serialize_some(inner),
		None => serializer.serialize_str("null"),
	}
}

impl RequestPOSTBody {
	pub fn with_defaults(method: RequestPOSTMethod) -> Self {
		Self {
			api_version: "1.0".to_string(),
			api_token: None,
			method,
		}
	}

	pub fn with_api_token<S: ToString>(token: S, method: RequestPOSTMethod) -> Self {
		Self {
			api_version: "1.0".to_string(),
			api_token: Some(token.to_string()),
			method,
		}
	}
}
