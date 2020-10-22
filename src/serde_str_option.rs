//! `serde_str` provides a serde adapter which handles the case where a rendered optional Mobiledoc string
//! is a field of some other object. Use like:
//!
//! ```rust
//! # use Mobiledoc;
//! #[derive(Serialize, Deserialize)]
//! struct Something {
//!     #[serde(with = "serde_str_option")]
//!     rendered_mobiledoc: Option<Mobiledoc>,
//! }
//! ```

use super::Mobiledoc;
use serde::{Deserializer, Serializer};
use std::fmt;

/// Serialize a `Mobiledoc` into an optional string.
///
/// Intended for use with serde's `serialize_with` attribute.
pub fn serialize<S>(opt: &Option<Mobiledoc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt {
        Some(ref mobiledoc) => serializer.serialize_some(&mobiledoc.to_string()),
        None => serializer.serialize_none(),
    }
}

pub struct Visitor;

/// Deserialize a `Mobiledoc` from an optional string.
///
/// Intended for use with serde's `deserialize_with` attribute.
impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Option<Mobiledoc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Mobiledoc serialized as an optional string")
    }

    fn visit_unit<E>(self) -> Result<Option<Mobiledoc>, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_none<E>(self) -> Result<Option<Mobiledoc>, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Option<Mobiledoc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_str(crate::serde_str::Visitor)
            .map(Some)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Mobiledoc>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(Visitor)
}
