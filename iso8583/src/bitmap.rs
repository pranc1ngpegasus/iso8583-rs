use crate::{Codec, Error, ErrorKind};

const LENGTH: usize = 8;

pub struct Bitmap<'a> {
    first: &'a [u8; LENGTH],
    second: Option<&'a [u8; LENGTH]>,
}

impl Bitmap<'_> {
    pub fn first(&self) -> &[u8; LENGTH] {
        self.first
    }

    pub fn second(&self) -> Option<&[u8; LENGTH]> {
        self.second
    }
}

impl<'a> Codec<'a> for Bitmap<'a> {
    type Error = Error;

    fn decode(src: &mut &'a [u8]) -> core::result::Result<Self, Self::Error> {
        let (first, tail) = src.split_first_chunk::<LENGTH>().ok_or_else(|| {
            Error::new(ErrorKind::InsufficientLength {
                need: LENGTH,
                got: src.len(),
            })
        })?;
        *src = tail;

        let second = if first[0] & 0x80 != 0 {
            let (second, tail) = src.split_first_chunk::<LENGTH>().ok_or_else(|| {
                Error::new(ErrorKind::InsufficientLength {
                    need: LENGTH,
                    got: src.len(),
                })
            })?;
            *src = tail;
            Some(second)
        } else {
            None
        };

        Ok(Self { first, second })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_single_bitmap() {
        let input = &[
            0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // first
            0xFF, 0xFF, // extra data
        ];
        let expected_first = &[0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let mut src = &input[..];
        let bitmap = Bitmap::decode(&mut src).expect("decoding failed");
        assert_eq!(bitmap.first(), expected_first);
        assert!(bitmap.second().is_none());
        assert_eq!(src, &[0xFF, 0xFF]);
    }

    #[test]
    fn test_decode_double_bitmap() {
        let input = &[
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // first
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, // second
            0xFF, 0xFF, // extra data
        ];
        let expected_first = &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected_second = &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

        let mut src = &input[..];
        let bitmap = Bitmap::decode(&mut src).expect("decoding failed");
        assert_eq!(bitmap.first(), expected_first);
        assert_eq!(bitmap.second(), Some(expected_second));
        assert_eq!(src, &[0xFF, 0xFF]);
    }
}
