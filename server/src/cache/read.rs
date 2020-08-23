use darkredis::{ConnectionPool, Error, Result};
use std::str::from_utf8;
use uuid::Uuid;

pub async fn read_request(pool: &ConnectionPool, request_id: Uuid) -> Result<String> {
    let mut conn = pool.get().await;
    let data = conn.get(request_id.to_string()).await?;
    let data = data.ok_or(Error::UnexpectedResponse(
        "could not find request id".into(),
    ))?;
    let data = from_utf8(&data)
        .map_err(|_| Error::UnexpectedResponse("could not convert byte to string".into()))?
        .to_string();
    Ok(data)
}

/// checks if passed data matches what's in the redis cache
/// returns true if a match, returns false if different
pub async fn check_request(pool: &ConnectionPool, request_id: Uuid, data: String) -> Result<bool> {
    Ok(read_request(pool, request_id).await?.eq(&data))
}
