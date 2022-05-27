use bytes::BytesMut;

use crate::{encoding, message::outgoing::ROOM_JOIN_ERROR};

pub async fn make_room_join_error(error_code: i32) -> BytesMut {
    let mut bytes = BytesMut::with_capacity(64);

    bytes.extend_from_slice(encoding::base64::encode(ROOM_JOIN_ERROR, 2).as_slice());

    bytes.extend_from_slice(encoding::wire::encode_i32(error_code).as_slice());

    bytes.extend_from_slice(&[1]);

    bytes
}
