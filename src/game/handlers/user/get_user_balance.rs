use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::{USER_CREDITS_BALANCE, USER_PIXELS_BALANCE},
};

/// Get the users balance, formatting credits and pixels to 1 decimal place as required by
/// the client.
pub async fn get_user_balance(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    let user = match session.get_user().await {
        Some(user) => user,
        None => return None,
    };

    let credits = user.get_credits() as f32;
    let pixels = user.get_pixels();

    result.extend_from_slice(encoding::base64::encode(USER_CREDITS_BALANCE, 2).as_slice());
    result.extend_from_slice(format!("{:.1}", credits).as_bytes());
    result.extend_from_slice(&[2, 1]);

    result.extend_from_slice(encoding::base64::encode(USER_PIXELS_BALANCE, 2).as_slice());
    result.extend_from_slice(encoding::wire::encode_i32(pixels).as_slice());
    result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    result.extend_from_slice(&[1]);

    println!("get_user_balance");
    Some(result)
}
