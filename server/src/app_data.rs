use darkredis::ConnectionPool;
use rusoto_sqs::SqsClient;

pub struct AppData {
    pub sqs: SqsClient,
    pub redis: ConnectionPool,
}
