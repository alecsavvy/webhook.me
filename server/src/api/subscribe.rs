use crate::app_data::AppData;
use crate::cache::write::save_request;
use crate::models::request::Request;
use crate::queue::operations::push_message;
use actix_web::{
    error, post,
    web::{Data, Json},
    Result,
};
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
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
pub async fn subscribe(
    app_data: Data<Mutex<AppData>>,
    data: Json<SubscribeRequest>,
) -> Result<Json<Request>> {
    let app_data = app_data.lock().expect("could not obtain lock on app data");
    // TODO: add header support
    let data = get(&data.endpoint)
        .await
        .map_err(|_| error::ErrorBadRequest("something blew up"))?
        .text()
        .await
        .map_err(|_| error::ErrorBadRequest("something else blew up"))?;

    let request_id = Uuid::new_v4();

    // save in redis
    save_request(&app_data.cache, request_id, data.clone())
        .await
        .map_err(|_| error::ErrorBadRequest("could not save in redis"))?;

    // push to sqs
    let sqs_message_id = push_message(&app_data.sqs, &app_data.queue_url, data.clone())
        .await
        .ok_or(error::ErrorBadRequest("could not push message to SQS"))?;

    let res = Request {
        data: data.clone(),
        request_id,
        sqs_message_id,
    };
    Ok(Json(res))
}
