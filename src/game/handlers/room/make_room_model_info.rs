use bytes::BytesMut;

use crate::{
    encoding,
    game::room::{Room, RoomModel},
    message::outgoing::ROOM_MODEL_INFO,
};

pub async fn make_room_model_info(room: &Room, model: &RoomModel) -> BytesMut {
    let mut bytes = BytesMut::with_capacity(256);

    bytes.extend_from_slice(encoding::base64::encode(ROOM_MODEL_INFO, 2).as_slice());

    bytes.extend_from_slice(model.name.as_bytes());
    bytes.extend_from_slice(&[2]);

    bytes.extend_from_slice(encoding::wire::encode_i32(room.id).as_slice());

    bytes.extend_from_slice(&[1]);

    bytes
}
