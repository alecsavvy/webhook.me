use crate::api::common::Status;
use crate::app_data::AppData;
use crate::cache::read_request;
use actix_web::{
    error, get,
    web::{Data, Json, Path},
    Result,
};
use std::sync::Mutex;
use uuid::Uuid;

#[get("/status/{request_id}")]
pub async fn status(
    app_data: Data<Mutex<AppData>>,
    request_id: Path<Uuid>,
) -> Result<Json<Status>> {
    let app_data = app_data.lock().expect("could not obtain lock on app data");
    let request = read_request(&app_data.cache, request_id.clone())
        .await
        .map_err(|_| error::ErrorBadRequest("could not find that request"))?;
    let res = Status {
        request_id: request_id.clone(),
        data: request,
        sqs_message_id: "".into(), // not sure how to include this yet
    };
    Ok(Json(res))
}
