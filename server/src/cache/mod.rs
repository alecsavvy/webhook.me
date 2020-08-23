use crate::config::REDIS_URL;
use darkredis::{ConnectionPool, Error, Result};
use std::str::from_utf8;
use uuid::Uuid;

pub mod read;
pub mod write;

pub async fn create_connection_pool() -> Result<ConnectionPool> {
    Ok(ConnectionPool::create(REDIS_URL.into(), None, num_cpus::get()).await?)
}
