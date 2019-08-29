use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Visitor;

#[derive(Debug, Clone)]
pub struct Uuid(pub uuid::Uuid);

impl Into<uuid::Uuid> for Uuid {
    #[inline]
    fn into(self) -> uuid::Uuid {
        self.0
    }
}

impl From<uuid::Uuid> for Uuid {
    #[inline]
    fn from(u: uuid::Uuid) -> Self {
        Uuid(u)
    }
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&*self.0.to_hyphenated().to_string())
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct UUIDVisitor;

        impl<'a> Visitor<'a> for UUIDVisitor {
            type Value = Uuid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                Ok(())
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
                E: serde::de::Error, {
                use std::str::FromStr;
                uuid::Uuid::from_str(&*v)
                    .map(|v| Uuid(v))
                    .map_err(|e| serde::de::Error::custom(e.to_string()))
            }
        }
        deserializer.deserialize_string(UUIDVisitor)
    }
}


#[derive(Debug, Clone)]
pub struct Uuidi128(pub uuid::Uuid);

impl Into<uuid::Uuid> for Uuidi128 {
    #[inline]
    fn into(self) -> uuid::Uuid {
        self.0
    }
}

impl From<uuid::Uuid> for Uuidi128 {
    #[inline]
    fn from(u: uuid::Uuid) -> Self {
        Uuidi128(u)
    }
}

impl Serialize for Uuidi128 {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut bytes = self.0.as_bytes().as_ref();
        let uuid = byteorder::ReadBytesExt::read_u128::<byteorder::NativeEndian>(&mut bytes).unwrap();
        serializer.serialize_u128(uuid)
    }
}

impl<'de> Deserialize<'de> for Uuidi128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct UUIDVisitor;

        impl<'a> Visitor<'a> for UUIDVisitor {
            type Value = Uuidi128;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                Ok(())
            }
            serde::serde_if_integer128! {
                fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where
                    E: serde::de::Error, {
                        Ok(Uuidi128(uuid::Uuid::from(v)))
                }
            }
        }
        deserializer.deserialize_u128(UUIDVisitor)
    }
}


