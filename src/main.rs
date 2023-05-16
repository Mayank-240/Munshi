mod config;
mod data;

extern crate serde_json;

use scylla::{Session, SessionBuilder, FromRow};
use std::error::Error;
use crate::config::Config;
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, web:: Data, App, HttpResponse, HttpServer};
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



// This handler uses json extractor
#[post("/")]
async fn ingest(item: web::Json<MyObj>, state: Data<AppState>) -> HttpResponse {
    
    let session = &state.session;
    
    let my_obj: MyObj = item.0;
    let my_obj2 = my_obj.clone();
    // keyspace : my_keyspace to be created in docker terminal
    // table : user to be created in docker terminal
    // CREATE TABLE IF NOT EXISTS my_keyspace.user (
    //     transaction_id UUID PRIMARY KEY,
    //     subscription_id UUID,
    //     client_id UUID,
    //     time_stamp TEXT,
    //     properties MAP<TEXT,TEXT>,
    // )
    session
        .query(
            "INSERT INTO my_keyspace.user (transaction_id, subscription_id, client_id, time_stamp, properties) VALUES (?, ?, ?, ?, ?)",
            (my_obj.transaction_id, my_obj.subscription_id, my_obj.client_id, my_obj.time_stamp, my_obj.properties),
        )
        .await.expect("Entry into Database not successful.");
    
    HttpResponse::Ok().json(my_obj2) // <- send response
}


#[actix_web::main]
async fn main() -> Result<()> {
    
    let config = Config::from_env().expect("Server Configuration");
    
    let uri = "127.0.0.1:9042".to_string();

    let session: Session = SessionBuilder::new()
        .known_node(uri)
        .build()
        .await?;

    // Create an example keyspace and table
    // session
    //     .query(
    //         "CREATE KEYSPACE IF NOT EXISTS my_keyspace WITH REPLICATION = \
    //         {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
    //         &[],
    //     )
    //     .await?;

    let port = config.port;
    let host = config.host;

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