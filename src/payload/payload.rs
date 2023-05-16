use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub transaction_id : uuid::Uuid,
    pub subscription_id : uuid::Uuid,
    pub client_id : uuid::Uuid,
    pub time_stamp: String,
    pub properties : HashMap<String,String>,
}