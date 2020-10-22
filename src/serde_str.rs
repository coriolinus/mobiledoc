//! `serde_str` provides a serde adapter which handles the case where a rendered Mobiledoc string
//! is a field of some other object. Use like:
//!
//! ```rust
//! # use Mobiledoc;
//! #[derive(Serialize, Deserialize)]
//! struct Something {
//!     #[serde(with = "serde_str")]
//!     rendered_mobiledoc: Mobiledoc,
//! }
//! ```

use super::Mobiledoc;
use serde::{Deserializer, Serializer};
use std::fmt;

/// Serialize a `Mobiledoc` into a string.
///
/// Intended for use with serde's `serialize_with` attribute.
pub fn serialize<S>(mobiledoc: &Mobiledoc, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&mobiledoc.to_string())
}

pub struct Visitor;

/// Deserialize a `Mobiledoc` from a string.
///
/// Intended for use with serde's `deserialize_with` attribute.
impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Mobiledoc;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Mobiledoc serialized as a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::from_str(s).map_err(|e| E::custom(e))
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Mobiledoc, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(Visitor)
}
