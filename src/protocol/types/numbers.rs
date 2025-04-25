use bytes::{Buf, Bytes};

pub struct U16;

impl U16 {
    pub fn read(buf: &mut Bytes) -> anyhow::Result<u16> {
        let mut value = buf.split_to(2);
        Ok(value.get_u16())
    }
}
