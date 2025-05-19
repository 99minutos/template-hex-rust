use std::sync::Arc;

pub struct MongoProvider {
    database: Arc<mongodb::Database>,
}

impl MongoProvider {
    pub async fn new(url: String, database_name: String) -> Result<Self, mongodb::error::Error> {
        let client = mongodb::Client::with_uri_str(url).await?;
        let database_conn = client.database(database_name.as_str());

        _ = database_conn.run_command(bson::doc! {"ping": 1}).await?;

        let arc_database = Arc::new(database_conn);
        Ok(MongoProvider {
            database: arc_database,
        })
    }

    pub fn get_database(&self) -> Arc<mongodb::Database> {
        self.database.clone()
    }
}
