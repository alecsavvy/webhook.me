use actix_web::{web::Data, App, HttpServer};
use rusoto_sqs::Sqs;
use std::io::Result;
use std::sync::Mutex;

// internal modules
mod api;
mod app_data;
mod cache;
mod config;
mod queue;

#[actix_rt::main]
async fn main() -> Result<()> {
    let cache = cache::create_connection_pool()
        .await
        .expect("could not connect to redis");

    let sqs_client = queue::create_client();
    let req = rusoto_sqs::ListQueuesRequest::default();
    let queues = sqs_client.list_queues(req).await;

    let shared_data = app_data::AppData {
        sqs: sqs_client,
        redis: cache,
    };
    let shared_data = Data::new(Mutex::new(shared_data));

    // TODO: poll elastic mq until service is up
    println!("queues: {:?}", queues);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(api::health::check)
            .service(api::subscribe::subscribe)
            .service(api::status::status)
    })
    .bind(config::BINDING_ADDRESS)?
    .run()
    .await
}
