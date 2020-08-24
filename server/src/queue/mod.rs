use crate::config::{
    SQS_ACCESS_KEY, SQS_PRIVATE_ACCESS_KEY, SQS_QUEUE_NAME, SQS_URL, SQS_WARMUP_DELAY,
};
use rusoto_core::credential::StaticProvider;
use rusoto_core::request::HttpClient;
use rusoto_sqs::Sqs;
use rusoto_sqs::SqsClient;
use std::time::Duration;
use tokio::time::delay_for;

pub mod consumer;
pub mod operations;

// TODO: add await to retry connection
pub async fn create_client() -> (SqsClient, String) {
    // gather aws credentials
    let provider = StaticProvider::new_minimal(
        SQS_ACCESS_KEY.to_string(),
        SQS_PRIVATE_ACCESS_KEY.to_string(),
    );

    // elastic mq client
    let region = rusoto_core::region::Region::Custom {
        name: "local_sqs".to_owned(),
        endpoint: SQS_URL.to_string(),
    };
    let client = SqsClient::new_with(
        HttpClient::new().expect("couldnt make http client 🤷‍♀️"),
        provider,
        region,
    );

    // elastic mq takes a minute to start up, temporary solution
    delay_for(Duration::from_secs(SQS_WARMUP_DELAY)).await;

    let create_queue = rusoto_sqs::CreateQueueRequest {
        queue_name: SQS_QUEUE_NAME.to_string(),
        attributes: None,
        tags: None,
    };

    let queue = client
        .create_queue(create_queue)
        .await
        .expect("could not create default queue");

    let queue_url = queue
        .queue_url
        .expect("how could we make a queue with no url?");

    (client, queue_url)
}
