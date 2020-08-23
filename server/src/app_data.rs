use darkredis::ConnectionPool;
use rusoto_sqs::SqsClient;

pub struct AppData {
    pub sqs: SqsClient,
    pub cache: ConnectionPool,
}
