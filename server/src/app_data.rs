use darkredis::ConnectionPool;
use rusoto_sqs::SqsClient;

pub struct AppData {
    pub sqs: SqsClient,
    pub queue_url: String,
    pub cache: ConnectionPool,
}
