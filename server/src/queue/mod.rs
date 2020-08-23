use crate::config::{SQS_ACCESS_KEY, SQS_PRIVATE_ACCESS_KEY, SQS_URL};
use rusoto_core::credential::StaticProvider;
use rusoto_core::request::HttpClient;
use rusoto_sqs::SqsClient;

// TODO: add await to retry connection
pub fn create_client() -> SqsClient {
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
    SqsClient::new_with(
        HttpClient::new().expect("couldnt make http client 🤷‍♀️"),
        provider,
        region,
    )
}
