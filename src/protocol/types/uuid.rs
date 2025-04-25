use bytes::{Buf, Bytes};
use uuid::Uuid;

pub struct UUID;

impl UUID {
    pub fn read(buf: &mut Bytes) -> anyhow::Result<Uuid> {
        let mut value = buf.split_to(16);
        Ok(Uuid::from_u128(value.get_u128()))
    }
}
