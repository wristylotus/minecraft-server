use crate::protocol::types::{ReadBuffer, VarInt, WriteBuffer};
use bytes::{Bytes, BytesMut};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MCString(Cow<'static, str>);

impl MCString {
    const MAX_LENGTH: usize = 32767;

    pub fn new(value: Cow<'static, str>) -> Self {
        Self(value)
    }
}

impl ReadBuffer for MCString {
    fn read(buf: &mut Bytes) -> anyhow::Result<MCString> {
        let length = VarInt::read(buf)?;
        let value = String::from_utf8_lossy(&buf.split_to(length.into())).into_owned();
        Ok(value.into())
    }
}

impl WriteBuffer for MCString {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        if self.0.len() > Self::MAX_LENGTH {
            return Err(anyhow::anyhow!("String too long"));
        }

        VarInt::write((self.0.len() as i32).into(), buf)?;
        buf.write_str(self.0.as_ref())?;

        Ok(())
    }
}

impl From<&'static str> for MCString {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for MCString {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Into<String> for MCString {
    fn into(self) -> String {
        self.0.into()
    }
}

impl PartialEq<&str> for MCString {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<String> for MCString {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl Display for MCString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
