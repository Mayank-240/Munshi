use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use scylla::ValueList;
use scylla::macros::FromRow;

#[derive(ValueList, Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Payload {
    pub transaction_id : uuid::Uuid,
    pub subscription_id : uuid::Uuid,
    pub client_id : uuid::Uuid,
    // correct time_stamp to time format
    pub time_stamp: i64,
    pub properties : HashMap<String,String>,
}

#[derive(ValueList, Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Client{
    pub client_id: uuid::Uuid,
    pub time_stamp: i64,
}

#[derive(ValueList, Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Subscription{
    pub subscription_id: uuid::Uuid,
    pub client_id: uuid::Uuid,
    pub time_stamp: i64,
}