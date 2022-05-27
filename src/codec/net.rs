use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::encoding;

use super::frame::{Message, MessageFrame};
use super::rc4::Rc4Codec;

#[derive(Debug)]
pub struct NetCodec {
    encrypted: bool,
    encryption_codec: Option<Rc4Codec>,
}

impl NetCodec {
    pub fn new() -> Self {
        Self {
            encrypted: false,
            encryption_codec: None,
        }
    }

    pub fn new_with_encryption() -> Self {
        Self {
            encrypted: true,
            encryption_codec: Some(Rc4Codec::new()),
        }
    }
}

impl Decoder for NetCodec {
    type Item = MessageFrame;

    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        if src.starts_with(b"<policy-file-request/>") {
            src.advance(src.chunk().len());

            return Ok(Some(MessageFrame::Policy));
        }

        let message_length = src.get(0..3).unwrap_or(&[]);

        if message_length.len() != 3 {
            return Ok(None);
        }

        let message_length = encoding::base64::decode(message_length) - 2;

        src.advance(3);

        let message_id = src.get(0..2).unwrap_or(&[]);

        if message_id.len() != 2 {
            return Ok(None);
        }

        let message_id = encoding::base64::decode(message_id);

        src.advance(2);

        let body = src.get(0..message_length as usize).unwrap_or(&[]);

        if body.len() != message_length as usize {
            return Ok(None);
        }

        let body = BytesMut::from(body);

        src.advance(message_length as usize);

        log::info!("received [{}] {:?}", message_id, &body);

        Ok(Some(MessageFrame::Packet(Message {
            id: message_id,
            raw_id: String::from_utf8_lossy(encoding::base64::encode(message_id, 2).as_ref())
                .to_string(),
            len: message_length,
            body,
        })))
    }
}

impl Encoder<BytesMut> for NetCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: BytesMut, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(item);

        Ok(())
    }
}
