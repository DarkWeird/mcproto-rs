use serde::{Deserialize, Deserializer};

pub use de::MCProtoDeserializer;
pub(crate) use de::Seq;

mod de;
pub(crate) mod read;

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: ?Sized + Deserialize<'de>,
        D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
}

