
pub struct MongoProvider {
    database: mongodb::Database,
}

impl MongoProvider {
    pub async fn new(url: String, database_name: String) -> Result<Self, mongodb::error::Error> {
        let client = mongodb::Client::with_uri_str(url).await?;
        let database_conn = client.database(database_name.as_str());

        _ = database_conn.run_command(bson::doc! {"ping": 1}).await?;

        Ok(MongoProvider {
            database: database_conn,
        })
    }

    pub fn get_database(&self) -> mongodb::Database {
        self.database.clone()
    }
}
