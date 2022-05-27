use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::ROOM_FURNI_CAMPAIGNS,
};

pub async fn get_room_furni_campaigns(
    _message: &mut Message,
    _session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(64);

    result.extend_from_slice(encoding::base64::encode(ROOM_FURNI_CAMPAIGNS, 2).as_slice());

    result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    result.extend_from_slice(&[1]);

    println!("get_furni_camps");
    Some(result)
}
