use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub data: String, // this could be any structure
    pub request_id: Uuid,
    pub sqs_message_id: String,
    pub endpoint: String,
    pub callback: String,
}
