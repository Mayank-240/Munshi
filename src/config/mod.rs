// use color_eyre::Result;
use dotenv::dotenv;
// use eyre::WrapErr;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub db_url: String,
    pub schema_file: String,
}

impl AppConfig {

    pub fn from_env() -> AppConfig{
        dotenv().ok();

        let c = config::Config::builder()
                    .add_source(config::Environment::default())
                    .build()
                    .expect("Environment variables error");

        // c.merge(config::Environment::default())?;

        // let config = c;

        // let config = c.try_into()
        //     .context("loading configuration from environment");
        let config: AppConfig = c.try_deserialize().expect("error creating Config");

        config
    }
}
