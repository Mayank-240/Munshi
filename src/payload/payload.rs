use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub transaction_id : uuid::Uuid,
    pub subscription_id : uuid::Uuid,
    pub client_id : uuid::Uuid,
    // correct time_stamp to time format
    pub time_stamp: i64,
    pub properties : HashMap<String,String>,
}