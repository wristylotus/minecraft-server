use crate::protocol::packet::Packet;
use crate::protocol::types::{MCString, NBTString, VarInt, I64, UUID};
use bytes::BytesMut;
use tokio::net::tcp::WriteHalf;
use uuid::Uuid;

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

    pub fn write_varint(&mut self, value: i32) -> anyhow::Result<()> {
        VarInt::write(value, &mut self.buf)
    }

    pub fn write_i64(&mut self, value: i64) -> anyhow::Result<()> {
        I64::write(value, &mut self.buf)
    }

    pub fn write_string(&mut self, value: &str) -> anyhow::Result<()> {
        MCString::write(value, &mut self.buf)
    }

    pub fn write_nbt_string(&mut self, value: &str) -> anyhow::Result<()> {
        NBTString::write(value, &mut self.buf)
    }

    pub fn write_uuid(&mut self, value: Uuid) -> anyhow::Result<()> {
        UUID::write(value, &mut self.buf)
    }

    pub async fn send_packet(&mut self, id: i32) -> anyhow::Result<()> {
        let packet = Packet::new(id, std::mem::take(&mut self.buf).into());

        packet.send(&mut self.stream).await
    }
}
