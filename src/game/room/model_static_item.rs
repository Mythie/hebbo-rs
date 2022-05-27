use std::sync::Arc;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

use super::RoomModel;
use crate::database::PgPool;
use crate::schema::*;

#[derive(Clone, Debug, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(RoomModel, foreign_key = "model_id")]
pub struct RoomModelStaticItem {
    pub id: i32,
    pub name: String,
    pub position_x: i32,
    pub position_y: i32,
    pub position_z: i32,
    pub rotation: i32,
    pub is_seat: bool,
    pub model_id: i32,
}

impl RoomModelStaticItem {
    pub async fn fetch_with_id(_id: i32, pool: Arc<PgPool>) -> Vec<Self> {
        use crate::schema::room_model_static_items::dsl::*;

        let result = room_model_static_items
            .find(id)
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

    pub async fn fetch_with_model(model: &RoomModel, pool: Arc<PgPool>) -> Vec<Self> {
        use crate::schema::room_model_static_items::dsl::*;

        let result = room_model_static_items
            .filter(model_id.eq(model.id))
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
