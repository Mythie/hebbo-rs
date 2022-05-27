use std::sync::Arc;

use chrono::{DateTime, Utc};
use diesel::{dsl::any, prelude::*};
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

use crate::database::PgPool;
use crate::game::achievement::Achievement;
use crate::schema::*;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    rank: i16,
    gender: i16,
    figure: String,
    credits: i32,
    pixels: i32,
    home_room: i32,
    sso_ticket: String,
    last_ip_addr: String,
    last_online: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserSubscription {
    pub id: i32,
    pub rank: i16,
    pub expires_at: DateTime<Utc>,
    pub acquired_on: DateTime<Utc>,
    pub user_id: i32,
}

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserFriend {
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
    pub accepted: bool,
}

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserIgnore {
    pub id: i32,
    pub user_id: i32,
    pub ignore_id: i32,
}

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct UserAchievement {
    pub id: i32,
    pub user_id: i32,
    pub achievement_id: i32,
    pub level: i32,
    pub progress: i32,
}

impl User {
    pub async fn new_from_sso<T: AsRef<str>>(ticket: T, pool: Arc<PgPool>) -> Option<Self> {
        use crate::schema::users::dsl::*;

        let result = users
            .filter(sso_ticket.eq(ticket.as_ref().to_string()))
            .first_async(pool.as_ref())
            .await;

        result.ok()
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    pub fn get_credits(&self) -> i32 {
        self.credits
    }

    pub fn get_pixels(&self) -> i32 {
        self.pixels
    }

    pub fn get_home_room(&self) -> i32 {
        self.home_room
    }

    pub fn get_gender(&self) -> i16 {
        self.gender
    }

    pub fn get_figure(&self) -> String {
        self.figure.clone()
    }

    pub async fn fetch_subscription(&self, pool: Arc<PgPool>) -> Option<UserSubscription> {
        use crate::schema::user_subscriptions::dsl::*;

        let result = user_subscriptions
            .filter(user_id.eq(self.id))
            .filter(expires_at.gt(Utc::now()))
            .load_async::<UserSubscription>(pool.as_ref())
            .await;

        match result {
            Ok(result) => result.iter().max_by(|&a, &b| a.rank.cmp(&b.rank)).cloned(),
            Err(_) => None,
        }
    }

    pub async fn fetch_friend_ids(&self, _pool: Arc<PgPool>) -> Vec<UserFriend> {
        vec![]
    }

    pub async fn fetch_ignored_ids(&self, pool: Arc<PgPool>) -> Vec<UserIgnore> {
        use crate::schema::user_ignores::dsl::*;

        let result = user_ignores
            .filter(user_id.eq(self.id))
            .load_async(pool.as_ref())
            .await;

        

        match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("{:?}", e);
                
                vec![]
            }
        }
    }

    pub async fn fetch_friends(&self, pool: Arc<PgPool>) -> Vec<User> {
        use crate::schema::user_friends::dsl::*;
        use crate::schema::users::dsl::id;
        use crate::schema::users::dsl::*;

        let friend_ids = user_friends
            .select(friend_id)
            .filter(user_id.eq(self.id))
            .filter(accepted.eq(true));

        let result = users
            .filter(id.eq(any(friend_ids)))
            .load_async(pool.as_ref())
            .await;

        

        match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("{:?}", e);
                
                vec![]
            }
        }
    }

    pub async fn fetch_ignores(&self, pool: Arc<PgPool>) -> Vec<User> {
        use crate::schema::user_ignores::dsl::*;
        use crate::schema::users::dsl::id;
        use crate::schema::users::dsl::*;

        let friend_ids = user_ignores.select(ignore_id).filter(user_id.eq(self.id));

        let result = users
            .filter(id.eq(any(friend_ids)))
            .load_async(pool.as_ref())
            .await;

        

        match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("{:?}", e);
                
                vec![]
            }
        }
    }

    pub async fn fetch_achievements(
        &self,
        pool: Arc<PgPool>,
    ) -> Vec<(UserAchievement, Achievement)> {
        use crate::schema::achievements::dsl::*;
        use crate::schema::user_achievements::dsl::*;

        let result = user_achievements
            .inner_join(achievements)
            .filter(user_id.eq(self.id))
            .load_async(pool.as_ref())
            .await;

        

        match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("{:?}", e);
                
                vec![]
            }
        }
    }
}
