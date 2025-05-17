use crate::protocol::types::U16;
use bytes::{BufMut, BytesMut};
pub struct NBTString;

impl NBTString {
    const ID: u8 = 8;
    pub fn write(value: &str, buf: &mut BytesMut) -> anyhow::Result<()> {
        let value = cesu8::to_java_cesu8(value);
        buf.put_slice(&[Self::ID]);
        U16::write(value.len() as u16, buf)?;
        buf.put_slice(&value);
        Ok(())
    }
}
