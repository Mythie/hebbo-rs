use env_logger::Env;
use hebbo::config::Config;
use hebbo::run;
use log::Level;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::default().filter_or("HEBBO_LOG", Level::Debug.as_str());

    env_logger::init_from_env(env);

    let config: Config = toml::from_str(&std::fs::read_to_string("hebbo.toml").unwrap())?;

    run(config).await
}
