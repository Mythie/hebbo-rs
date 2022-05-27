use bytes::BytesMut;

use crate::{encoding, message::outgoing::ROOM_URL};

pub async fn make_room_url<T: AsRef<str>>(url: T) -> BytesMut {
    let mut bytes = BytesMut::with_capacity(512);

    bytes.extend_from_slice(encoding::base64::encode(ROOM_URL, 2).as_slice());

    bytes.extend_from_slice(url.as_ref().as_bytes());
    bytes.extend_from_slice(&[2]);

    bytes.extend_from_slice(&[1]);

    bytes
}
