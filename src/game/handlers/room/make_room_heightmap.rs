use bytes::BytesMut;

use crate::{
    encoding,
    game::room::{Room, RoomModel},
    message::outgoing::ROOM_HEIGHTMAP,
};

pub async fn make_room_heightmap(_room: &Room, room_model: &RoomModel) -> BytesMut {
    let mut bytes = BytesMut::with_capacity(512);

    bytes.extend_from_slice(encoding::base64::encode(ROOM_HEIGHTMAP, 2).as_slice());

    bytes.extend_from_slice(room_model.heightmap.join("\r").as_bytes());

    bytes.extend_from_slice(&[13, 2, 1]);

    bytes
}
