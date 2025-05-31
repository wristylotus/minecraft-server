use crate::protocol::types::{ReadBuffer, VarInt, WriteBuffer};
use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};

pub struct Packet {
    pub length: usize,
    pub id: VarInt,
    pub data: Bytes,
}

impl Packet {
    pub fn new(id: VarInt, data: Bytes) -> Self {
        let length = id.size() + data.len();

        Packet { length, id, data }
    }

    pub async fn read(stream: &mut ReadHalf<'_>) -> anyhow::Result<Self> {
        let length = VarInt::read_stream(stream).await?.into();

        let mut data = BytesMut::zeroed(length);
        stream.read_exact(&mut data).await?;

        let mut data: Bytes = data.into();
        let id = VarInt::read(&mut data)?;

        Ok(Packet { length, id, data })
    }

    pub async fn send(self, stream: &mut WriteHalf<'_>) -> anyhow::Result<()> {
        let mut packet_buf = BytesMut::with_capacity(self.length + VarInt::MAX_LEN);

        // Write length of the packet
        VarInt::write((self.length as i32).into(), &mut packet_buf)?;
        // Write ID of the packet
        VarInt::write(self.id, &mut packet_buf)?;
        // Write the packet data
        packet_buf.put_slice(&self.data);
        // Send the packet
        stream.write_all(&packet_buf).await?;
        stream.flush().await?;

        Ok(())
    }
}
