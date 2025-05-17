use crate::protocol::packet::Packet;
use crate::protocol::types::{I64, MCString, UUID, VarInt, NBTString};
use bytes::BytesMut;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use tokio::net::TcpStream;
use uuid::Uuid;

pub struct ProtocolWriter {
    stream: Rc<RefCell<TcpStream>>,
    buf: Cell<BytesMut>,
}

impl ProtocolWriter {
    pub fn from_stream(stream: Rc<RefCell<TcpStream>>) -> anyhow::Result<Self> {
        let reader = ProtocolWriter {
            stream,
            buf: Cell::new(BytesMut::new()),
        };

        Ok(reader)
    }

    pub fn write_varint(&mut self, value: i32) -> anyhow::Result<()> {
        VarInt::write(value, self.buf.get_mut())
    }

    pub fn write_i64(&mut self, value: i64) -> anyhow::Result<()> {
        I64::write(value, self.buf.get_mut())
    }

    pub fn write_string(&mut self, value: &str) -> anyhow::Result<()> {
        MCString::write(value, self.buf.get_mut())
    }

    pub fn write_nbt_string(&mut self, value: &str) -> anyhow::Result<()> {
        NBTString::write(value, self.buf.get_mut())
    }

    pub fn write_uuid(&mut self, value: Uuid) -> anyhow::Result<()> {
        UUID::write(value, self.buf.get_mut())
    }

    pub async fn send_packet(&mut self, id: i32) -> anyhow::Result<()> {
        let packet = Packet::new(id, self.buf.take().into());
        self.buf.set(BytesMut::new());

        packet.send(&mut self.stream.borrow_mut()).await
    }
}
