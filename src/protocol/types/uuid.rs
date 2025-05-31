use crate::protocol::types::{ReadBuffer, WriteBuffer};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

impl ReadBuffer for Uuid {
    fn read(buf: &mut Bytes) -> anyhow::Result<Uuid> {
        let mut value = buf.split_to(size_of::<u128>());
        Ok(Uuid::from_u128(value.get_u128()))
    }
}

impl WriteBuffer for Uuid {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u128(self.as_u128());
        Ok(())
    }
}
