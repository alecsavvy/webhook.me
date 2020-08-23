use crate::api::common::Status;
use actix_web::{error, post, web::Json, Result};
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    callback: String,
    endpoint: String,
    headers: HashMap<String, String>,
}

/// subscribes to an endpoint for continuous polling
/// makes initial GET request, saves in redis, then queues the next poll
/// returns request id along with initial GET response data unparsed
#[post("/subscribe")]
pub async fn subscribe(data: Json<SubscribeRequest>) -> Result<Json<Status>> {
    // TODO: add header support
    // only support for GET requests
    let data = get(&data.endpoint)
        .await
        .map_err(|_| error::ErrorBadRequest("something blew up"))?
        .text()
        .await
        .map_err(|_| error::ErrorBadRequest("something else blew up"))?;

    let request_id = Uuid::new_v4();

    let res = Status { data, request_id };

    // TODO: save request / response in redis
    // TODO: queue request in sqs
    // TODO: add response payload that's more than body
    Ok(Json(res))
}
