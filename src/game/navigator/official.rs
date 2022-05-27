use std::sync::Arc;

use bytes::BytesMut;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

use crate::schema::*;
use crate::{database::PgPool, encoding, game::room::Room};

use crate::schema_ext::{NavigatorOfficialBannerType, NavigatorOfficialDisplayType, RoomState};

#[derive(Debug, Queryable, Insertable)]
#[table_name = "navigator_official"]
pub struct NavigatorOfficial {
    pub id: i32,
    pub enabled: bool,
    pub name: String,
    pub description: String,
    pub order: i32,
    pub display: NavigatorOfficialDisplayType,
    pub banner_type: NavigatorOfficialBannerType,
    pub banner_source: String,
    pub banner_label: String,
    pub is_expanded: bool,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub is_category: bool,
    pub allows_trading: bool,
    pub room_id: Option<i32>,
    pub parent_id: Option<i32>,
}

impl NavigatorOfficial {
    pub async fn fetch_all(pool: Arc<PgPool>) -> Vec<NavigatorOfficial> {
        use crate::schema::navigator_official::dsl::*;

        let result = navigator_official.load_async(pool.as_ref()).await;

        match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("{:?}", e);

                vec![]
            }
        }
    }

    pub async fn fetch_room(&self, pool: Arc<PgPool>) -> Option<Room> {
        match self.room_id.as_ref() {
            Some(room_id) => {
                println!("Fetching room with id {} for {}", room_id, &self.name);
                Room::fetch_with_id(*room_id, pool).await
            }
            None => None,
        }
    }

    pub fn serialize_private_room(room: &Room, is_event: bool) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(4096);

        let room_state = match room.state {
            RoomState::Open => 1,
            RoomState::Doorbell => 2,
            RoomState::Password => 3,
        };

        // Room ID
        bytes.extend_from_slice(encoding::wire::encode_i32(room.id).as_slice());

        // Is Event?
        bytes.extend_from_slice(encoding::wire::encode_bool(is_event).as_slice());

        // Room Name
        bytes.extend_from_slice(room.name.as_bytes());
        bytes.extend_from_slice(&[2]);

        // Description?
        bytes.extend_from_slice(room.description.as_bytes());
        bytes.extend_from_slice(&[2]);

        // Room State
        bytes.extend_from_slice(encoding::wire::encode_i32(room_state).as_slice());

        // Current Users
        bytes.extend_from_slice(encoding::wire::encode_i32(room.current_users).as_slice());

        // Max Users
        bytes.extend_from_slice(encoding::wire::encode_i32(room.max_users).as_slice());

        // Description?
        bytes.extend_from_slice(room.description.as_bytes());
        bytes.extend_from_slice(&[2]);

        // ???
        bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

        // !: Can Trade
        bytes.extend_from_slice(encoding::wire::encode_bool(true).as_slice());

        // Score
        bytes.extend_from_slice(encoding::wire::encode_i32(room.score).as_slice());

        // Category ID
        bytes.extend_from_slice(
            room.navigator_category_id
                .map_or("".to_string(), |f| f.to_string())
                .as_bytes(),
        );
        bytes.extend_from_slice(&[2]);

        // Event Time Started
        bytes.extend_from_slice("".as_bytes());
        bytes.extend_from_slice(&[2]);

        // Tag Count
        bytes.extend_from_slice(encoding::wire::encode_i32(room.tags.len() as i32).as_slice());

        for tag in &room.tags {
            // Tag
            bytes.extend_from_slice(tag.as_bytes());
            bytes.extend_from_slice(&[2]);
        }

        // Icon Background
        bytes.extend_from_slice(encoding::wire::encode_i32(room.icon_background).as_slice());

        // Icon Foreground
        bytes.extend_from_slice(encoding::wire::encode_i32(room.icon_foreground).as_slice());

        // !: Icon Elements
        bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

        // for element in room.icon_elements {
        //
        // }

        // ???
        bytes.extend_from_slice(encoding::wire::encode_i32(0).as_slice());

        // ???
        bytes.extend_from_slice(encoding::wire::encode_i32(1).as_slice());

        bytes
    }
}
