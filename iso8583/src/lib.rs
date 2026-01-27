#![no_std]

extern crate alloc;

mod bitmap;
mod codec;
mod error;
mod mti;

use crate::bitmap::Bitmap;
pub use crate::codec::Codec;
pub use crate::error::{Error, ErrorKind};
use crate::mti::Mti;

pub struct ISO8583<'a> {
    pub mti: Mti<'a>,
    pub bitmap: Bitmap<'a>,
}

impl<'a> Codec<'a> for ISO8583<'a> {
    type Error = Error;

    fn decode(src: &mut &'a [u8]) -> core::result::Result<Self, Self::Error> {
        let mti = Mti::decode(src)?;
        let bitmap = Bitmap::decode(src)?;

        Ok(Self { mti, bitmap })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let input = &[
            0x01, 0x00, // MTI
            0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Bitmap first (no secondary)
            0xFF, 0xFF, // extra data
        ];
        let expected_mti = &[0x01, 0x00];
        let expected_bitmap_first = &[0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let mut src = &input[..];
        let iso8583 = ISO8583::decode(&mut src).expect("decoding failed");
        assert_eq!(iso8583.mti.data(), expected_mti);
        assert_eq!(iso8583.bitmap.first(), expected_bitmap_first);
        assert!(iso8583.bitmap.second().is_none());
        assert_eq!(src, &[0xFF, 0xFF]);
    }

    #[test]
    fn test_decode_with_secondary_bitmap() {
        let input = &[
            0x01, 0x00, // MTI
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Bitmap first (has secondary)
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, // Bitmap second
            0xFF, 0xFF, // extra data
        ];
        let expected_mti = &[0x01, 0x00];
        let expected_bitmap_first = &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected_bitmap_second = &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

        let mut src = &input[..];
        let iso8583 = ISO8583::decode(&mut src).expect("decoding failed");
        assert_eq!(iso8583.mti.data(), expected_mti);
        assert_eq!(iso8583.bitmap.first(), expected_bitmap_first);
        assert_eq!(iso8583.bitmap.second(), Some(expected_bitmap_second));
        assert_eq!(src, &[0xFF, 0xFF]);
    }
}
