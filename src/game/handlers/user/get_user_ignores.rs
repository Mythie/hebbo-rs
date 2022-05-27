use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::IGNORED_USERS_LIST,
};

pub async fn get_user_ignores(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(1024);

    let user = match session.get_user().await {
        Some(user) => user,
        None => return None,
    };

    let ignored_users = user.fetch_ignores(session.get_pool()).await;

    result.extend_from_slice(encoding::base64::encode(IGNORED_USERS_LIST, 2).as_slice());

    // Ignored Count
    result.extend_from_slice(encoding::wire::encode_i32(ignored_users.len() as i32).as_slice());

    for ignored in ignored_users {
        result.extend_from_slice(ignored.get_username().as_bytes());
        result.extend_from_slice(&[2]);
    }

    // Terminator
    result.extend_from_slice(&[1]);

    println!("get_user_ignores");
    Some(result)
}
