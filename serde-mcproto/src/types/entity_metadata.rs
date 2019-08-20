use std::collections::HashMap;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

use super::var::VarInt;
use super::slot::Slot;
use super::uuid::Uuid;
use super::chat::Chat;
use serde::de::{Visitor, SeqAccess, DeserializeOwned, DeserializeSeed};
use std::convert::TryInto;
use core::borrow::Borrow;

#[derive(Debug, Serialize)]
pub struct EntityMetadata(HashMap<u8, Entry>);

#[derive(Debug, Serialize, Deserialize)]
pub enum Entry {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Slot(Option<Slot>),
    Rotation([i32; 3]),
}

impl<'de> Deserialize<'de> for EntityMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct MetaDataVisitor;
        impl<'de> Visitor<'de> for MetaDataVisitor {
            type Value = EntityMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                formatter.write_str("a entity meta")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where
                A: SeqAccess<'de>, {
                let mut map = HashMap::new();
                while let Some((id, entry)) = seq.next_element()? {
                    map.entry(id).or_insert(entry);
                };
                Ok(EntityMetadata(map))
            }
        }
        deserializer.deserialize_newtype_struct("MCMETADATAENTRY", MetaDataVisitor)
    }
}