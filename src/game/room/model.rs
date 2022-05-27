use std::sync::Arc;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

use crate::database::PgPool;
use crate::schema::*;
use crate::schema_ext::RoomType;

use super::{RoomModelStaticItem};

#[derive(Clone, Debug, Queryable, Insertable)]
pub struct RoomModel {
    pub id: i32,
    pub name: String,
    pub enabled: bool,
    pub type_: RoomType,
    pub heightmap: Vec<String>,
    pub door_x: i32,
    pub door_y: i32,
    pub door_z: i32,
    pub door_direction: i32,
    pub required_rank: i32,
    pub max_users: i32,
}

impl RoomModel {
    pub async fn fetch_with_id(_id: i32, pool: Arc<PgPool>) -> Option<Self> {
        use crate::schema::room_models::dsl::*;

        let result = room_models.find(id).first_async(pool.as_ref()).await;

        result.ok()
    }

    pub async fn load_static_items(&self, pool: Arc<PgPool>) -> Vec<RoomModelStaticItem> {
        RoomModelStaticItem::fetch_with_model(self, pool).await
    }
}
