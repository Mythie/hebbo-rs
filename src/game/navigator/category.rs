use std::sync::Arc;

use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use tokio_diesel::*;

use crate::database::PgPool;
use crate::schema::*;

#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name = "navigator_categories"]
pub struct NavigatorCategory {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub allows_trading: bool,
}

impl NavigatorCategory {
    pub async fn fetch_all(pool: Arc<PgPool>) -> Vec<NavigatorCategory> {
        use crate::schema::navigator_categories::dsl::*;

        let result = navigator_categories.load_async(pool.as_ref()).await;

        

        match result {
            Ok(result) => result,
            Err(e) => {
                log::error!("{:?}", e);
                
                vec![]
            }
        }
    }
}
