use crate::{Codec, Error, ErrorKind};

const LENGTH: usize = 2;

pub struct Mti<'a> {
    data: &'a [u8; LENGTH],
}

impl Mti<'_> {
    pub fn data(&self) -> &[u8; LENGTH] {
        self.data
    }
}

impl<'a> Codec<'a> for Mti<'a> {
    type Error = Error;

    fn decode(src: &mut &'a [u8]) -> core::result::Result<Self, Self::Error> {
        let (data, tail) = src.split_first_chunk::<LENGTH>().ok_or_else(|| {
            Error::new(ErrorKind::InsufficientLength {
                need: LENGTH,
                got: src.len(),
            })
        })?;
        *src = tail;

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let input = &[0x01, 0x00, 0xFF, 0xFF];
        let expected = &[0x01, 0x00];

        let mut src = &input[..];
        let mti = Mti::decode(&mut src).expect("decoding failed");
        assert_eq!(mti.data(), expected);
        assert_eq!(src, &[0xFF, 0xFF]);
    }
}
