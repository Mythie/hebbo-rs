use bytes::BytesMut;

use super::incoming::*;

use crate::codec::frame::Message;
use crate::game::handlers::handshake::*;
use crate::game::handlers::messenger::*;
use crate::game::handlers::misc::*;
use crate::game::handlers::navigator::*;
use crate::game::handlers::room::*;
use crate::game::handlers::user::*;
use crate::game::session::Session;
use crate::game::session_manager::SessionManager;

pub async fn handle_message(
    message: &mut Message,
    session: &Session,
    session_manager: &SessionManager,
) -> Option<BytesMut> {
    match message.id {
        INIT_CRYPTO => handle_init_crypto(message, session, session_manager).await,
        USER_SSO_LOGIN => handle_user_sso_login(message, session, session_manager).await,
        GET_USER_BALANCE => get_user_balance(message, session, session_manager).await,
        GET_USER_INFO => get_user_info(message, session, session_manager).await,
        GET_USER_IGNORES => get_user_ignores(message, session, session_manager).await,
        GET_USER_SUBSCRIPTIONS => get_user_subscriptions(message, session, session_manager).await,
        INITIALISE_MESSENGER => initialise_messenger(message, session, session_manager).await,
        GET_MESSAGE_OF_THE_DAY => get_message_of_the_day(message, session, session_manager).await,
        PERFORM_LATENCY_TEST => perform_latency_test(message, session, session_manager).await,
        GET_NAVIGATOR_CATEGORIES => {
            get_navigator_categories(message, session, session_manager).await
        }
        GET_NAVIGATOR_OFFICIAL_ROOMS => {
            get_navigator_official_rooms(message, session, session_manager).await
        }
        GET_ROOM_GROUP_BADGES => get_room_group_badges(message, session, session_manager).await,
        GET_ROOM_FURNI_CAMPAIGNS => {
            get_room_furni_campaigns(message, session, session_manager).await
        }
        GET_ROOM_OBJECTS => get_room_objects(message, session, session_manager).await,
        GET_ROOM_HEIGHTMAP => get_room_heightmap(message, session, session_manager).await,
        GET_ACHIVEMENT_SCORE => get_achievement_score(message, session, session_manager).await,
        GET_ROOM_PUBLIC_DATA => get_room_public_data(message, session, session_manager).await,
        ROOM_OPEN_PUBLIC => open_room_public(message, session, session_manager).await,
        GET_SOUND_SETTINGS => get_sound_settings(message, session, session_manager).await,
        LATENCY_TEST_RESULT => not_implemented(message, session, session_manager).await,
        GET_USER_NOTIFICATIONS => not_implemented(message, session, session_manager).await,
        NAVIGATOR_ENTER_INQUIRED_ROOM => not_implemented(message, session, session_manager).await,
        SESSION_DEBUG => not_implemented(message, session, session_manager).await,
        USER_AGENT_DEBUG => not_implemented(message, session, session_manager).await,

        _ => {
            log::error!("No handler for {:?}", message);

            None
        }
    }
}
