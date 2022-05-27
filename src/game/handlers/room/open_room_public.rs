use bytes::{Buf, BytesMut};

use crate::{
    codec::frame::Message,
    encoding::{self},
    game::{
        room::{Room},
        session::Session,
        session_manager::SessionManager,
    },
    schema_ext::{RoomType}
};

pub async fn open_room_public(
    message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let (unknown, bytes_read) = encoding::wire::decode_i32(message.body.as_ref());
    message.body.advance(bytes_read);
    println!("unknown: {}", &unknown);

    let (room_id, bytes_read) = encoding::wire::decode_i32(message.body.as_ref());
    message.body.advance(bytes_read);

    println!("room_id: {}", &room_id);

    let room = match Room::fetch_with_id(room_id, session.get_pool()).await {
        Some(room) => room,
        None => return None,
    };

    if room.type_ != RoomType::Public {
        return None;
    }

    Room::open_room(&room, session, None).await;

    None
}
