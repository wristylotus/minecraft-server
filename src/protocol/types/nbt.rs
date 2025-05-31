use crate::protocol::types::WriteBuffer;
use bytes::{BufMut, BytesMut};
use std::borrow::Cow;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct NBTString(Cow<'static, str>);

impl NBTString {
    const ID: u8 = 8;

    pub fn new(value: Cow<'static, str>) -> Self {
        Self(value)
    }
}

impl WriteBuffer for NBTString {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        let value = cesu8::to_java_cesu8(self.0.as_ref());
        buf.put_slice(&[Self::ID]);
        u16::write(value.len() as u16, buf)?;
        buf.put_slice(&value);
        Ok(())
    }
}

impl From<&'static str> for NBTString {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for NBTString {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Into<String> for NBTString {
    fn into(self) -> String {
        self.0.into()
    }
}

impl PartialEq<&str> for NBTString {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<String> for NBTString {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl Display for NBTString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
