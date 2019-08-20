use std::io::prelude::*;
use super::read::*;
use serde::de::{Visitor, DeserializeSeed};
use serde::Deserializer;
use nbt::de::Decoder;

pub struct MCProtoDeserializer<R: Read> {
    pub reader: R
}

impl<R: Read> MCProtoDeserializer<R> {
    pub fn new(r: R) -> MCProtoDeserializer<R> {
        MCProtoDeserializer {
            reader: r
        }
    }
}

impl<'de, 'a, R: Read> Deserializer<'de> for &'a mut MCProtoDeserializer<R> {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_bool(&mut self.reader) {
            Ok(b) => visitor.visit_bool(b),
            Err(e) => panic!("it's not bool {:?}", e) //TODO make normal error
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_i8(&mut self.reader) {
            Ok(b) => visitor.visit_i8(b),
            Err(_) => panic!("it's not i8") //TODO make normal error
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_i16(&mut self.reader) {
            Ok(b) => visitor.visit_i16(b),
            Err(_) => panic!("it's not i16") //TODO make normal error
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_i32(&mut self.reader) {
            Ok(b) => visitor.visit_i32(b),
            Err(_) => panic!("it's not i32") //TODO make normal error
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_i64(&mut self.reader) {
            Ok(b) => visitor.visit_i64(b),
            Err(_) => panic!("it's not i64") //TODO make normal error
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_u8(&mut self.reader) {
            Ok(b) => visitor.visit_u8(b),
            Err(e) => panic!("it's not u8, {}", e) //TODO make normal error
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_u16(&mut self.reader) {
            Ok(b) => visitor.visit_u16(b),
            Err(_) => panic!("it's not u16") //TODO make normal error
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_u32(&mut self.reader) {
            Ok(b) => visitor.visit_u32(b),
            Err(_) => panic!("it's not u32") //TODO make normal error
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_u64(&mut self.reader) {
            Ok(b) => visitor.visit_u64(b),
            Err(_) => panic!("it's not u64") //TODO make normal error
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_f32(&mut self.reader) {
            Ok(b) => visitor.visit_f32(b),
            Err(_) => panic!("it's not f32") //TODO make normal error
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_u128(&mut self.reader) {
            Ok(b) => visitor.visit_u128(b),
            Err(_) => panic!("it's not f32") //TODO make normal error
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_f64(&mut self.reader) {
            Ok(b) => visitor.visit_f64(b),
            Err(_) => panic!("it's not f64") //TODO make normal error
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("char"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a str"))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match read_String(&mut self.reader) {
            Ok(b) => {
                visitor.visit_string(b)
            }
            Err(e) => panic!("it's not string, {}", e) //TODO make normal error
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a bytes"))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a byte buf"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        let id = read_i16(&mut self.reader).unwrap();
        if id != -1i16 {
            // some hack. push back :D
            let mut prefix_data = Vec::new();
            crate::ser::write::write_i16(&id, &mut prefix_data);
            let mut de = MCProtoDeserializer
                { reader: Read::chain(prefix_data.as_slice(), &mut self.reader) };
            // hack end
            visitor.visit_some(&mut de)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a unit"))
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a unit struct"))
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        match name {
            "MCVARINT" => {
                let v = read_varint(&mut self.reader).unwrap();
                visitor.visit_i32(v)
            }
            "MCVARLONG" => {
                let v = read_varlong(&mut self.reader).unwrap();
                visitor.visit_i64(v)
            }
            "MCUBYTEPREFIXEDARRAY" => {
                let len = read_u8(&mut self.reader)?;
                visitor.visit_seq(Seq { len: len as usize, de: self })
            }
            "MCBYTEPREFIXEDARRAY" => {
                let len = read_i8(&mut self.reader)?;
                visitor.visit_seq(Seq { len: len as usize, de: self })
            }
            "MCSHORTPREFIXEDARRAY" => {
                let len = read_i16(&mut self.reader)?;
                visitor.visit_seq(Seq { len: len as usize, de: self })
            }
            "MCINTPREFIXEDARRAY" => {
                let len = read_i32(&mut self.reader)?;
                visitor.visit_seq(Seq { len: len as usize, de: self })
            }
            "MCVARINTPREFIXEDARRAY" => {
                let len = read_varint(&mut self.reader)?;
                visitor.visit_seq(Seq { len: len as usize, de: self })
            }
            "MCMETADATAENTRY" => {
                visitor.visit_seq(MetaDataSeq { de: self })
            }
            "NBT" => {
                let data_len = read_u16(&mut self.reader).unwrap();
                let mut data =  vec![0; data_len as usize];
                let readed = self.reader.read(&mut data).unwrap();
                visitor.visit_newtype_struct(&mut Decoder::new(&*data))
                    .map_err(|e| crate::error::Error::Serde(format!("NBT read error: {}", e.to_string())))
            }
            "GZIPNBT" => {
                let data_len = read_u16(&mut self.reader).unwrap();
                let mut data =  vec![0; data_len as usize];
                let readed = self.reader.read(&mut data).unwrap();
                visitor.visit_newtype_struct(
                    &mut Decoder::new(&mut flate2::read::GzDecoder::new(&*data)))
                    .map_err(|e| crate::error::Error::Serde(format!("NBT read error: {}", e.to_string())))
            }
            _ => Err(crate::error::Error::Serde(format!("Unknown new type {}", name)))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a unsized seq"))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_seq(Seq { de: self, len })
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a tuple struct"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        Err(crate::error::Error::UnrepresentableType("a map"))
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_enum(Enum { de: self })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }
}


struct Enum<'a, R: Read> {
    de: &'a mut MCProtoDeserializer<R>
}

impl<'a, 'de, R: Read> serde::de::EnumAccess<'de> for Enum<'a, R> {
    type Error = crate::error::Error;
    type Variant = Enum<'a, R>;

    fn variant_seed<V: serde::de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
        use serde::de::IntoDeserializer;
        let c: i32 = read_varint(&mut self.de.reader)?.into();
        let val: Result<_, Self::Error> = seed.deserialize((c as u32).into_deserializer());
        Ok((val?, self))
    }
}

impl<'a, 'de, R: Read> serde::de::VariantAccess<'de> for Enum<'a, R> {
    type Error = crate::error::Error;
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn newtype_variant_seed<T: serde::de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
        seed.deserialize(self.de)
    }
    fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        self.de.deserialize_tuple(len, visitor)
    }
    fn struct_variant<V: Visitor<'de>>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
        self.de.deserialize_struct("unknown", fields, visitor)
    }
}


pub struct MetaDataSeq<'a, R: Read> {
    de: &'a mut MCProtoDeserializer<R>
}

impl<'a, 'de, R: Read> serde::de::SeqAccess<'de> for MetaDataSeq<'a, R> {
    type Error = crate::error::Error;

    fn next_element_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        let mask: u8 = read_u8(&mut self.de.reader)?;
        if mask == 127 { return Ok(None); }
        let id: u8 = (mask & 0b11100000) >> 5;
        let data_type: u8 = mask & 0b00011111;
        let mut prefix_data = Vec::new();
        crate::ser::write::write_u8(&id, &mut prefix_data);
        crate::ser::write::write_varint(&(id as i32), &mut prefix_data);
        let mut de = MCProtoDeserializer
            { reader: Read::chain(prefix_data.as_slice(), &mut self.de.reader) };
        let value = serde::de::DeserializeSeed::deserialize(seed, &mut de)?;
        Ok(Some(value))
    }
}

pub struct Seq<'a, R: Read> {
    de: &'a mut MCProtoDeserializer<R>,
    len: usize,
}

impl<'a, 'de, R: Read> serde::de::SeqAccess<'de> for Seq<'a, R> {
    type Error = crate::error::Error;
    fn next_element_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        if self.len == 0 { return Ok(None); }
        self.len -= 1;
        let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.de)?;
        Ok(Some(value))
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}
