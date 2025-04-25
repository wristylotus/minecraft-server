use crate::protocol::types::VarInt;
use bytes::{Bytes, BytesMut};
use std::fmt::Write;

pub struct MCString;

impl MCString {
    const MAX_LENGTH: usize = 32767;
    pub fn read(buf: &mut Bytes) -> anyhow::Result<String> {
        let length = VarInt::read(buf)? as usize;

        let value = buf.split_to(length).to_vec();

        Ok(String::from_utf8(value)?)
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
