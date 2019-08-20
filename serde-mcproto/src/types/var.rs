use serde::{Serialize, Serializer, Deserialize, Deserializer};
use crate::ser::write::{write_varint, write_varlong};
use serde::de::{Visitor, DeserializeSeed, MapAccess};
use serde::export::{Formatter};

#[derive(Debug)]
pub struct VarInt(i32);

#[derive(Debug)]
pub struct VarLong(i64);


impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_varint(&self.0, &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}

impl Serialize for VarLong {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_varlong(&self.0, &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}


impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct VarLongVisitor;
        impl<'de> Visitor<'de> for VarLongVisitor {
            type Value = VarLong;

            fn expecting(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
                formatter.write_str("varlong")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where
                E: serde::de::Error, {
                Ok(VarLong(v))
            }
        }
        deserializer.deserialize_newtype_struct("MCVARLONG", VarLongVisitor)
    }
}

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct VarIntVisitor;
        impl<'de> Visitor<'de> for VarIntVisitor {
            type Value = VarInt;

            fn expecting(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
                formatter.write_str("varint")
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where
                E: serde::de::Error, {
                Ok(VarInt(v))
            }
        }
        deserializer.deserialize_newtype_struct("MCVARINT", VarIntVisitor)
    }
}


impl From<i32> for VarInt {
    #[inline]
    fn from(v: i32) -> Self {
        VarInt(v)
    }
}

impl From<i64> for VarLong {
    #[inline]
    fn from(v: i64) -> Self {
        VarLong(v)
    }
}

impl From<VarLong> for i64 {
    fn from(v: VarLong) -> Self {
        v.0
    }
}

impl From<VarInt> for i32 {
    fn from(v: VarInt) -> Self {
        v.0
    }
}