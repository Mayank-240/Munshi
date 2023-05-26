mod config;
mod payload;
mod db;
mod api;

use crate::api::api::{ingest, create_client, create_subscription};
use crate::config::Config;
use crate::db::scylladb::ScyllaDbService;

use scylla::Session;
use actix_web::{web::Data, App, HttpServer};
use color_eyre::Result;

pub struct AppState {
    session: Session,
}


#[actix_web::main]
async fn main() -> Result<()> {
    
    let config = Config::from_env().expect("Server Configuration");
    let port = config.port;
    let host = config.host;

    let session = ScyllaDbService::new(config.db_url,config.schema_file).await;
    let session = session.db_session;
    
    let data = Data::new(AppState{
        session,
    });

    HttpServer::new( move || {
        App::new()
            .app_data(data.clone())
            .service(ingest)
            .service(create_client)
            .service(create_subscription)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}