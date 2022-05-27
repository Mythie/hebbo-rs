use bytes::BytesMut;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::ACHIEVEMENT_SCORE_VALUE,
};

pub async fn get_achievement_score(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    let user = match session.get_user().await {
        Some(user) => user,
        None => return None,
    };

    let achievements = user.fetch_achievements(session.get_pool()).await;

    let achievement_score: i32 = achievements
        .iter()
        .map(|(user_achievement, achievement)| user_achievement.level * achievement.reward_points)
        .sum();

    result.extend_from_slice(encoding::base64::encode(ACHIEVEMENT_SCORE_VALUE, 2).as_slice());

    result.extend_from_slice(encoding::wire::encode_i32(achievement_score).as_slice());

    result.extend_from_slice(&[1]);

    println!("get_user_ach_score");
    Some(result)
}
