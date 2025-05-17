use crate::protocol::packet::Packet;
use crate::protocol::types::{I8, I64, MCString, U16, UUID, VarInt, U8};
use bytes::Bytes;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use tokio::net::TcpStream;
use uuid::Uuid;

pub struct ProtocolReader {
    stream: Rc<RefCell<TcpStream>>,
    packet_id: Cell<i32>,
    packet_length: Cell<usize>,
    data: RefCell<Bytes>,
}

impl ProtocolReader {
    pub fn from_stream(stream: Rc<RefCell<TcpStream>>) -> anyhow::Result<Self> {
        let reader = ProtocolReader {
            stream,
            packet_id: Cell::new(-1),
            packet_length: Cell::new(0),
            data: RefCell::new(Bytes::new()),
        };

        Ok(reader)
    }

    pub async fn packet_id(&self) -> anyhow::Result<i32> {
        self.check_for_packet_end().await?;
        Ok(self.packet_id.get())
    }

    pub async fn packet_length(&self) -> anyhow::Result<usize> {
        self.check_for_packet_end().await?;
        Ok(self.packet_length.get())
    }

    pub async fn read_varint(&self) -> anyhow::Result<i32> {
        self.check_for_packet_end().await?;
        let value = VarInt::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_i8(&self) -> anyhow::Result<i8> {
        self.check_for_packet_end().await?;
        let value = I8::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_u8(&self) -> anyhow::Result<u8> {
        self.check_for_packet_end().await?;
        let value = U8::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_bool(&self) -> anyhow::Result<bool> {
        self.check_for_packet_end().await?;
        let value = I8::read(&mut self.data.borrow_mut())?;

        Ok(if value == 0 { false } else { true })
    }

    pub async fn read_u16(&self) -> anyhow::Result<u16> {
        self.check_for_packet_end().await?;
        let value = U16::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_i64(&self) -> anyhow::Result<i64> {
        self.check_for_packet_end().await?;
        let value = I64::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_string(&self) -> anyhow::Result<String> {
        self.check_for_packet_end().await?;
        let value = MCString::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_uuid(&self) -> anyhow::Result<Uuid> {
        self.check_for_packet_end().await?;
        let value = UUID::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    async fn check_for_packet_end(&self) -> anyhow::Result<()> {
        if self.data.borrow().is_empty() {
            self.load_next_packet().await?;
        }
        Ok(())
    }

    async fn load_next_packet(&self) -> anyhow::Result<()> {
        self.stream.borrow().readable().await?;

        let packet = Packet::read(&mut self.stream.borrow_mut()).await?;
        self.packet_id.set(packet.id);
        self.packet_length.set(packet.length);
        drop(self.data.replace(packet.data));

        Ok(())
    }
}
