use crate::types::VarInt;
use bytes::{Bytes, BytesMut};
use std::fmt::Write;

pub struct MCString;

impl MCString {
    const MAX_LENGTH: usize = 32767;
    pub fn read(buf: Bytes) -> anyhow::Result<(String, Bytes)> {
        let (length, buf) = VarInt::read(buf)?;

        let data = Vec::from(&buf[..length as usize]);
        let buf = Bytes::copy_from_slice(&buf[length as usize..]);

        Ok((String::from_utf8(data)?, buf))
    }

    pub fn write(value: &str, buf: &mut BytesMut) -> anyhow::Result<()> {
        if value.len() > Self::MAX_LENGTH {
            return Err(anyhow::anyhow!("String too long"));
        }

        VarInt::write(value.len() as i32, buf)?;
        buf.write_str(value)?;

        Ok(())
    }
}
