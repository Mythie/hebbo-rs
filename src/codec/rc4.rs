use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct Rc4Codec;

impl Rc4Codec {
    pub fn new() -> Self {
        Self
    }
}

impl Decoder for Rc4Codec {
    type Item = BytesMut;

    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(src.clone()))
    }
}

impl Encoder<BytesMut> for Rc4Codec {
    type Error = std::io::Error;

    fn encode(&mut self, item: BytesMut, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(item);

        Ok(())
    }
}
