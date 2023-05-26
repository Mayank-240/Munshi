mod config;
mod payload;
mod db;

use crate::config::Config;
use crate::db::scylladb::ScyllaDbService;
use crate::payload::payload::{Payload, Client, Subscription};

use scylla::IntoTypedRows;
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

    session
        .query(
            "INSERT INTO my_keyspace.events (transaction_id, subscription_id, 
                client_id, time_stamp_epoch, properties, created_at, updated_at) 
                VALUES (?, ?, ?, ?, ?, dateof(now()), dateof(now()))", my_obj
        )
        .await.expect("Entry of event not successful.");

    Ok(HttpResponse::Ok().into()) 
}


#[post("/client/create")]
async fn create_client(item: web::Json<Client>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    
    let session = &state.session;
    
    let obj: Client = item.0;
    let client_id = obj.client_id.clone();
    // // checking if the client already exists
    let query_result = session.query("SELECT * from my_keyspace.clients where client_id = ?",
                    (client_id,)).await;
    
    match query_result{
        Ok(v) =>if let Some(rows) = v.rows{
                                for row in rows.into_typed::<Client>() {
                                    // Access the columns of each row
                                    let my_client: Client = row.unwrap();
                                    if my_client.client_id == obj.client_id {
                                        return Ok(HttpResponse::Ok().body("Client already exists"));
                                    }
                                }
                            }
        Err(_x) =>return Ok(HttpResponse::Ok().body("Querry Error"))
    }
    
    session
        .query(
            "INSERT INTO my_keyspace.clients (
                client_id, time_stamp) 
                VALUES (?, ?)",
            (obj.client_id, obj.time_stamp),
        )
        .await.expect("Entry into clients not successful.");
    
    Ok(HttpResponse::Ok().json(obj)) 
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
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}