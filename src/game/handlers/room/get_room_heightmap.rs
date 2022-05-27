use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    game::{room::Room, session::Session, session_manager::SessionManager},
};

use super::{make_room_heightmap, make_room_relative_heightmap};

pub async fn get_room_heightmap(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let _result = BytesMut::with_capacity(2048);

    let room_id = session.get_state().get_current_room();

    if room_id == -1 {
        return None;
    }

    let room = match Room::fetch_with_id(room_id, session.get_pool()).await {
        Some(room) => room,
        None => return None,
    };

    let room_model = match room.load_model(session.get_pool()).await {
        Some(room_model) => room_model,
        None => return None,
    };

    let heightmap = make_room_heightmap(&room, &room_model).await;
    session.send_message(heightmap).await;

    let relative_heightmap = make_room_relative_heightmap(&room, &room_model).await;
    session.send_message(relative_heightmap).await;

    None
}
