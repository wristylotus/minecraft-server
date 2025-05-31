use crate::protocol::types::{ReadBuffer, WriteBuffer};
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::{Display, Formatter, UpperHex};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::ReadHalf;

#[derive(Error, Debug)]
pub enum VarIntErr {
    #[error("VarInt more than 5 bytes")]
    TooLongError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(i32);

impl VarInt {
    pub const MAX_LEN: usize = 5;

    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub async fn read_stream(stream: &mut ReadHalf<'_>) -> anyhow::Result<VarInt> {
        let mut result = 0;
        let mut buf = [0u8; 1];

        for pos in 0..VarInt::MAX_LEN {
            stream.read_exact(&mut buf).await?;

            let value = (buf[0] & 0x7F) as i32;
            result |= value << (7 * pos);

            if buf[0] & 0x80 == 0 {
                return Ok(VarInt(result));
            }
        }

        Err(VarIntErr::TooLongError.into())
    }

    pub fn size(&self) -> usize {
        Self::sizeof(self.0)
    }

    #[inline]
    pub fn sizeof(value: i32) -> usize {
        match value {
            0 => 1,
            n => (31 - n.leading_zeros() as usize) / 7 + 1,
        }
    }
}

impl ReadBuffer for VarInt {
    fn read(buf: &mut Bytes) -> anyhow::Result<VarInt> {
        let mut result = 0;

        for pos in 0..VarInt::MAX_LEN {
            let next_byte = buf[pos];

            let value = (next_byte & 0x7F) as i32;
            result |= value << (7 * pos);

            if next_byte & 0x80 == 0 {
                drop(buf.split_to(pos + 1));
                return Ok(VarInt(result));
            }
        }

        Err(VarIntErr::TooLongError.into())
    }
}

impl WriteBuffer for VarInt {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        let mut value: i32 = self.into();
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
}

impl Default for VarInt {
    fn default() -> Self {
        Self(0)
    }
}

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl PartialEq<i32> for VarInt {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl Display for VarInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UpperHex for VarInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}
