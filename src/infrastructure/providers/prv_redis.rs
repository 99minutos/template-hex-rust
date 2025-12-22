#![allow(dead_code)]
use std::sync::Arc;

pub struct RedisProvider {
    database: Arc<redis::Connection>,
    prefix: String,
}

impl RedisProvider {
    pub async fn new(url: &str, prefix: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        let mut conn = client.get_connection()?;

        redis::cmd("PING").query::<()>(&mut conn)?;

        let arc_client = Arc::new(conn);

        Ok(RedisProvider {
            database: arc_client,
            prefix: prefix.to_string(),
        })
    }

    pub fn get_database(&self) -> Arc<redis::Connection> {
        self.database.clone()
    }

    pub fn get_path(&self, key: &[&str]) -> String {
        format!("{}:{}", self.prefix, key.join(":"))
    }
}
