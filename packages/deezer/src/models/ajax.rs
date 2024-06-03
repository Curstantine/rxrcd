use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ResponseBody<T: Debug> {
	pub results: Option<T>,
	#[serde(deserialize_with = "crate::serde::de_ajax_req_err")]
	pub error: Option<AjaxRequestError>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AjaxRequestError {
	GatewayError(String),
	ValidTokenRequired(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub(crate) enum RequestPOSTMethod {
	#[serde(rename = "deezer.getUserData")]
	GetUserData,
}

#[derive(Debug, Serialize)]
pub(crate) struct RequestPOSTBody {
	api_version: String,
	method: RequestPOSTMethod,
	#[serde(serialize_with = "crate::serde::ser_none_as_str")]
	api_token: Option<String>,
}

impl<T: Debug> ResponseBody<T> {
	/// Maps Self into a [Result] type that contains either [Self::results] or [Self::error]
	///
	/// Note: This panics if both results and error are none.
	pub fn into_result(self) -> Result<T, AjaxRequestError> {
		self.results
			.ok_or_else(|| self.error.expect("Both results and errors are none"))
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

	#[allow(dead_code)]
	pub fn with_api_token<S: ToString>(token: S, method: RequestPOSTMethod) -> Self {
		Self {
			api_version: "1.0".to_string(),
			api_token: Some(token.to_string()),
			method,
		}
	}
}
