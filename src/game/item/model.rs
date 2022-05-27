use crate::schema_ext::{ItemBehaviour, ItemStackingBehaviour, ItemType, ItemWalkable};
use crate::schema::*;
use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Identifiable, Queryable, Insertable)]
pub struct Item {
    pub id: i32,
    pub sprite_id: String,
    pub name: String,
    pub type_: ItemType,
    pub behaviour: ItemBehaviour,
    pub behaviour_states: i32,
    pub stacking_behaviour: ItemStackingBehaviour,
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
    pub allow_recycling: bool,
    pub allow_trading: bool,
    pub allow_selling: bool,
    pub allow_gifting: bool,
    pub allow_inventory_stacking: bool,
    pub walkable: ItemWalkable,
    pub room_limit: i32,
}
