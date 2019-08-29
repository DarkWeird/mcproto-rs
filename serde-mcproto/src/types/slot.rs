use serde::{Serialize, Deserialize};
use super::nbt::GZIPNBT;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    id: i16,
    count: u8,
    damage: i16,
    tag: Option<GZIPNBT>,
}