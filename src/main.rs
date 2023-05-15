mod config;
mod data;

extern crate serde_json;
// extern crate num_cpus;

use crate::config::Config;
// use tracing::debug;
// use crate::data::model::get_id_from_url;
// use crate::data::rest_api::TraversalNodeRequest;
// use crate::db::scylladb::ScyllaDbService;
// use crate::s3::s3::read_file;
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::Logger;
// use actix_web::web::Json;
use actix_web::{get, post, web, web:: Data, App, Error, HttpResponse, HttpServer};
use color_eyre::Result;
// use data::model::{Node, Relation, TraversalNode};
// use data::rest_api::{GetNodeRequest, IngestionRequest};
// use data::source_model::{Relation as SourceRelation, Nodes};
// use db::model::DbNode;
// use futures::future::{BoxFuture, FutureExt};
use std::collections::HashMap;
// use std::sync::Arc;
// use std::time::Instant;
// use tokio::sync::{AcquireError, OwnedSemaphorePermit, Semaphore};
// use tokio::task;
// use tokio::task::JoinHandle;
// use tracing::{debug, error, info};
// use uuid::Uuid;
use serde::{Deserialize, Serialize};
// use std::string::ToString;
// use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppState {
    // db_svc: ScyllaDbService,
    // semaphore: Arc<Semaphore>,
    // region: String,
    company_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    transaction_id : uuid::Uuid,
    subscription_id : uuid::Uuid,
    client_id : uuid::Uuid,
    time_stamp: String,
    properties : HashMap<String,String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComRes{
    payload: MyObj,
    company_name: String,
}

// This handler uses json extractor
#[post("/")]
async fn ingest(item: web::Json<MyObj>, state: Data<AppState>) -> HttpResponse {
    // println!("model: {:?}", &item);
    // debug!("Config: {:?}", state);
    // let state = state.AppState
    let resp = ComRes{
        payload:item.0,
        company_name:state.company_name.clone(),
    };
    HttpResponse::Ok().json(resp)
    // HttpResponse::Ok().json(item) // <- send response
}

// #[post("/ingest")]
// async fn ingest(
//     payload: web::Json<IngestionRequest>,
//     state: Data<AppState>,
// ) -> Result<HttpResponse, Error> {
//     info!("Ingest Request: {:?}", payload.files);
//     let now = Instant::now();

//     let mut handlers: Vec<JoinHandle<_>> = Vec::new();

//     for file in payload.files.iter() {
//         let permit = state.semaphore.clone().acquire_owned().await;
//         handlers.push(task::spawn(process_file(
//             payload.ingestion_id.clone(),
//             state.clone(),
//             file.to_string(),
//             permit,
//         )));
//     }

//     debug!("Waiting for files to be processed...");
//     for thread in handlers {
//         match thread.await {
//             Err(e) => return Err(ErrorInternalServerError(e)),
//             Ok(r) => {
//                 if let Err(e) = r {
//                     error!("Error: {:?}", e);
//                     return Err(ErrorInternalServerError(e));
//                 }
//             }
//         }
//     }

//     let elapsed = now.elapsed();
//     info!("Ingestion Time: {:.2?}", elapsed);
//     Ok(HttpResponse::Ok().json(r#"{ "status": "OK"}"#))
// }


#[actix_web::main]
async fn main() -> Result<()> {
    
    let config = Config::from_env().expect("Server Configuration");

    let port = config.port;
    let host = config.host;
    // let num_cpus = num_cpus::get();
    // let parallel_files = config.parallel_files;
    // let db_parallelism = config.db_parallelism;
    // let region = config.region;

    // info!(
    //     "Starting application. Num CPUs {}. Max Parallel Files {}. DB Parallelism {}.  Region {}",
    //     num_cpus, parallel_files, db_parallelism, region
    // );

    // let db = ScyllaDbService::new(config.db_dc, config.db_url, 
    //     db_parallelism, config.schema_file).await;

    // let sem = Arc::new(Semaphore::new(parallel_files));

    // info!("Starting server at http://{}:{}/", host, port);

    // let state = AppState{
    //     company_name : String::from("Finbox_Billing"),
    // };

    HttpServer::new( move || {
        App::new()
            // .wrap(Logger::default())
            .app_data(web::Data::new(AppState{
                company_name : String::from("Finbox_Billing")
            }))
            // .service(web::resource("/").route(web::post().to(ingest)))
            .service(ingest)
            // .service(get_by_id)
            // .service(traversal_by_id)
    })
    .bind(format!("{}:{}", host, port))?
    // .workers(num_cpus * 2)
    .run()
    .await?;

    Ok(())
}