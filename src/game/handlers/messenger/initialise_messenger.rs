use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{
        session::Session,
        session_manager::SessionManager,
        user::{User, UserFriend},
    },
    message::outgoing::{MESSENGER_FRIENDS_LIST, MESSENGER_REQUESTS_LIST},
};

pub async fn initialise_messenger(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(4096);

    let user = match session.get_user().await {
        Some(user) => user,
        None => return None,
    };

    let messenger_friends = user.fetch_friend_ids(session.get_pool()).await;
    let friends = user.fetch_friends(session.get_pool()).await;

    let pending_requests = messenger_friends
        .iter()
        .filter(|friend| !friend.accepted)
        .collect::<Vec<&UserFriend>>();

    write_friends_list(&mut result, friends);

    if !pending_requests.is_empty() {
        write_pending_requests(&mut result, pending_requests)
    }

    println!("init_messenger");
    Some(result)
}

fn write_friends_list(bytes: &mut BytesMut, friends: Vec<User>) {
    bytes.extend_from_slice(encoding::base64::encode(MESSENGER_FRIENDS_LIST, 2).as_slice());

    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(300).as_slice());
    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(300).as_slice());
    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(800).as_slice());
    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(1100).as_slice());

    // ???
    bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

    // Friends Count
    bytes.extend_from_slice(encoding::wire::encode_i32(friends.len() as i32).as_slice());

    for friend in friends {
        // Id
        bytes.extend_from_slice(encoding::wire::encode_i32(friend.get_id()).as_slice());

        // Username
        bytes.extend_from_slice(friend.get_username().as_bytes());
        bytes.extend_from_slice(&[2]);

        // ???
        bytes.extend_from_slice(encoding::wire::encode_bool(true).as_slice());

        // Online
        bytes.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

        // Currently in Room
        bytes.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

        // Figure
        bytes.extend_from_slice(friend.get_figure().as_bytes());
        bytes.extend_from_slice(&[2]);

        // ???
        bytes.extend_from_slice(encoding::wire::encode_bool(false).as_slice());

        // Motto
        bytes.extend_from_slice("motto".as_bytes());
        bytes.extend_from_slice(&[2]);

        // Last Online
        bytes.extend_from_slice("".as_bytes());
        bytes.extend_from_slice(&[2]);

        // Real Name
        bytes.extend_from_slice(friend.get_username().as_bytes());
        bytes.extend_from_slice(&[2]);

        // Real Name
        bytes.extend_from_slice("".as_bytes());
        bytes.extend_from_slice(&[2]);
    }

    // Terminator
    bytes.extend_from_slice(&[1]);
}

fn write_pending_requests(bytes: &mut BytesMut, requests: Vec<&UserFriend>) {
    bytes.extend_from_slice(encoding::base64::encode(MESSENGER_REQUESTS_LIST, 2).as_slice());

    // Requests Count
    bytes.extend_from_slice(encoding::wire::encode_i32(requests.len() as i32).as_slice());

    // Requests Count (again?)
    bytes.extend_from_slice(encoding::wire::encode_i32(requests.len() as i32).as_slice());

    // !: Evaluate if i32 request id's are required
    for (_index, request) in requests.into_iter().enumerate() {
        // Request ID
        bytes.extend_from_slice(encoding::wire::encode_i32(request.id).as_slice());

        // Last Online
        bytes.extend_from_slice("".as_bytes());

        // Request ID
        bytes.extend_from_slice(encoding::wire::encode_i32(request.id).as_slice());
    }

    // Terminator
    bytes.extend_from_slice(&[1]);
}
