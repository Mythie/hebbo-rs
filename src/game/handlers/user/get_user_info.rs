use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager, user::User},
    message::outgoing::USER_CHARACTER_INFO,
};

pub async fn get_user_info(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    let user = match session.get_user().await {
        Some(user) => user,
        None => return None,
    };

    write_user_info(&mut result, &user);
    // write_achievement_score(&mut result);

    result.extend_from_slice(&[1]);

    println!("get_user_info");
    Some(result)
}

fn write_user_info(bytes: &mut BytesMut, user: &User) {
    let gender = if user.get_gender() == 0 { "M" } else { "F" };

    bytes.extend_from_slice(encoding::base64::encode(USER_CHARACTER_INFO, 2).as_slice());

    // ID
    bytes.extend_from_slice(user.get_id().to_string().as_bytes());
    bytes.extend_from_slice(&[2]);

    // Username
    bytes.extend_from_slice(user.get_username().as_bytes());
    bytes.extend_from_slice(&[2]);

    // Figure
    bytes.extend_from_slice(user.get_figure().as_bytes());
    bytes.extend_from_slice(&[2]);

    // Gender (M|F)
    bytes.extend_from_slice(gender.as_bytes());
    bytes.extend_from_slice(&[2]);

    // Motto
    bytes.extend_from_slice("Motto".as_bytes());
    bytes.extend_from_slice(&[2]);

    // Realname
    bytes.extend_from_slice(user.get_username().as_bytes());
    bytes.extend_from_slice(&[2]);

    // Friends Stream Active
    bytes.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // Last Online
    bytes.extend_from_slice("01-01-1970 00:00:00".as_bytes());
    bytes.extend_from_slice(&[2]);

    // Name Changes Allowed
    bytes.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    // Respect Points
    bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    // Respect Credits (Human)
    bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    // Respect Credits (Pet)
    bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(24708).as_slice());
}
