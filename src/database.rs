use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::config::Config;
use std::{env, error::Error};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(config: &Config) -> Result<PgPool, Box<dyn Error>> {
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.hostname,
        env::var("DATABASE_PORT").map_or(5432, |port| port.parse::<i32>().unwrap_or(5432)),
        config.database.database
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}
