#![warn(rust_2018_idioms)]

#[macro_use]
extern crate diesel;

pub(crate) mod codec;
pub mod config;
pub(crate) mod database;
pub mod encoding;
pub(crate) mod game;
pub(crate) mod message;
pub(crate) mod schema;
pub(crate) mod schema_ext;
pub(crate) mod server;

use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use crate::config::Config;
use crate::game::session_manager::SessionManager;
use crate::server::Server;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::new(
        IpAddr::from_str(&config.server.ip).unwrap(),
        config.server.port,
    );

    let server = Server::new(addr, config.server.max_conns_per_ip).await;

    let pool = database::establish_connection(&config)?;

    let pool = Arc::new(pool);

    log::info!(
        "server now running on {}:{}",
        config.server.ip,
        config.server.port
    );

    let session_manager = Arc::new(SessionManager::new());

    server.run(session_manager, pool).await;

    Ok(())
}
