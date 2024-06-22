use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ResponseBody<T: Debug> {
	pub results: Option<T>,
	#[serde(deserialize_with = "crate::serde::de_ajax_req_err")]
	pub error: Option<AjaxRequestError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AjaxRequestError {
	GatewayError(String),
	ValidTokenRequired(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub(crate) enum RequestPOSTMethod {
	#[serde(rename = "deezer.getUserData")]
	GetUserData,

	#[serde(rename = "song.getListByAlbum")]
	GetListByAlbum,
}

#[derive(Debug, Serialize)]
pub(crate) struct RequestPOSTBody<T> {
	api_version: String,
	method: RequestPOSTMethod,
	#[serde(serialize_with = "crate::serde::ser_none_as_str")]
	api_token: Option<String>,

	#[serde(flatten)]
	data: Option<T>,
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

impl<T> RequestPOSTBody<T> {
	pub fn new(method: RequestPOSTMethod) -> Self {
		Self {
			api_version: "1.0".to_string(),
			api_token: None,
			method,
			data: None,
		}
	}

	pub fn set_api_token<S: ToString>(mut self, token: S) -> Self {
		self.api_token = Some(token.to_string());
		self
	}

	pub fn set_data(mut self, data: T) -> Self {
		self.data = Some(data);
		self
	}
}

#[cfg(test)]
mod test {
	use serde::Serialize;

	use super::{RequestPOSTBody, RequestPOSTMethod};

	#[derive(Debug, Serialize)]
	struct BodyData {
		#[serde(rename = "ALB_ID")]
		album_id: u64,
	}

	impl BodyData {
		fn new(album_id: u64) -> Self {
			Self { album_id }
		}
	}

	#[test]
	fn post_req_body_data_ser() {
		const API_TOKEN: &str = "241--213";
		const ALBUM_ID: u64 = 10012;

		let body = RequestPOSTBody::new(RequestPOSTMethod::GetListByAlbum)
			.set_api_token(API_TOKEN)
			.set_data(BodyData::new(ALBUM_ID));
		let body_str = serde_json::to_string(&body);

		assert!(body_str.is_ok(), "{:#?}", body_str.unwrap_err());
		assert_eq!(
			body_str.unwrap(),
			format!(
				r#"{{"api_version":"1.0","method":"song.getListByAlbum","api_token":"{API_TOKEN}","ALB_ID":{ALBUM_ID}}}"#,
			)
		);
	}

	#[test]
	fn post_req_body_data_none_ser() {
		let body = RequestPOSTBody::<()>::new(RequestPOSTMethod::GetUserData);
		let body_str = serde_json::to_string(&body);

		assert!(body_str.is_ok(), "{:#?}", body_str.unwrap_err());
		assert_eq!(
			body_str.unwrap(),
			r#"{"api_version":"1.0","method":"deezer.getUserData","api_token":"null"}"#
		);
	}
}
