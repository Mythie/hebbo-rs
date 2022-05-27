use std::sync::Arc;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

use super::Room;
use crate::database::PgPool;
use crate::schema::*;

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, Associations)]
#[belongs_to(Room)]
pub struct RoomItem {
    pub position_x: i32,
    pub position_y: i32,
    pub extra_data: Option<serde_json::Value>,
    pub room_id: i32,
    pub item_id: i32,
    pub id: i32,
    pub position_wall: f64,
    pub position_z: i32,
}

impl RoomItem {
    pub async fn fetch_with_room(room: &Room, pool: Arc<PgPool>) -> Vec<Self> {
        use crate::schema::room_items::dsl::*;

        let result = room_items
            .filter(room_id.eq(room.id))
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
