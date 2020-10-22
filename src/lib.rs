use std::fmt;

use serde::{ser::SerializeSeq, Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

pub mod serde_str;
pub mod serde_str_option;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Mobiledoc {
    pub version: String,
    pub markups: Vec<Markup>,
    pub atoms: Vec<Atom>,
    pub cards: Vec<Card>,
    pub sections: Vec<Value>,
}

impl fmt::Display for Mobiledoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&serde_json::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}

/// Markups represent HTML tags
///
/// Markups have a tagName and an optional array of attributes. Not all markups can have attributes,
/// but for those that do the attributes array is a single array of all the attribute names and values,
/// one after another. E.g., `["a", ["href", "http://bustle.com", "target", "_blank"]`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Markup {
    pub tag: String,
    pub attributes: Vec<MarkupAttribute>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MarkupAttribute {
    pub name: String,
    pub value: String,
}

impl Serialize for Markup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.tag)?;
        let mut attributes = Vec::with_capacity(2 * self.attributes.len());
        for attribute in &self.attributes {
            attributes.push(&attribute.name);
            attributes.push(&attribute.value);
        }
        seq.serialize_element(&attributes)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Markup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Markup;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Markup")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Markup, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let tag = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let attributes_raw: Vec<String> = seq.next_element()?.unwrap_or_default();
                let mut attributes = Vec::with_capacity(attributes_raw.len() / 2);
                for chunk in attributes_raw.chunks(2) {
                    attributes.push(MarkupAttribute {
                        name: chunk[0].clone(),
                        value: chunk[1].clone(),
                    });
                }
                Ok(Markup { tag, attributes })
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

/// Atoms have a name, text value, and arbitrary payload.
///
/// E.g. `['mention', '@bob', { id: 42 }]`.
#[derive(Debug, PartialEq, Eq, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Atom {
    pub name: String,
    pub text: String,
    pub payload: Value,
}

/// Cards have a name and arbitrary payload.
///
/// E.g. `['image', {'src': 'http://google.com/logo.png'}]`
#[derive(Debug, PartialEq, Eq, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Card {
    pub name: String,
    pub payload: Value,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SectionType {
    Text = 1,
    Image = 2,
    List = 3,
    Card = 10,
}

/// Sections
///
/// E.g. `['image', {'src': 'http://google.com/logo.png'}]`
#[derive(Debug, PartialEq, Eq, Clone, Serialize_tuple, Deserialize_tuple)]
pub struct Card {
    pub name: String,
    pub payload: Value,
}
