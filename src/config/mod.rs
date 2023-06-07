use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub db_url: String,
    pub schema_file: String,
}

impl AppConfig {

    pub fn from_env() -> Result<AppConfig, config::ConfigError>{
        dotenv().ok();

        let c = config::Config::builder()
                    .add_source(config::Environment::default())
                    .build()
                    .expect("Environment variables error");

        let config = c.try_deserialize();

        config
    }
}
