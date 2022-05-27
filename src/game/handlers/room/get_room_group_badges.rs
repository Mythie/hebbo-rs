use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{room::Room, session::Session, session_manager::SessionManager},
    message::outgoing::ROOM_GROUP_BADGES,
};

pub async fn get_room_group_badges(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    let room_id = session.get_state().get_current_room();

    if room_id == -1 {
        return None;
    }

    match Room::fetch_with_id(room_id, session.get_pool()).await {
        Some(_) => {}
        None => return None,
    };

    result.extend_from_slice(encoding::base64::encode(ROOM_GROUP_BADGES, 2).as_slice());

    result.extend_from_slice(
        "Ib[ZCs58116s04078s04072s52074889902cf4440630470f222ad5c6489d7".as_bytes(),
    );
    result.extend_from_slice(&[2]);

    result.extend_from_slice(&[1]);

    println!("get_Room_badges");
    Some(result)
}
