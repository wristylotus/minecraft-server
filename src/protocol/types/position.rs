use crate::protocol::types::{ReadBuffer, WriteBuffer};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::fmt::Debug;

const I26_MASK_U64: u64 = (1u64 << 26) - 1;
const I12_MASK_U64: u64 = (1u64 << 12) - 1;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position(i32, i32, i16);

fn sign_extend(v: u64, width: u32) -> i64 {
    debug_assert!(width > 0 && width <= 63);
    let sign_bit = 1u64 << (width - 1);
    let mask = (1u64 << width) - 1;
    let v = v & mask;
    if (v & sign_bit) != 0 {
        (v | !mask) as i64
    } else {
        v as i64
    }
}

impl ReadBuffer for Position {
    fn read(buf: &mut Bytes) -> anyhow::Result<Position> {
        let word = buf.split_to(size_of::<u64>()).get_u64();

        let x = sign_extend(word & I26_MASK_U64, 26) as i32;
        let y = sign_extend((word >> 26) & I26_MASK_U64, 26) as i32;
        let z = sign_extend((word >> 52) & I12_MASK_U64, 12) as i16;

        Ok(Position(x, y, z))
    }
}

impl WriteBuffer for Position {
    fn write(self, buf: &mut BytesMut) -> anyhow::Result<()> {
        let Position(x, y, z) = self;
        let x_bits = (x as u32 as u64) & I26_MASK_U64;
        let y_bits = (y as u32 as u64) & I26_MASK_U64;
        let z_bits = (z as u16 as u64) & I12_MASK_U64;

        buf.put_u64(x_bits | (y_bits << 26) | (z_bits << 52));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_write_correctness() {
        let mut buf = BytesMut::new();
        let expected = Position(-33554432, 33554431, -2048);
        
        expected.clone().write(&mut buf).unwrap();
        let actual = Position::read(&mut buf.freeze()).unwrap();

        assert_eq!(expected, actual)
    }
}
