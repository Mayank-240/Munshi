mod config;
mod payload;
mod db;

use crate::config::Config;
use crate::db::scylladb::ScyllaDbService;
use crate::payload::payload::Payload;

use scylla::Session;
use actix_web::{post, web, web::Data, App, Error, HttpResponse, HttpServer};
use color_eyre::Result;

struct AppState {
    session: Session,
}

#[post("/")]
async fn ingest(item: web::Json<Payload>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    
    let session = &state.session;
    
    let my_obj: Payload = item.0;
    let epoch_time = my_obj.time_stamp.clone();
    session
        .query(
            "INSERT INTO my_keyspace.user (transaction_id, subscription_id, client_id, time_stamp_epoch, time_stamp, created_at, updated_at, properties) VALUES (?, ?, ?, ?, ?, dateof(now()), dateof(now()), ?)",
            (my_obj.transaction_id, my_obj.subscription_id, my_obj.client_id, my_obj.time_stamp, epoch_time.clone(), my_obj.properties),
        )
        .await.expect("Entry into Database not successful.");
    
    Ok(HttpResponse::Ok().into()) 
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