use darkredis::{ConnectionPool, Result};
use uuid::Uuid;

pub async fn save_request(pool: &ConnectionPool, request_id: Uuid, data: String) -> Result<()> {
    let mut conn = pool.get().await;
    conn.set(request_id.to_string(), data.as_bytes()).await?;
    Ok(())
}
