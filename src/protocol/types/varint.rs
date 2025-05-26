use bytes::{BufMut, Bytes, BytesMut};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::ReadHalf;

#[derive(Error, Debug)]
pub enum VarIntErr {
    #[error("VarInt more than 5 bytes")]
    TooLongError,
}

pub struct VarInt;

impl VarInt {
    pub const MAX_LEN: usize = 5;
    pub async fn read_stream(stream: &mut ReadHalf<'_>) -> anyhow::Result<i32> {
        let mut result = 0;
        let mut buf = [0u8; 1];

        for pos in 0..VarInt::MAX_LEN {
            stream.read_exact(&mut buf).await?;

            let value = (buf[0] & 0x7F) as i32;
            result |= value << (7 * pos);

            if buf[0] & 0x80 == 0 {
                return Ok(result);
            }
        }

        Err(VarIntErr::TooLongError.into())
    }

    pub fn read(buf: &mut Bytes) -> anyhow::Result<i32> {
        let mut result = 0;

        for pos in 0..VarInt::MAX_LEN {
            let next_byte = buf[pos];

            let value = (next_byte & 0x7F) as i32;
            result |= value << (7 * pos);

            if next_byte & 0x80 == 0 {
                drop(buf.split_to(pos + 1));
                return Ok(result);
            }
        }

        Err(VarIntErr::TooLongError.into())
    }

    pub fn write(mut value: i32, buf: &mut BytesMut) -> anyhow::Result<()> {
        loop {
            let temp = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                buf.put_u8(temp | 0x80);
            } else {
                buf.put_u8(temp);
                break;
            }
        }
        Ok(())
    }

    #[inline]
    pub fn sizeof(value: i32) -> usize {
        match value {
            0 => 1,
            n => (31 - n.leading_zeros() as usize) / 7 + 1,
        }
    }
}
