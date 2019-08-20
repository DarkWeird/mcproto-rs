use bytes::{Buf, Bytes, BytesMut, IntoBuf};
use futures::{Poll, Sink, StartSend, Stream};
use serde::{Deserialize, Serialize};
use tokio_serde::{Deserializer, FramedRead, FramedWrite, Serializer};

use serde_mcproto::de::MCProtoDeserializer;
use serde_mcproto::ser::MCProtoSerializer;
use std::marker::PhantomData;

pub struct MCProto<T> {
    ghost: PhantomData<T>,
}

pub struct ReadMCProto<T, U> {
    inner: FramedRead<T, U, MCProto<U>>,
}

pub struct WriteMCProto<T: Sink, U> {
    inner: FramedWrite<T, U, MCProto<U>>,
}

impl<T, U> ReadMCProto<T, U>
where
    T: Stream,
    T::Error: From<serde_mcproto::error::Error>,
    for<'a> U: Deserialize<'a>,
    BytesMut: From<T::Item>,
{
    pub fn new(inner: T) -> ReadMCProto<T, U> {
        let mcproto = MCProto { ghost: PhantomData };
        ReadMCProto {
            inner: FramedRead::new(inner, mcproto),
        }
    }
}

impl<T, U> ReadMCProto<T, U> {
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T, U> ReadMCProto<T, U>
where
    T: Stream,
    T::Error: From<serde_mcproto::error::Error>,
    for<'a> U: Deserialize<'a>,
    BytesMut: From<T::Item>,
{
    pub fn change_state<N>(self) -> ReadMCProto<T, N>
    where
        for<'a> N: Deserialize<'a>,
    {
        ReadMCProto::new(self.into_inner())
    }
}

impl<T: Stream, U> Stream for ReadMCProto<T, U>
where
    T: Stream,
    T::Error: From<serde_mcproto::error::Error>,
    for<'a> U: Deserialize<'a>,
    BytesMut: From<T::Item>,
{
    type Item = U;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}

impl<T, U> Sink for ReadMCProto<T, U>
where
    T: Sink,
{
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: T::SinkItem) -> StartSend<T::SinkItem, T::SinkError> {
        self.get_mut().start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.get_mut().poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.get_mut().close()
    }
}

impl<T, U> WriteMCProto<T, U>
where
    T: Sink<SinkItem = Bytes>,
    T::SinkError: From<serde_mcproto::error::Error>,
    U: Serialize,
{
    pub fn new(inner: T) -> WriteMCProto<T, U> {
        let mcproto = MCProto { ghost: PhantomData };
        WriteMCProto {
            inner: FramedWrite::new(inner, mcproto),
        }
    }
}

impl<T: Sink, U> WriteMCProto<T, U> {
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T, U> Sink for WriteMCProto<T, U>
where
    T: Sink<SinkItem = Bytes>,
    T::SinkError: From<serde_mcproto::error::Error>,
    U: Serialize,
{
    type SinkItem = U;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: U) -> StartSend<U, T::SinkError> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.inner.close()
    }
}

impl<T, U> Stream for WriteMCProto<T, U>
where
    T: Stream + Sink,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<T::Item>, T::Error> {
        self.get_mut().poll()
    }
}

impl<T> Deserializer<T> for MCProto<T>
where
    for<'a> T: Deserialize<'a>,
{
    type Error = serde_mcproto::error::Error;

    fn deserialize(&mut self, src: &BytesMut) -> Result<T, Self::Error> {
        serde_mcproto::de::deserialize(&mut MCProtoDeserializer {
            reader: src.into_buf().reader(),
        })
    }
}

impl<T: Serialize> Serializer<T> for MCProto<T> {
    type Error = serde_mcproto::error::Error;

    fn serialize(&mut self, item: &T) -> Result<Bytes, Self::Error> {
        let mut buffer = Vec::new();
        serde_mcproto::ser::serialize(
            item,
            &mut MCProtoSerializer {
                writer: &mut buffer,
            },
        )
        .map(move |_| buffer.into())
    }
}
