// TODO: implement .env

/// Server
pub const BINDING_ADDRESS: &'static str = "0.0.0.0:3000";

/// Cache / Persistence
pub const REDIS_URL: &'static str = "redis:6379";

/// Queue
pub const SQS_ACCESS_KEY: &'static str = "local_access_key";
pub const SQS_CONSUMER_INTERVAL: u64 = 3;
pub const SQS_MESSAGE_DELAY: i64 = 30; // seconds
pub const SQS_PRIVATE_ACCESS_KEY: &'static str = "local_private_access_key";
pub const SQS_QUEUE_NAME: &'static str = "captain-hook";
pub const SQS_URL: &'static str = "http://elasticmq:9324";
pub const SQS_WARMUP_DELAY: u64 = 10; // seconds
