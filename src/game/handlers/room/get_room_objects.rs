use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    game::{
        room::{Room, RoomItem},
        session::Session,
        session_manager::SessionManager,
    },
};

use super::make_room_model_static_items;

pub async fn get_room_objects(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let _result = BytesMut::with_capacity(8192);

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

    let room_model_static_items = room_model.load_static_items(session.get_pool()).await;

    let room_items = room.load_items(session.get_pool()).await;

    let (_floor_items, _wall_items): (Vec<&RoomItem>, Vec<&RoomItem>) = room_items
        .iter()
        .partition(|&item| item.position_wall == 0.0);

    let room_model_static_items = make_room_model_static_items(room_model_static_items).await;
    session.send_message(room_model_static_items).await;

    // if room.type_ == RoomType::Private {
    //     let room_floor_items = make_room_floor_items(floor_items).await;
    //     session.send_message(room_floor_items).await;

    //     let room_wall_items = make_room_wall_items(wall_items).await;
    //     session.send_message(room_wall_items).await;
    // }

    None
}
