use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::SESSION_PARAMS,
};

pub async fn handle_init_crypto(
    _message: &Message,
    _session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    log::info!("handling init_crypto event");

    let mut result = BytesMut::with_capacity(256);

    result.extend_from_slice(encoding::base64::encode(SESSION_PARAMS, 2).as_slice());

    // Number of pairs
    result.extend_from_slice(encoding::wire::encode_i32(9).as_slice());

    // COPPA
    result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // Vouchers
    result.extend_from_slice(encoding::wire::encode_i32(1).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(true).as_slice());

    // Parent Email Required
    result.extend_from_slice(encoding::wire::encode_i32(2).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // Parent Email Required when Registering
    result.extend_from_slice(encoding::wire::encode_i32(3).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // Allow Direct Email
    result.extend_from_slice(encoding::wire::encode_i32(4).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(true).as_slice());

    // Date (requires dd-MM-YYYY format but value doesn't actually matter)
    result.extend_from_slice(encoding::wire::encode_i32(5).as_slice());
    result.extend_from_slice(b"dd-MM-yyyy\x02");

    // Allow Profile Editing
    result.extend_from_slice(encoding::wire::encode_i32(7).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // Tracking Heading
    result.extend_from_slice(encoding::wire::encode_i32(8).as_slice());
    result.extend_from_slice(b"Hebbo\x02");

    // Tutorial Enabled
    result.extend_from_slice(encoding::wire::encode_i32(9).as_slice());
    result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // Terminator
    result.extend_from_slice(&[1]);

    println!("init_crypto");
    Some(result)
}
