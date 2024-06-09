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

/// Guards the deserialization of [`User`] (or field that digests a [`u64`]) with a minimum value of 1.
pub fn de_user_id<'de, D: Deserializer<'de>>(value: D) -> Result<u64, D::Error> {
	value.deserialize_u64(RangedU64Visitor(Some(1), None))
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

struct RangedU64Visitor(Option<u64>, Option<u64>);

impl<'de> Visitor<'de> for RangedU64Visitor {
	type Value = u64;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			formatter,
			"an unsigned 64 bit integer with min: {} and max: {}",
			self.0.unwrap_or(u64::MIN),
			self.1.unwrap_or(u64::MAX)
		)
	}

	fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
		let min_caught = self.0.is_none() || matches!(self.0, Some(min) if v >= min);
		let max_caught = self.1.is_none() || matches!(self.1, Some(max) if v <= max);

		if !min_caught | !max_caught {
			return Err(E::invalid_value(serde::de::Unexpected::Unsigned(v), &self));
		}

		Ok(v)
	}
}

#[cfg(test)]
mod test {
	use serde::Deserialize;

	use crate::{constants, models::ajax::AjaxRequestError};

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

	#[test]
	fn de_user_id() {
		#[derive(Debug, Deserialize)]
		struct Test {
			#[serde(deserialize_with = "crate::serde::de_user_id")]
			id: u64,
		}

		let json_zeroed = r#"{ "id": 0 }"#;
		let zeroed = serde_json::from_str::<Test>(json_zeroed);
		assert!(zeroed.is_err());

		let error = zeroed.unwrap_err();
		assert!(error.is_data());
		assert!(error.to_string().contains(constants::ERROR_SERDE_INVALID_ID));

		let json_good = r#"{ "id": 1 }"#;
		let good = serde_json::from_str::<Test>(json_good);
		assert!(good.is_ok(), "{good:#?}");
		assert_eq!(good.unwrap().id, 1);
	}
}
