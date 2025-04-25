use crate::protocol::packet::Packet;
use crate::protocol::types::{MCString, U16, UUID, VarInt};
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
    pub async fn from_stream(stream: Rc<RefCell<TcpStream>>) -> anyhow::Result<Self> {
        let reader = ProtocolReader {
            stream,
            packet_id: Cell::new(-1),
            packet_length: Cell::new(0),
            data: RefCell::new(Bytes::new()),
        };
        reader.load_next_packet().await?;

        Ok(reader)
    }

    pub fn packet_id(&self) -> anyhow::Result<i32> {
        Ok(self.packet_id.get())
    }

    pub fn packet_length(&self) -> anyhow::Result<usize> {
        Ok(self.packet_length.get())
    }

    pub async fn read_varint(&self) -> anyhow::Result<i32> {
        self.check_for_packet_end().await?;
        let value = VarInt::read(&mut self.data.borrow_mut())?;

        Ok(value)
    }

    pub async fn read_u16(&self) -> anyhow::Result<u16> {
        self.check_for_packet_end().await?;
        let value = U16::read(&mut self.data.borrow_mut())?;

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
        let packet = Packet::read(&mut self.stream.borrow_mut()).await?;
        self.packet_id.set(packet.id);
        self.packet_length.set(packet.length);
        drop(self.data.replace(packet.data));
        Ok(())
    }
}
