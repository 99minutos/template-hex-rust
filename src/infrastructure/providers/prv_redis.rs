use std::sync::Arc;

pub struct RedisProvider {
    database: Arc<redis::Connection>,
}

impl RedisProvider {
    pub async fn new(url: String) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        let mut conn = client.get_connection()?;

        redis::cmd("PING").query::<()>(&mut conn)?;

        let arc_client = Arc::new(conn);

        Ok(RedisProvider {
            database: arc_client,
        })
    }

    pub fn get_database(&self) -> Arc<redis::Connection> {
        self.database.clone()
    }
}
