use crate::api::common::Status;
use actix_web::{
    get,
    web::{Json, Path},
    HttpRequest, Result,
};
use uuid::Uuid;

#[get("/status/{request_id}")]
pub async fn status(_req: HttpRequest, request_id: Path<Uuid>) -> Result<Json<Status>> {
    let res = Status {
        request_id: request_id.into_inner(),
        data: "".into(),
    };
    Ok(Json(res))
}
