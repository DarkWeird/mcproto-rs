use bytes::IntoBuf;
use bytes::{Buf, BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use serde_mcproto::de::MCProtoDeserializer;
use serde_mcproto::error::Error;
use serde_mcproto::ser::MCProtoSerializer;
use serde_mcproto::write_varint;
use std::marker::PhantomData;
use tokio::codec::{Decoder, Encoder};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MCProtoCodec<T, R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    _ghost_t: PhantomData<T>,
    _ghost_r: PhantomData<R>,
}

impl<T, R> MCProtoCodec<T, R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    pub fn new() -> Self {
        MCProtoCodec {
            _ghost_t: PhantomData,
            _ghost_r: PhantomData,
        }
    }
}

impl<T, R> Decoder for MCProtoCodec<T, R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    type Item = R;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() > 0 {
            let (len, consumed_bytes) = read_varint(src).unwrap();
            if src.len() - consumed_bytes < len as usize {
                Ok(None)
            } else {
                src.advance(consumed_bytes);
                let result = src.split_to(len as usize);
                serde_mcproto::de::deserialize(&mut MCProtoDeserializer {
                    reader: result.into_buf().reader(),
                })
                .map_err(Into::into)
                .map(|e| Some(e))
            }
        } else {
            Ok(None)
        }
    }
}

impl<T, R> Encoder for MCProtoCodec<T, R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    type Item = T;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut buffer = Vec::new();
        serde_mcproto::ser::serialize(
            &item,
            &mut MCProtoSerializer {
                writer: &mut buffer,
            },
        )?;
        let len = buffer.len() as i32;
        let mut varint_length = Vec::new();
        write_varint(&len, &mut varint_length).unwrap();
        dst.reserve(varint_length.len() + buffer.len());
        dst.put(varint_length);
        dst.put(buffer);
        Ok(())
    }
}

pub fn read_varint(bytes: &mut BytesMut) -> Result<(i32, usize), Error> {
    let mut result = 0;
    let msb: u8 = 0b10000000;
    let mask: u8 = !msb;

    for i in 0..5 {
        let read: &u8 = bytes.get(i).unwrap();
        result |= ((read & mask) as i32) << (7 * i as i32);

        /* The last (5th) byte is only allowed to have the 4 LSB set */
        if i == 4 && (read & 0xf0 != 0) {
            return Err(Error::Serde(format!(
                "VarInt is too long, last byte: {}",
                read
            )));
        }

        if (read & msb) == 0 {
            return Ok((result, i + 1));
        }
    }

    panic!("read_varint reached end of loop, which should not be possible");
}
