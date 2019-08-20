use std::io::prelude::*;
use serde::{Serialize, ser, serde_if_integer128};
use std::fmt::Display;
use super::write::*;


pub struct MCProtoSerializer<W: Write> {
    pub writer: W
}

impl<W: Write> MCProtoSerializer<W> {
    /// Creates a new Serializer with the given `Write`r.
    pub fn new(w: W) -> MCProtoSerializer<W> {
        MCProtoSerializer {
            writer: w
        }
    }
}

impl<'a, W: Write> serde::Serializer for &'a mut MCProtoSerializer<W> {
    type Ok = ();
    type Error = crate::error::Error;
    type SerializeSeq = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write_bool(&v, &mut self.writer)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_i8(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_i16(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_i32(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_i64(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_u8(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_u16(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_u32(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_u64(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_f32(&v, &mut self.writer);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_f64(&v, &mut self.writer);
        Ok(())
    }

    serde_if_integer128! {

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        write_u128(&v, &mut self.writer);
        Ok(())
    }

    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        write_String(&val, &mut self.writer)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write(value);
        Ok(())//TODO handle
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write(&[0xff]);
        Ok(())//TODO handle
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        write_varint(&(variant_index as i32), &mut self.writer)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        write_varint(&(variant_index as i32), &mut self.writer);
        Ok(Compound { ser: self })
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Display {
        unimplemented!()
    }
}

pub struct Compound<'a, W: 'a + Write> {
    ser: &'a mut MCProtoSerializer<W>,
}

impl<'a, W> serde::ser::SerializeStruct for Compound<'a, W>
    where
        W: Write
{
    type Ok = ();
    type Error = crate::error::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for Compound<'a, W>
    where
        W: Write
{
    type Ok = ();
    type Error = crate::error::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
