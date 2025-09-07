use crate::protocol::types::{ReadBuffer, VarInt, WriteBuffer};
use bytes::{Bytes, BytesMut};

impl<T> WriteBuffer for Vec<T>
where
    T: WriteBuffer,
{
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        VarInt::new(self.len() as i32).write(buf)?;
        for item in self {
            item.write(buf)?;
        }
        Ok(())
    }
}

impl<T> ReadBuffer for Vec<T>
where
    T: ReadBuffer,
{
    fn read(buf: &mut Bytes) -> anyhow::Result<Self> {
        let size = VarInt::read(buf)?;
        let mut vector = Vec::with_capacity(size.into());
        for _ in 0..size.into() {
            let value = T::read(buf)?;
            vector.push(value);
        }

        Ok(vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::types::MCString;
    #[test]
    fn test_read_write_correctness() {
        let mut buf = BytesMut::new();
        let expected: Vec<MCString> = vec!["hello".into(), "test".into(), "world".into()];

        expected.clone().write(&mut buf).unwrap();
        let actual = Vec::<MCString>::read(&mut buf.freeze()).unwrap();

        assert_eq!(expected, actual)
    }
}
