use crate::protocol::types::{ReadBuffer, WriteBuffer};
use bytes::{Buf, BufMut, Bytes, BytesMut};

macro_rules! impl_buffer {
    ($t:ty, $get:ident, $put:ident) => {
        impl ReadBuffer for $t {
            fn read(buf: &mut Bytes) -> anyhow::Result<$t> {
                let mut value = buf.split_to(size_of::<$t>());
                Ok(value.$get())
            }
        }

        impl WriteBuffer for $t {
            fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
                buf.$put(self);
                Ok(())
            }
        }
    };
}

impl_buffer!(i8, get_i8, put_i8);
impl_buffer!(u8, get_u8, put_u8);
impl_buffer!(i16, get_i16, put_i16);
impl_buffer!(u16, get_u16, put_u16);
impl_buffer!(i32, get_i32, put_i32);
impl_buffer!(u32, get_u32, put_u32);
impl_buffer!(i64, get_i64, put_i64);
impl_buffer!(u64, get_u64, put_u64);

impl ReadBuffer for bool {
    fn read(buf: &mut Bytes) -> anyhow::Result<bool> {
        Ok(u8::read(buf)? == 0x01)
    }
}

impl WriteBuffer for bool {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        buf.put_u8(self as u8);
        Ok(())
    }
}
