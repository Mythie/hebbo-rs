use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::MESSAGE_OF_THE_DAY,
};

pub async fn get_message_of_the_day(
    _message: &mut Message,
    _session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(4096);

    result.extend_from_slice(encoding::base64::encode(MESSAGE_OF_THE_DAY, 2).as_slice());

    // Message Enabled
    result.extend_from_slice(encoding::wire::encode_bool(true).as_slice());

    // Message Content
    result.extend_from_slice("Welcome to Hebbo!".as_bytes());
    result.extend_from_slice(&[2]);

    // Terminator
    result.extend_from_slice(&[1]);

    println!("get_motd");
    Some(result)
}
