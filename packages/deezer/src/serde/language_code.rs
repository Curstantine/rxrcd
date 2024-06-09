use serde::{de::Visitor, Deserializer, Serializer};

use crate::models::Language;

// Note(Curstantine):
// The package doesn't currently rely on serializing the language code, but we might need to use this in the future.
// Can't use lint reason either because reason attribute is still experimental.
#[allow(dead_code)]
pub fn serialize<S: Serializer>(value: &Language, serializer: S) -> Result<S::Ok, S::Error> {
	serializer.serialize_str(value.as_language_code())
}

pub fn deserialize<'de, D: Deserializer<'de>>(value: D) -> Result<Language, D::Error> {
	value.deserialize_string(LanguageCodeVisitor)
}

struct LanguageCodeVisitor;

impl<'de> Visitor<'de> for LanguageCodeVisitor {
	type Value = Language;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a readable string containing a language code")
	}

	fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
		Language::from_language_code(v).ok_or_else(|| E::invalid_value(serde::de::Unexpected::Str(v), &self))
	}
}

#[cfg(test)]
mod test {
	use reqwest::header::HeaderValue;
	use serde::{Deserialize, Serialize};
	use serde_json::json;

	use crate::models::Language;

	#[test]
	fn from_language_code() {
		let val = Language::from_language_code("en");
		assert!(val.is_some());
		assert_eq!(val.unwrap(), Language::English);

		let unknown = Language::from_language_code("ida");
		assert!(unknown.is_none());
	}

	#[test]
	fn as_language_code() {
		let val = Language::BrazilianPortuguese;
		assert_eq!(val.as_language_code(), "br");
	}

	#[test]
	fn to_header_value() {
		let val = Language::Japanese;
		let header_val = HeaderValue::from_static("ja");

		assert_eq!(val.to_header_value(), header_val);
	}

	#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
	struct Test {
		#[serde(with = "crate::serde::language_code")]
		language: Language,
	}

	impl Test {
		fn new(language: Language) -> Self {
			Self { language }
		}
	}

	#[test]
	fn serde_serialization() {
		let val = Test::new(Language::English);
		let json_val = serde_json::to_value(val);

		let exp_val = json!({ "language": "en" });

		assert!(json_val.is_ok(), "{:#?}", json_val.unwrap_err());
		assert_eq!(json_val.unwrap(), exp_val);
	}

	#[test]
	fn serde_deserialization() {
		let json_val = r#"{ "language": "br" }"#;
		let val = serde_json::from_str::<Test>(json_val);

		assert!(val.is_ok(), "{:#?}", val.unwrap_err());
		assert_eq!(val.unwrap(), Test::new(Language::BrazilianPortuguese));
	}
}
