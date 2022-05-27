use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding::{self},
    game::{room::Room, session::Session, session_manager::SessionManager},
    message::outgoing::ROOM_PUBLIC_DATA,
};

pub async fn get_room_public_data(
    message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(4096);

    let (room_id, _) = encoding::wire::decode_i32(message.body.as_ref());

    println!("pubdata room_id: {}", &room_id);

    let room = match Room::fetch_with_id(room_id, session.get_pool()).await {
        Some(room) => room,
        None => return None,
    };

    // Message Header
    result.extend_from_slice(encoding::base64::encode(ROOM_PUBLIC_DATA, 2).as_slice());

    // Room ID
    result.extend_from_slice(encoding::wire::encode_i32(27).as_slice());

    // Room SWFs
    result.extend_from_slice(room.swfs.join(",").as_bytes());
    result.extend_from_slice(&[2]);

    // Room ID
    result.extend_from_slice(encoding::wire::encode_i32(room.id).as_slice());

    result.extend_from_slice(&[1]);

    println!("get_room_pub_data");
    Some(result)
}
