use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    game::{session::Session, session_manager::SessionManager},
};

pub async fn not_implemented(
    _message: &mut Message,
    _session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    None
}
