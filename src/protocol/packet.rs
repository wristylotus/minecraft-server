use crate::protocol::types::VarInt;
use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};

pub struct Packet {
    pub length: usize,
    pub id: i32,
    pub data: Bytes,
}

impl Packet {
    pub fn new(id: i32, data: Bytes) -> Self {
        let length = VarInt::sizeof(id) + data.len();

        Packet { length, id, data }
    }

    pub async fn read(stream: &mut ReadHalf<'_>) -> anyhow::Result<Self> {
        let length = VarInt::read_stream(stream).await? as usize;

        let mut data = BytesMut::zeroed(length);
        stream.read_exact(&mut data).await?;

        let mut data: Bytes = data.into();
        let id = VarInt::read(&mut data)?;

        Ok(Packet { length, id, data })
    }

    pub async fn send(&self, stream: &mut WriteHalf<'_>) -> anyhow::Result<()> {
        let mut packet_buf = BytesMut::with_capacity(self.length + VarInt::MAX_LEN);

        // Write length of the packet
        VarInt::write(self.length as i32, &mut packet_buf)?;
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
