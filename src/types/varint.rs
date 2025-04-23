use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum VarIntErr {
    #[error("VarInt more than 5 bytes")]
    TooLongError,
}

pub struct VarInt;

impl VarInt {
    pub async fn read(stream: &mut TcpStream) -> anyhow::Result<i32> {
        let mut result = 0;
        let mut buf = [0u8; 1];

        for byte_num in 1..=5 {
            stream.read_exact(&mut buf).await?;

            let value = (buf[0] & 0x7F) as i32;
            result |= value << (7 * (byte_num - 1));

            if buf[0] & 0x80 == 0 {
                break;
            }

            if byte_num == 5 {
                return Err(VarIntErr::TooLongError.into());
            }
        }
        Ok(result)
    }
}
