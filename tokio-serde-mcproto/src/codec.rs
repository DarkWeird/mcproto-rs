use bytes::{Buf, Bytes, BytesMut, IntoBuf, BufMut};
use tokio::codec::{Decoder, Encoder};
use std::io::{BufRead, Read};
use serde_mcproto::error::Error;
use serde_mcproto::write_varint;

pub struct MCProtoCodec;

impl Decoder for MCProtoCodec {
    type Item = BytesMut;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() > 0 {
            let (len, consumed_bytes) = read_varint(src).unwrap();
            src.advance(consumed_bytes);
            let result = src.split_to(len as usize);
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for MCProtoCodec {
    type Item = Bytes;
    type Error = std::io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = item.len() as i32;
        println!("encoding");
        let mut varint_length = Vec::new();
        write_varint(&len, &mut varint_length).unwrap();

        dst.reserve(varint_length.len() + item.len());
        dst.put(varint_length);
        dst.put(item);
        println!("encoded {:X?}", dst);
        Ok(())
    }
}


pub fn read_varint(bytes: &mut BytesMut) -> Result<(i32, usize), Error> {
    let mut result = 0;
    let msb: u8 = 0b10000000;
    let mask: u8 = !msb;

    for i in 0..5 {
        let read: &u8 = bytes.get(i).unwrap();
        dbg!(read);
        result |= ((read & mask) as i32) << (7 * i as i32);

        /* The last (5th) byte is only allowed to have the 4 LSB set */
        if i == 4 && (read & 0xf0 != 0) {
            return Err(Error::Serde(format!("VarInt is too long, last byte: {}", read)));
        }

        if (read & msb) == 0 {
            return Ok((result, i + 1));
        }
    }

    panic!("read_varint reached end of loop, which should not be possible");
}
