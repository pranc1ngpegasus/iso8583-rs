pub trait Codec<'a>: Sized {
    type Error: core::error::Error;

    fn decode(src: &mut &'a [u8]) -> core::result::Result<Self, Self::Error>;
}
