use crate::config::SQS_MESSAGE_DELAY;
use rusoto_sqs::{SendMessageRequest, Sqs, SqsClient};

pub async fn push_message(
    client: &SqsClient,
    queue_url: &str,
    message_body: String,
) -> Option<String> {
    let msg = SendMessageRequest {
        delay_seconds: Some(SQS_MESSAGE_DELAY),
        queue_url: queue_url.to_string(),
        message_body,
        // unnecessary fields
        message_attributes: None,
        message_deduplication_id: None,
        message_group_id: None,
        message_system_attributes: None,
    };

    Some(
        client
            .send_message(msg)
            .await
            .ok()?
            .message_id
            .expect("pushed message to SQS but got no message id?"),
    )
}

pub async fn process_message() {}
