use crate::app_data::AppData;
use crate::cache::read::read_request;
use crate::models::request::Request;
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
) -> Result<Json<Request>> {
    let app_data = app_data.lock().expect("could not obtain lock on app data");
    let res = read_request(&app_data.cache, request_id.clone())
        .await
        .map_err(|_| error::ErrorBadRequest("could not find that request"))?;
    Ok(Json(res))
}
