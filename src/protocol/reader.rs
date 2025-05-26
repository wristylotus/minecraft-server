use crate::protocol::packet::Packet;
use crate::protocol::types::{MCString, VarInt, I64, I8, U16, U8, UUID};
use bytes::Bytes;
use std::cell::{Cell, RefCell};
use tokio::net::tcp::ReadHalf;
use uuid::Uuid;

pub struct ProtocolReader<'a> {
    stream: ReadHalf<'a>,
    packet_id: Cell<i32>,
    packet_length: Cell<usize>,
    data: RefCell<Bytes>,
}

impl<'a> ProtocolReader<'a> {
    pub fn from_stream(stream: ReadHalf<'a>) -> anyhow::Result<ProtocolReader<'a>> {
        let reader = ProtocolReader {
            stream,
            packet_id: Cell::new(-1),
            packet_length: Cell::new(0),
            data: RefCell::new(Bytes::new()),
        };

        Ok(reader)
    }

    pub async fn packet_id(&mut self) -> anyhow::Result<i32> {
        self.check_for_packet_end().await?;
        Ok(self.packet_id.get())
    }

    pub async fn packet_length(&mut self) -> anyhow::Result<usize> {
        self.check_for_packet_end().await?;
        Ok(self.packet_length.get())
    }

    pub async fn read_varint(&mut self) -> anyhow::Result<i32> {
        self.check_for_packet_end().await?;
        let value = VarInt::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_i8(&mut self) -> anyhow::Result<i8> {
        self.check_for_packet_end().await?;
        let value = I8::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_u8(&mut self) -> anyhow::Result<u8> {
        self.check_for_packet_end().await?;
        let value = U8::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_bool(&mut self) -> anyhow::Result<bool> {
        self.check_for_packet_end().await?;
        let value = I8::read(&mut self.data.borrow_mut())?;

        Ok(if value == 0 { false } else { true })
    }

    pub async fn read_u16(&mut self) -> anyhow::Result<u16> {
        self.check_for_packet_end().await?;
        let value = U16::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_i64(&mut self) -> anyhow::Result<i64> {
        self.check_for_packet_end().await?;
        let value = I64::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_string(&mut self) -> anyhow::Result<String> {
        self.check_for_packet_end().await?;
        let value = MCString::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_uuid(&mut self) -> anyhow::Result<Uuid> {
        self.check_for_packet_end().await?;
        let value = UUID::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    async fn check_for_packet_end(&mut self) -> anyhow::Result<()> {
        if self.data.borrow().is_empty() {
            self.load_next_packet().await?;
        }
        Ok(())
    }

    async fn load_next_packet(&mut self) -> anyhow::Result<()> {
        self.stream.readable().await?;

        let packet = Packet::read(&mut self.stream).await?;
        self.packet_id.set(packet.id);
        self.packet_length.set(packet.length);
        drop(self.data.replace(packet.data));

        Ok(())
    }
}
