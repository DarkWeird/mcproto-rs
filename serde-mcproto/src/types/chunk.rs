use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, Visitor, DeserializeSeed};

use super::array::IntPrefix;

#[derive(Debug, Serialize)]
pub struct ChunkDataBulk {
    chunk_column_count: i16,
    data_length: i32,
    sky_light_sent: bool,
    compressed_chunk_data: Vec<u8>,
    meta: Vec<ChunkMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkMeta {
    x: i32,
    z: i32,
    mask: u16,
    add_mask: u16,
}


struct ExternalLength<'a, T: 'a>(usize, &'a mut Vec<T>);

impl<'de, 'a, T> DeserializeSeed<'de> for ExternalLength<'a, T>
    where
        T: Deserialize<'de>,
{
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {

        struct ExtendVecVisitor<'a, T: 'a>(usize, &'a mut Vec<T>);
        impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
            where
                T: Deserialize<'de>,
        {
            type Value = ();
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an array of T")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
                where
                    A: SeqAccess<'de>,
            {
                let mut len = self.0;
                while len > 0 {
                    len -= 1;
                    self.1.push(seq.next_element()?.unwrap());
                }
                Ok(())
            }
        }
        deserializer.deserialize_tuple(self.0,ExtendVecVisitor(self.0, self.1))
    }
}

impl<'de> Deserialize<'de> for ChunkDataBulk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        struct BulkVisitor;
        impl<'de> Visitor<'de> for BulkVisitor {
            type Value = ChunkDataBulk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                formatter.write_str("a chunk bulk")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where
                A: SeqAccess<'de>, {

                let chunk_column_count: i16 = seq.next_element()?.unwrap();
                let data_length: i32 = seq.next_element()?.unwrap();
                let sky_light_sent: bool = seq.next_element()?.unwrap();

                let mut compressed_chunk_data: Vec<u8> = Vec::with_capacity(data_length as usize);
                seq.next_element_seed(ExternalLength(data_length as usize, &mut compressed_chunk_data))?.unwrap();

                let mut meta: Vec<ChunkMeta> = Vec::with_capacity(chunk_column_count as usize);
                seq.next_element_seed(ExternalLength(chunk_column_count as usize, &mut meta))?.unwrap();

                Ok(ChunkDataBulk {
                    chunk_column_count,
                    data_length,
                    sky_light_sent,
                    compressed_chunk_data,
                    meta,
                })
            }
        }
        let FIELDS: &'static [&'static str] = &["chunk_column_count",
            "data_length",
            "sky_light_sent",
            "compressed_chunk_data",
            "meta"];
        deserializer.deserialize_struct("unknown", FIELDS, BulkVisitor)
    }
}

