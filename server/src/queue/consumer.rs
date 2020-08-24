use crate::config::SQS_CONSUMER_INTERVAL;
use crate::models::request::Request;
use crate::queue::operations::get_message;
use crate::{cache, cache::read::read_request, queue};
use reqwest::get;
use std::time::Duration;
use tokio::{stream::StreamExt, time};

/// future that never completes
/// polls SQS for new messages and processing
pub async fn consumer() {
    // create it's own connections to redis
    let cache = cache::create_connection_pool()
        .await
        .expect("could not connect to redis in consumer");

    // SQS is idempotent, recreating the same queue is harmless
    let (sqs, queue_url) = queue::create_client().await;

    loop {
        time::interval(Duration::from_secs(SQS_CONSUMER_INTERVAL))
            .next()
            .await;

        // handle this in a result
        let request_id = get_message(&sqs, &queue_url).await;

        if request_id.is_none() {
            println!("didn't find any messages");
            continue;
        }
        let request_id = request_id.unwrap();

        println!("got message! {}", request_id);

        let request = read_request(&cache, request_id).await.ok();
        if request.is_none() {
            println!("could not find request in cache");
            continue;
        }
        let request = request.unwrap();
        let endpoint = request.endpoint.clone();
        let body = check_data(&request).await;
        let _ = callback(endpoint, body);

        // TODO: push new message up
        // TODO: delete message
    }
}

/// returns body of get request if different
async fn check_data(request: &Request) -> Option<String> {
    let old_data = request.data.to_string();

    let data = get(&request.endpoint).await.ok()?.text().await.ok()?;
    if old_data.eq(&data) {
        None
    } else {
        Some(data) // data has updated
    }
}

async fn callback(endpoint: String, body: Option<String>) -> Option<()> {
    let body = body?;
    let client = reqwest::Client::new();
    println!("callback!");
    client
        .post(&endpoint)
        .body(body)
        .send()
        .await
        .ok()
        .map(|_| ())
}
