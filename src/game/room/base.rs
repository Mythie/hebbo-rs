use std::sync::Arc;

use crate::{
    database::PgPool,
    game::{
        handlers::{
            room::{make_room_join_error, make_room_model_info, make_room_url},
            user::make_user_kicked_from_room,
        },
        session::Session,
    },
};

use super::RoomItem;
use super::RoomModel;
use crate::game::user::User;

use crate::schema::*;
use crate::schema_ext::{RoomState, RoomType};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

#[derive(Debug, Queryable, Identifiable, Insertable, Associations)]
#[belongs_to(User, foreign_key = "owner_id")]
#[belongs_to(RoomModel, foreign_key = "model_id")]
pub struct Room {
    pub id: i32,
    pub state: RoomState,
    pub type_: RoomType,
    pub name: String,
    pub description: String,
    pub password: String,
    pub score: i32,
    pub current_users: i32,
    pub max_users: i32,
    pub tags: Vec<String>,
    pub swfs: Vec<String>,
    pub icon_foreground: i32,
    pub icon_background: i32,
    pub wallpaper: String,
    pub landscape: String,
    pub floor: String,
    pub wall_thickness: i32,
    pub floor_thickness: i32,
    pub allow_pets: bool,
    pub allow_pet_feeding: bool,
    pub allow_blocking: bool,
    pub hidden_walls: bool,
    pub navigator_category_id: Option<i32>,
    pub owner_id: Option<i32>,
    pub model_id: i32,
}

impl Room {
    pub async fn fetch_with_id(_id: i32, pool: Arc<PgPool>) -> Option<Self> {
        use crate::schema::rooms::dsl::*;

        let result = rooms.find(id).first_async(pool.as_ref()).await;

        result.ok()
    }

    pub async fn load_model(&self, pool: Arc<PgPool>) -> Option<RoomModel> {
        RoomModel::fetch_with_id(self.model_id, pool).await
    }

    pub async fn load_items(&self, pool: Arc<PgPool>) -> Vec<RoomItem> {
        RoomItem::fetch_with_room(self, pool).await
    }

    pub async fn open_room(room: &Self, session: &Session, password: Option<&str>) -> bool {
        let session_state = session.get_state();

        let room_kicked_packet = make_user_kicked_from_room().await;

        let _user = match session.get_user().await {
            Some(user) => user,
            None => return false,
        };

        let model = match room.load_model(session.get_pool()).await {
            Some(model) => model,
            None => {
                session.send_message(room_kicked_packet).await;

                return false;
            }
        };

        if 0 >= 1 {
            let room_join_error_packet = make_room_join_error(1).await;

            session.send_message(room_join_error_packet).await;
            session.send_message(room_kicked_packet).await;

            return false;
        }

        // TODO: Add room_ban checks
        // if room.is_user_banned(user.get_id()).await {
        //
        // }

        if session_state.get_current_room() != room.id {
            session_state.set_current_room(room.id);
        }

        if room.type_ == RoomType::Private {
            // TODO: Send private room message?
        }

        if !session_state.get_is_room_authenticated() {
            match room.state {
                RoomState::Password => {
                    if password.unwrap_or("") != room.password {
                        return false;
                    }
                }
                RoomState::Doorbell => {
                    // TODO: Implement
                    return false;
                }
                _ => {}
            }
        }

        session_state.set_is_room_authenticated(true);

        Self::enter_room(room, &model, session).await
    }

    pub async fn enter_room(room: &Self, model: &RoomModel, session: &Session) -> bool {
        let session_state = session.get_state();

        if !session_state.get_is_room_authenticated() {
            return false;
        }

        let room_url = make_room_url(format!("/rooms/{}", room.id)).await;
        session.send_message(room_url).await;

        let room_model_info = make_room_model_info(room, model).await;
        session.send_message(room_model_info).await;

        if room.type_ == RoomType::Public {
            // TODO: Decorations, Ratings, Events
        }

        true
    }
}
