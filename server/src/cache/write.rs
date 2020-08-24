use crate::models::request::Request;
use darkredis::{ConnectionPool, Result};
use serde_json::to_string;
use uuid::Uuid;

pub async fn save_request(pool: &ConnectionPool, request_id: Uuid, data: &Request) -> Result<()> {
    let mut conn = pool.get().await;
    let data = to_string(data).expect("could not convert data to string");
    conn.set(request_id.to_string(), data.as_bytes()).await?;
    Ok(())
}
