use std::marker::PhantomData;

use serde::{
	de::{value::MapAccessDeserializer, Visitor},
	Deserialize, Deserializer, Serializer,
};

use crate::models::ajax::AjaxRequestError;

pub mod language_code;

/// Deserializes an [`AjaxRequestError`] value that might either be an empty array
pub fn de_ajax_req_err<'de, D: Deserializer<'de>>(value: D) -> Result<Option<AjaxRequestError>, D::Error> {
	value.deserialize_any(OptionAjaxNullishVisitor::<AjaxRequestError>(PhantomData))
}

pub fn ser_none_as_str<S: Serializer>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error> {
	match value {
		Some(inner) => serializer.serialize_some(inner),
		None => serializer.serialize_str("null"),
	}
}

/// Serde visitor to map empty units, sequences and other falsy values to None.
///
/// Deezer's ajax handler uses PHP, and PHP-based backends have this type mismatch issue.
struct OptionAjaxNullishVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for OptionAjaxNullishVisitor<T> {
	type Value = Option<T>;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("An empty vector without an underlying type")
	}

	fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
		T::deserialize(deserializer).map(Some)
	}

	fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
		let map_de = MapAccessDeserializer::new(map);
		T::deserialize(map_de).map(Some)
	}

	fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
		Ok(None)
	}

	fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
		Ok(None)
	}

	fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
		match seq.size_hint() {
			None => Ok(None),
			Some(0) => Ok(None),
			Some(_) => panic!("This value is a sequence and is populated. You should use a Vec proxy instead"),
		}
	}
}

#[cfg(test)]
mod test {
	use serde::Deserialize;

	use crate::models::ajax::AjaxRequestError;

	#[test]
	fn test_de_ajax_req_err() {
		#[derive(Debug, Deserialize)]
		struct Error {
			#[serde(deserialize_with = "crate::serde::de_ajax_req_err")]
			error: Option<AjaxRequestError>,
		}

		let nulled_json = r#"{ "error": [] }"#;
		let nulled = serde_json::from_str::<Error>(nulled_json);
		assert!(
			nulled.is_ok(),
			"Expected nulled_json to deserialize without an issue, but errored out with: {:#?}",
			nulled.unwrap_err()
		);
		assert!(nulled.unwrap().error.is_none());

		let valued_json = r#"{ "error": { "GATEWAY_ERROR": "Sample error ahead" } }"#;
		let valued = serde_json::from_str::<Error>(valued_json);
		assert!(
			valued.is_ok(),
			"Expected valued_json to deserialize without an issue, but errored out with: {:?}",
			valued.unwrap_err()
		);

		let valued_unwrapped = valued.unwrap();
		assert!(valued_unwrapped.error.is_some());
		assert_eq!(
			valued_unwrapped.error.unwrap(),
			AjaxRequestError::GatewayError("Sample error ahead".to_string())
		)
	}
}
