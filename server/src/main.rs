use actix_web::{web::Data, App, HttpServer};
use std::io::Result;
use std::sync::Mutex;
use tokio::spawn;

// internal modules
mod api;
mod app_data;
mod cache;
mod config;
mod models;
mod queue;

use queue::consumer::consumer;

#[actix_rt::main]
async fn main() -> Result<()> {
    let cache = cache::create_connection_pool()
        .await
        .expect("could not connect to redis");

    let (sqs, queue_url) = queue::create_client().await;

    let shared_data = app_data::AppData {
        sqs,
        cache,
        queue_url,
    };
    let shared_data = Data::new(Mutex::new(shared_data));

    // spawn SQS consumer thread
    spawn(async {
        consumer().await;
    });

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
