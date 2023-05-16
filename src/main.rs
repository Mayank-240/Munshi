mod config;
mod db;

use crate::config::Config;
use crate::db::scylladb::ScyllaDbService;

use scylla::Session;
use actix_web::{post, web, web::Data, App, HttpResponse, HttpServer};
use color_eyre::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

struct AppState {
    session: Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MyObj {
    transaction_id : uuid::Uuid,
    subscription_id : uuid::Uuid,
    client_id : uuid::Uuid,
    time_stamp: String,
    properties : HashMap<String,String>,
}

#[post("/")]
async fn ingest(item: web::Json<MyObj>, state: Data<AppState>) -> HttpResponse {
    
    let session = &state.session;
    
    let my_obj: MyObj = item.0;
    let my_obj2 = my_obj.clone();

    session
        .query(
            "INSERT INTO my_keyspace.user (transaction_id, subscription_id, client_id, time_stamp, properties) VALUES (?, ?, ?, ?, ?)",
            (my_obj.transaction_id, my_obj.subscription_id, my_obj.client_id, my_obj.time_stamp, my_obj.properties),
        )
        .await.expect("Entry into Database not successful.");
    
    HttpResponse::Ok().json(my_obj2) 
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
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}