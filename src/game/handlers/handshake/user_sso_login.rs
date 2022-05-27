use bytes::{Buf, BytesMut};

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager, user::User},
    message::outgoing::AUTHENTICATION_OK,
};

pub async fn handle_user_sso_login(
    message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    let sso_ticket = match get_sso_ticket(&mut message.body) {
        Some(ticket) => ticket,
        None => return None,
    };

    let user = match User::new_from_sso(sso_ticket, session.get_pool()).await {
        Some(user) => user,
        None => return None,
    };

    // Send authentication okay and a terminator
    result.extend_from_slice(encoding::base64::encode(AUTHENTICATION_OK, 2).as_slice());

    result.extend_from_slice(&[1]);

    // let subscription_level = user.fetch_subscription_level(session.get_pool()).await;
    // let subscription_level_with_max = subscription_level.max(2);

    // Send user rights and a terminator
    // result.extend_from_slice(encoding::base64::encode(USER_RIGHTS, 2).as_slice());
    // result.extend_from_slice(
    //     encoding::wire::encode_i32(subscription_level_with_max as i32).as_slice(),
    // );

    // !: FIX
    // if false == true {
    //     result.extend_from_slice(encoding::wire::encode_i32(1000).as_slice());
    // } else {
    //     result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());
    // }

    // result.extend_from_slice(&[1]);

    session.set_user(user).await;

    println!("user_sso_login");
    Some(result)
}

fn get_sso_ticket(bytes: &mut BytesMut) -> Option<String> {
    let message_len = bytes.get(0..2).unwrap_or(&[]);

    if message_len.is_empty() {
        return None;
    }

    let message_len = encoding::base64::decode(message_len);

    bytes.advance(2);

    let sso_ticket = bytes.get(0..message_len as usize).unwrap_or(&[]).to_vec();

    if sso_ticket.is_empty() {
        return None;
    }

    bytes.advance(message_len as usize);

    Some(String::from_utf8_lossy(sso_ticket.as_slice()).to_string())
}
