mod nbt;
mod primitives;
mod string;
mod uuid;
mod varint;

pub trait ReadBuffer {
    fn read(buf: &mut Bytes) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait WriteBuffer {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()>;
}

use bytes::{Bytes, BytesMut};
pub use nbt::NBTString;
pub use string::MCString;
pub use varint::VarInt;
