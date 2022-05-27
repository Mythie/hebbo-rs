use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{navigator::NavigatorCategory, session::Session, session_manager::SessionManager},
    message::outgoing::NAVIGATOR_CATEGORIES,
};

pub async fn get_navigator_categories(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(4096);

    result.extend_from_slice(encoding::base64::encode(NAVIGATOR_CATEGORIES, 2).as_slice());

    let categories = NavigatorCategory::fetch_all(session.get_pool()).await;

    result.extend_from_slice(encoding::wire::encode_i32(categories.len() as i32).as_slice());

    for category in categories {
        result.extend_from_slice(encoding::wire::encode_i32(category.id).as_slice());

        result.extend_from_slice(category.title.as_bytes());
        result.extend_from_slice(&[2]);

        result.extend_from_slice(encoding::wire::encode_bool(category.is_enabled).as_slice());
    }

    result.extend_from_slice(&[1]);

    println!("get_nav_cats");
    Some(result)
}
