use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Status {
    pub data: String, // this could be any structure
    pub request_id: Uuid,
    pub sqs_message_id: String,
}
