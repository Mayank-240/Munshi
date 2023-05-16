use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub db_url: String,
    pub schema_file: String,
}

impl Config {

    pub fn from_env() -> Result<Config>{
        dotenv().ok();

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        let config = c.try_into()
            .context("loading configuration from environment");

        config
    }
}
