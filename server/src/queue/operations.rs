use crate::config::SQS_MESSAGE_DELAY;
use rusoto_sqs::{ReceiveMessageRequest, SendMessageRequest, Sqs, SqsClient};
use uuid::Uuid;

pub async fn push_message(client: &SqsClient, queue_url: &str, request_id: Uuid) -> Option<String> {
    let msg = SendMessageRequest {
        delay_seconds: Some(SQS_MESSAGE_DELAY),
        queue_url: queue_url.to_string(),
        message_body: request_id.to_string(),
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

pub async fn get_message(client: &SqsClient, queue_url: &str) -> Option<Uuid> {
    let mut req = ReceiveMessageRequest::default();
    req.queue_url = queue_url.to_string();

    let msg = client.receive_message(req).await.ok()?;
    let msg = msg.messages?.pop()?.body?;
    Uuid::parse_str(&msg).ok()
}
