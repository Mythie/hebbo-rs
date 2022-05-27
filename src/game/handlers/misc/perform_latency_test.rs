use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::LATENCY_TEST,
};

pub async fn perform_latency_test(
    message: &mut Message,
    _session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(128);

    let (magic_number, _) = encoding::wire::decode_i32(&message.body[..]);

    result.extend_from_slice(encoding::base64::encode(LATENCY_TEST, 2).as_slice());

    // Magic Number
    result.extend_from_slice(encoding::wire::encode_i32(magic_number).as_slice());

    // Terminator
    result.extend_from_slice(&[1]);

    println!("perform_latency_test");
    Some(result)
}
