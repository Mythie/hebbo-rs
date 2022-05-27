use crate::schema::*;
use diesel::{Insertable, Queryable};

#[derive(Clone, Debug, Queryable, Insertable)]
pub struct Achievement {
    pub id: i32,
    pub name: String,
    pub type_: String,
    pub level: i32,
    pub reward_pixels: i32,
    pub reward_points: i32,
    pub progress_needed: i32,
}
