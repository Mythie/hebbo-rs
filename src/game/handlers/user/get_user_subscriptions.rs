use bytes::BytesMut;
use chrono::Utc;

use crate::{
    codec::frame::Message,
    encoding,
    game::{session::Session, session_manager::SessionManager},
    message::outgoing::USER_SUBSCRIPTIONS_STATUS,
};

pub async fn get_user_subscriptions(
    _message: &mut Message,
    session: &Session,
    _session_manager: &SessionManager,
) -> Option<BytesMut> {
    let mut result = BytesMut::with_capacity(256);

    let user = match session.get_user().await {
        Some(user) => user,
        None => return None,
    };

    let subscription = user.fetch_subscription(session.get_pool()).await;

    result.extend_from_slice(encoding::base64::encode(USER_SUBSCRIPTIONS_STATUS, 2).as_slice());

    result.extend_from_slice("habbo_club".as_bytes());
    result.extend_from_slice(&[2]);

    match subscription {
        Some(subscription) => {
            let subscription_remaining = subscription.expires_at - Utc::now();
            let subscription_elapsed = Utc::now() - subscription.acquired_on;

            let mut days_remaining = subscription_remaining.num_days();
            let months_remaining = days_remaining / 31;
            days_remaining -= months_remaining * 31;

            let days_elapsed = subscription_elapsed.num_days();

            let past_hc_days = if subscription.rank == 1 {
                days_elapsed as i32
            } else {
                0
            };
            let past_vip_days = if subscription.rank == 2 {
                days_elapsed as i32
            } else {
                0
            };

            // HC Days Remaining (Current Month)
            result.extend_from_slice(encoding::wire::encode_i32(days_remaining as i32).as_slice());
            // Is Active
            result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());
            // HC Months Remaining
            result
                .extend_from_slice(encoding::wire::encode_i32(months_remaining as i32).as_slice());
            // Bought from catalog (Always true)
            result.extend_from_slice(encoding::wire::encode_bool(true).as_slice());
            // ???
            result.extend_from_slice(encoding::wire::encode_i32(1).as_slice());
            // Is VIP
            result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());
            // Past HC Days
            result.extend_from_slice(encoding::wire::encode_i32(past_hc_days).as_slice());
            // Past VIP Days
            result.extend_from_slice(encoding::wire::encode_i32(past_vip_days).as_slice());
            // Enable Discount Message
            result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());
            // Regular Price Discount Message
            result.extend_from_slice(encoding::wire::encode_i32(30).as_slice());
            // Special Price Discount Message
            result.extend_from_slice(encoding::wire::encode_i32(25).as_slice());
        }
        None => {
            // HC Days Remaining (Current Month)
            result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());
            // Is Active
            result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());
            // HC Months Remaining
            result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());
            // Bought from catalog (Always true)
            result.extend_from_slice(encoding::wire::encode_bool(true).as_slice());
            // ???
            result.extend_from_slice(encoding::wire::encode_i32(1).as_slice());
            // Is VIP
            result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());
            // Past HC Days
            result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());
            // Past VIP Days
            result.extend_from_slice(encoding::wire::encode_i32(0).as_slice());
            // Enable Discount Message
            result.extend_from_slice(encoding::wire::encode_bool(false).as_slice());
            // Regular Price Discount Message
            result.extend_from_slice(encoding::wire::encode_i32(30).as_slice());
            // Special Price Discount Message
            result.extend_from_slice(encoding::wire::encode_i32(25).as_slice());
        }
    }

    result.extend_from_slice(&[1]);

    println!("get_user_subs");
    Some(result)
}
