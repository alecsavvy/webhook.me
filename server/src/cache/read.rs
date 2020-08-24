use crate::models::request::Request;
use darkredis::{ConnectionPool, Error, Result};
use serde_json::from_str;
use std::str::from_utf8;
use uuid::Uuid;

pub async fn read_request(pool: &ConnectionPool, request_id: Uuid) -> Result<Request> {
    let mut conn = pool.get().await;
    let data = conn.get(request_id.to_string()).await?;
    let data = data.ok_or(Error::UnexpectedResponse(
        "could not find request id".into(),
    ))?;
    let data = from_utf8(&data)
        .map_err(|_| Error::UnexpectedResponse("could not convert byte to string".into()))?
        .to_string();

    let data = from_str::<Request>(&data).expect("could not read request from redis");
    Ok(data)
}
