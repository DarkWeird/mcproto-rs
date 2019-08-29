use serde::{Serialize, Deserialize, Deserializer};
use serde::de::Visitor;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NBT(nbt::Blob);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GZIPNBT(nbt::Blob);
