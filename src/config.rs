use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_ip")]
    pub ip: String,

    #[serde(default = "default_server_port")]
    pub port: u16,

    #[serde(default = "default_server_max_conns_per_ip")]
    pub max_conns_per_ip: u8,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub database: String,

    #[serde(default = "default_database_port")]
    pub port: u16,
}

fn default_server_ip() -> String {
    String::from("127.0.0.1")
}

fn default_server_port() -> u16 {
    9000
}

fn default_server_max_conns_per_ip() -> u8 {
    5
}

fn default_database_port() -> u16 {
    5432
}
