use deezer::models::Language;
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataLanguage {
	/// Same as account
	Account,
	Defined(Language),
}

impl Serialize for DataLanguage {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			DataLanguage::Account => serializer.serialize_str("account"),
			DataLanguage::Defined(lang) => lang.serialize(serializer),
		}
	}
}

impl<'de> Deserialize<'de> for DataLanguage {
	fn deserialize<D: Deserializer<'de>>(value: D) -> Result<DataLanguage, D::Error> {
		value.deserialize_string(DataLanguageVisitor)
	}
}

struct DataLanguageVisitor;

impl<'de> serde::de::Visitor<'de> for DataLanguageVisitor {
	type Value = DataLanguage;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a readable string containing a language code or an 'account' variant")
	}

	fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
		if v == "account" {
			Ok(DataLanguage::Account)
		} else {
			Language::deserialize(v.into_deserializer()).map(DataLanguage::Defined)
		}
	}
}

#[cfg(test)]
mod test {
	use deezer::models::Language;
	use serde::{Deserialize, Serialize};

	use super::DataLanguage;

	#[derive(Debug, Serialize, Deserialize)]
	struct DataLanguageTest {
		pub inner: DataLanguage,
	}

	#[test]
	fn data_language_tagged_ser() {
		let lang = DataLanguageTest {
			inner: DataLanguage::Defined(Language::English),
		};

		let json_repl = serde_json::to_string(&lang);
		assert!(json_repl.is_ok(), "{:#?}", json_repl.unwrap_err());
		assert_eq!(json_repl.unwrap(), r#"{"inner":"English"}"#);

		let lang = DataLanguageTest {
			inner: DataLanguage::Account,
		};
		let json_repl = serde_json::to_string(&lang);
		assert!(json_repl.is_ok(), "{:#?}", json_repl.unwrap_err());
		assert_eq!(json_repl.unwrap(), r#"{"inner":"Default"}"#);
	}

	#[test]
	fn data_language_tagged_de() {
		let json_str = r#"{"inner":""}"#;
		let json = serde_json::from_str::<DataLanguageTest>(json_str);

		assert!(json.is_ok(), "{:#?}", json.unwrap_err());
		assert_eq!(json.unwrap().inner, DataLanguage::Account);
	}
}
