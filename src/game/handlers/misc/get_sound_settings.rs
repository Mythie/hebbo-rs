use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::SOUND_SETTINGS,
};

pub async fn get_sound_settings(
    _message: &mut Message,
    _session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    result.extend_from_slice(encoding::base64::encode(SOUND_SETTINGS, 2).as_slice());

    // System
    result.extend_from_slice(encoding::wire::encode_i32(100).as_slice());
    // Furni
    result.extend_from_slice(encoding::wire::encode_i32(100).as_slice());
    // Trax
    result.extend_from_slice(encoding::wire::encode_i32(100).as_slice());

    // Terminator
    result.extend_from_slice(&[1]);

    println!("get_sound_settings");
    Some(result)
}
