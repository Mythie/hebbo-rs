use bytes::BytesMut;

use crate::{encoding, message::outgoing::USER_KICKED_FROM_ROOM};

pub async fn make_user_kicked_from_room() -> BytesMut {
    BytesMut::from(encoding::base64::encode(USER_KICKED_FROM_ROOM, 2).as_slice())
}
