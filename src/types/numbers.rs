use bytes::Bytes;

pub struct U16;

impl U16 {
    pub fn read(buf: Bytes) -> anyhow::Result<(u16, Bytes)> {
        let data = &buf[..2];
        let buf = Bytes::copy_from_slice(&buf[2..]);

        Ok((u16::from_be_bytes(data.try_into()?), buf))
    }
}
