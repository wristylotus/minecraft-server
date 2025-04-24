use bytes::{BufMut, Bytes, BytesMut};
use std::fmt::Display;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum VarIntErr {
    #[error("VarInt more than 5 bytes")]
    TooLongError,
    #[error("Need to be at least 5 bytes")]
    TooSmallError,
}

pub struct VarInt;

impl VarInt {
    const MAX_LEN: usize = 5;
    pub async fn read_stream(stream: &mut TcpStream) -> anyhow::Result<i32> {
        let mut result = 0;
        let mut buf = [0u8; 1];

        for pos in 0..VarInt::MAX_LEN {
            stream.read_exact(&mut buf).await?;

            let value = (buf[0] & 0x7F) as i32;
            result |= value << (7 * pos);

            if buf[0] & 0x80 == 0 {
                break;
            }

            if pos + 1 == VarInt::MAX_LEN {
                return Err(VarIntErr::TooLongError.into());
            }
        }

        Ok(result)
    }

    pub fn read(buf: Bytes) -> anyhow::Result<(i32, Bytes)> {
        let mut result = 0;
        let mut bytes_read = 0;

        for pos in 0..VarInt::MAX_LEN {
            let next_byte = buf[pos];

            let value = (next_byte & 0x7F) as i32;
            result |= value << (7 * pos);

            if next_byte & 0x80 == 0 {
                bytes_read = pos + 1;
                break;
            }

            if pos + 1 == VarInt::MAX_LEN {
                return Err(VarIntErr::TooLongError.into());
            }
        }
        let buf = Bytes::copy_from_slice(&buf[bytes_read..]);

        Ok((result, buf))
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
}

#[derive(Debug)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl Display for HandshakeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl HandshakeState {
    fn from(num: i32) -> anyhow::Result<HandshakeState> {
        match num {
            1 => Ok(HandshakeState::Status),
            2 => Ok(HandshakeState::Login),
            3 => Ok(HandshakeState::Transfer),
            _ => anyhow::bail!("Unknown handshake state"),
        }
    }

    pub fn read(buf: Bytes) -> anyhow::Result<(Self, Bytes)> {
        let (state, buf) = VarInt::read(buf)?;

        Ok((HandshakeState::from(state)?, buf))
    }
}