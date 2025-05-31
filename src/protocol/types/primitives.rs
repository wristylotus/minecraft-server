use crate::protocol::types::{ReadBuffer, WriteBuffer};
use bytes::{Buf, BufMut, Bytes, BytesMut};

impl ReadBuffer for i8 {
    fn read(buf: &mut Bytes) -> anyhow::Result<i8> {
        let mut value = buf.split_to(size_of::<u8>());
        Ok(value.get_i8())
    }
}

impl WriteBuffer for i8 {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_i8(self);
        Ok(())
    }
}

impl ReadBuffer for u8 {
    fn read(buf: &mut Bytes) -> anyhow::Result<u8> {
        let mut value = buf.split_to(size_of::<u8>());
        Ok(value.get_u8())
    }
}

impl WriteBuffer for u8 {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u8(self);
        Ok(())
    }
}

impl ReadBuffer for u16 {
    fn read(buf: &mut Bytes) -> anyhow::Result<u16> {
        let mut value = buf.split_to(size_of::<u16>());
        Ok(value.get_u16())
    }
}

impl WriteBuffer for u16 {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u16(self);
        Ok(())
    }
}

impl ReadBuffer for i64 {
    fn read(buf: &mut Bytes) -> anyhow::Result<i64> {
        let mut value = buf.split_to(size_of::<i64>());
        Ok(value.get_i64())
    }
}

impl WriteBuffer for i64 {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_i64(self);
        Ok(())
    }
}
