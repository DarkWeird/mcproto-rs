use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, SeqAccess, DeserializeOwned};
use serde::export::PhantomData;
use crate::ser::write::{write_varint, write_u8, write_i8, write_i16, write_i32};

#[derive(Debug)]
pub struct UBytePrefix<T>
    where T: DeserializeOwned + Serialize {
    elements: Vec<T>,
}

impl<T> UBytePrefix<T>
where T: DeserializeOwned + Serialize {
    pub fn new(elements: Vec<T>) -> Self {
        UBytePrefix { elements }
    }
}

#[derive(Debug)]
pub struct BytePrefix<T>
    where T: DeserializeOwned + Serialize {
    elements: Vec<T>,
}

impl<T> BytePrefix<T>
    where T: DeserializeOwned + Serialize {
    pub fn new(elements: Vec<T>) -> Self {
        BytePrefix { elements }
    }
}

#[derive(Debug)]
pub struct ShortPrefix<T>
    where T: DeserializeOwned + Serialize
{
    elements: Vec<T>,
}

impl<T> ShortPrefix<T>
    where T: DeserializeOwned + Serialize {
    pub fn new(elements: Vec<T>) -> Self {
        ShortPrefix { elements }
    }
}

#[derive(Debug)]
pub struct IntPrefix<T>
    where T: DeserializeOwned + Serialize {
    elements: Vec<T>,
}

impl< T> IntPrefix<T>
    where T: DeserializeOwned + Serialize {
    pub fn new(elements: Vec<T>) -> Self {
        IntPrefix { elements }
    }
}

#[derive(Debug)]
pub struct VarIntPrefix<T>
    where T: DeserializeOwned + Serialize {
    elements: Vec<T>,
}

impl<T> VarIntPrefix<T>
    where T: DeserializeOwned + Serialize {
    pub fn new(elements: Vec<T>) -> Self {
        VarIntPrefix { elements }
    }
}

impl<'de, T> Deserialize<'de> for UBytePrefix<T>
    where T: DeserializeOwned + Serialize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_newtype_struct("MCUBYTEPREFIXEDARRAY", VecVisitor {
            marker: PhantomData,
        }).map(|v| UBytePrefix::new(v))
    }
}

impl<'de, T> Deserialize<'de> for BytePrefix<T>
    where T: DeserializeOwned + Serialize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_newtype_struct("MCBYTEPREFIXEDARRAY", VecVisitor {
            marker: PhantomData,
        }).map(|v| BytePrefix::new(v))
    }
}

impl<'de, T> Deserialize<'de> for ShortPrefix<T>
    where T: DeserializeOwned + Serialize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_newtype_struct("MCSHORTPREFIXEDARRAY", VecVisitor {
            marker: PhantomData,
        }).map(|v| ShortPrefix::new(v))
    }
}

impl<'de, T> Deserialize<'de> for IntPrefix<T>
    where T: DeserializeOwned + Serialize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_newtype_struct("MCINTPREFIXEDARRAY", VecVisitor {
            marker: PhantomData,
        }).map(|v| IntPrefix::new(v))
    }
}

impl<'de, T> Deserialize<'de> for VarIntPrefix<T>
    where T: DeserializeOwned + Serialize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_newtype_struct("MCVARINTPREFIXEDARRAY", VecVisitor {
            marker: PhantomData,
        }).map(|v| VarIntPrefix::new(v))
    }
}

struct VecVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for VecVisitor<T>
    where
        T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(seq.size_hint().unwrap());

        while let Some(value) = (seq.next_element())? {
            values.push(value);
        }

        Ok(values)
    }
}

impl<T> Serialize for UBytePrefix<T>
    where T: DeserializeOwned + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_u8(&(self.elements.len() as u8), &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}

impl<T> Serialize for BytePrefix<T>
    where T: DeserializeOwned + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_i8(&(self.elements.len() as i8), &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}

impl<T> Serialize for ShortPrefix<T>
    where T: DeserializeOwned + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_i16(&(self.elements.len() as i16), &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}

impl<T> Serialize for IntPrefix<T>
    where T: DeserializeOwned + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_i32(&(self.elements.len() as i32), &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}

impl<'de, T> Serialize for VarIntPrefix<T>
    where T: DeserializeOwned + Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut bytes: Vec<u8> = Vec::new();
        write_i32(&(self.elements.len() as i32), &mut bytes);
        serializer.serialize_bytes(bytes.as_slice())
    }
}
