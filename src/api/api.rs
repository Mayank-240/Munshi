use crate::payload::payload::{Payload, Client, Subscription};
use crate::db::error::Error;
use crate::AppState;


use scylla::IntoTypedRows;
use actix_web::{post, web, web::Data, HttpResponse,};
use color_eyre::Result;


#[post("/")]
async fn ingest(item: web::Json<Payload>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    
    let session = &state.session;
    
    let my_obj: Payload = item.0;
    
    let query_result = session.query(
        "SELECT * from my_keyspace.events where transaction_id = ? and client_id = ? and time_stamp_epoch = ?",
        (my_obj.transaction_id, my_obj.client_id, my_obj.time_stamp)).await;

    match query_result{
        Ok(v) => if v.rows_num().unwrap() != 0{
                                return Err(Error {
                                    msg: "Invalid Payload. Duplicate Transaction ID.".to_string(),
                                    status: 400,
                                });
                            },
        Err(_x) => return Ok(HttpResponse::BadRequest().json("querry error"))
    }

    let query_result = session.query(
                "INSERT INTO my_keyspace.events (transaction_id, subscription_id, 
                    client_id, time_stamp_epoch, properties, created_at, updated_at) 
                    VALUES (?, ?, ?, ?, ?, dateof(now()), dateof(now()))", my_obj.clone()
            ).await;
    
    match query_result{
        Ok(_v) => Ok(HttpResponse::Ok().json(my_obj)),
        Err(_e) => Err(Error {
                                msg: "Data save failed.".to_string(),
                                status: 400,
                            }),
    } 
}


#[post("/client/create")]
async fn create_client(item: web::Json<Client>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    
    let session = &state.session;
    
    let obj: Client = item.0;
    let client_id = obj.client_id.clone();
    // checking if the client already exists
    let query_result = session.query(
                    "SELECT * from my_keyspace.clients where client_id = ?",
                    (client_id,)).await;
    
    match query_result{
        Ok(v) => 
            if let Some(rows) = v.rows{
                for row in rows.into_typed::<Client>() {
                    // Access the columns of each row
                    let my_client: Client = row.unwrap();
                    if my_client.client_id == obj.client_id {
                        return Err(Error {
                            msg: "Client already exists".to_string(),
                            status: 400,
                        });
                    }
                }
            } 
        Err(_x) => return Ok(HttpResponse::Ok().json("Querry Error"))
    }
    
    let query_result = session.query(
                        "INSERT INTO my_keyspace.clients (
                            client_id, time_stamp) 
                            VALUES (?, ?)",
                        (obj.client_id, obj.time_stamp),
                    ).await;
    
    match query_result{
        Ok(_v) => Ok(HttpResponse::Ok().json(obj)),
        Err(_e) => Err(Error {
                                msg: "Insert querry failed".to_string(),
                                status: 400,
                            }),
    } 
}



#[post("/subscription/create")]
async fn create_subscription(item: web::Json<Subscription>, state: Data<AppState>) -> Result<HttpResponse, Error> {
    
    let session = &state.session;
    
    let obj: Subscription = item.0;
    let subscription_id = obj.subscription_id.clone();
    let client_id = obj.client_id.clone();
    // checking if the client exists
    let query_client = session.query("SELECT * from my_keyspace.clients where client_id = ?",
                    (client_id,)).await;
    
    match query_client{
        Ok(v) => if v.rows_num().unwrap() == 0{
                                return Err(Error {
                                    msg: "Clientt does not exists".to_string(),
                                    status: 400,
                                })
                            },
        Err(_x) => return Err(Error {
                                msg: "querry failed".to_string(),
                                status: 400,
                            })
    }

    // checking if the subscription_id already exists
    let query_result = session.query("SELECT * from my_keyspace.subscriptions where subscription_id = ?",
                    (subscription_id,)).await;
    
    match query_result{
        Ok(v) =>if v.rows_num().unwrap() != 0{
                               return Err(Error {
                                    msg: "subscription already exists".to_string(),
                                    status: 400,
                                });
                    },
        Err(_x) =>return Err(Error {
                                    msg: "querry failed".to_string(),
                                    status: 400,
                                })
    }
    
    let query_result = session.query(
                    "INSERT INTO my_keyspace.subscriptions (
                        subscription_id, client_id, time_stamp) 
                        VALUES (?, ?, ?)",
                    (obj.subscription_id, obj.client_id, obj.time_stamp),
                ).await;
    
    match query_result{
        Ok(_v) => Ok(HttpResponse::Ok().json(obj)),
        Err(_e) => Err(Error {
                                msg: "Insert querry failed".to_string(),
                                status: 400,
                            }),
    }
}
