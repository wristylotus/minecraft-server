use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct I8;

impl I8 {
    pub fn read(buf: &mut Bytes) -> anyhow::Result<i8> {
        let mut value = buf.split_to(1);
        Ok(value.get_i8())
    }
}

pub struct U8;

impl U8 {
    pub fn read(buf: &mut Bytes) -> anyhow::Result<u8> {
        let mut value = buf.split_to(1);
        Ok(value.get_u8())
    }
}

pub struct U16;

impl U16 {
    pub fn read(buf: &mut Bytes) -> anyhow::Result<u16> {
        let mut value = buf.split_to(2);
        Ok(value.get_u16())
    }

    pub fn write(value: u16, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u16(value);
        Ok(())
    }
}

pub struct I64;

impl I64 {
    pub fn read(buf: &mut Bytes) -> anyhow::Result<i64> {
        let mut value = buf.split_to(8);
        Ok(value.get_i64())
    }

    pub fn write(value: i64, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_i64(value);
        Ok(())
    }
}
