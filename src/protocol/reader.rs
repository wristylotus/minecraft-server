use crate::protocol::packet::Packet;
use crate::protocol::types::{MCString, ReadBuffer, VarInt};
use bytes::Bytes;
use tokio::net::tcp::ReadHalf;
use uuid::Uuid;

pub struct ProtocolReader<'a> {
    stream: ReadHalf<'a>,
    packet_id: VarInt,
    packet_length: usize,
    data: Bytes,
}

impl<'a> ProtocolReader<'a> {
    pub fn from_stream(stream: ReadHalf<'a>) -> anyhow::Result<ProtocolReader<'a>> {
        let reader = ProtocolReader {
            stream,
            packet_id: VarInt::default(),
            packet_length: 0,
            data: Bytes::default(),
        };
        Ok(reader)
    }

    pub async fn packet_id(&mut self) -> anyhow::Result<VarInt> {
        self.check_for_packet_end().await?;
        Ok(self.packet_id)
    }

    pub async fn packet_length(&mut self) -> anyhow::Result<usize> {
        self.check_for_packet_end().await?;
        Ok(self.packet_length)
    }

    pub async fn read_varint(&mut self) -> anyhow::Result<VarInt> {
        self.check_for_packet_end().await?;
        Ok(VarInt::read(&mut self.data)?)
    }

    pub async fn read_i8(&mut self) -> anyhow::Result<i8> {
        self.check_for_packet_end().await?;
        Ok(i8::read(&mut self.data)?)
    }

    pub async fn read_u8(&mut self) -> anyhow::Result<u8> {
        self.check_for_packet_end().await?;
        Ok(u8::read(&mut self.data)?)
    }

    pub async fn read_bool(&mut self) -> anyhow::Result<bool> {
        self.check_for_packet_end().await?;
        Ok(bool::read(&mut self.data)?)
    }

    pub async fn read_u16(&mut self) -> anyhow::Result<u16> {
        self.check_for_packet_end().await?;
        Ok(u16::read(&mut self.data)?)
    }

    pub async fn read_i64(&mut self) -> anyhow::Result<i64> {
        self.check_for_packet_end().await?;
        Ok(i64::read(&mut self.data)?)
    }

    pub async fn read_string(&mut self) -> anyhow::Result<MCString> {
        self.check_for_packet_end().await?;
        Ok(MCString::read(&mut self.data)?)
    }

    pub async fn read_uuid(&mut self) -> anyhow::Result<Uuid> {
        self.check_for_packet_end().await?;
        Ok(Uuid::read(&mut self.data)?)
    }

    async fn check_for_packet_end(&mut self) -> anyhow::Result<()> {
        if self.data.is_empty() {
            self.load_next_packet().await?;
        }
        Ok(())
    }

    async fn load_next_packet(&mut self) -> anyhow::Result<()> {
        self.stream.readable().await?;

        let packet = Packet::read(&mut self.stream).await?;
        self.packet_id = packet.id;
        self.packet_length = packet.length;
        self.data = packet.data;

        Ok(())
    }
}
