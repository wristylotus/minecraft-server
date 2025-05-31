use crate::protocol::packet::Packet;
use crate::protocol::types::{VarInt, WriteBuffer};
use bytes::BytesMut;
use tokio::net::tcp::WriteHalf;

pub struct ProtocolWriter<'a> {
    stream: WriteHalf<'a>,
    buf: BytesMut,
}

impl<'a> ProtocolWriter<'a> {
    pub fn from_stream(stream: WriteHalf<'a>) -> anyhow::Result<ProtocolWriter<'a>> {
        let reader = ProtocolWriter {
            stream,
            buf: BytesMut::default(),
        };

        Ok(reader)
    }

    pub fn write<T>(&mut self, value: T) -> anyhow::Result<()>
    where
        T: WriteBuffer,
    {
        value.write(&mut self.buf)
    }

    pub async fn send_packet(&mut self, id: VarInt) -> anyhow::Result<()> {
        let packet = Packet::new(id, std::mem::take(&mut self.buf).into());

        packet.send(&mut self.stream).await
    }
}
