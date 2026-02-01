use crate::config;
use mongodb::{Client, Database, options::ClientOptions};

#[derive(Clone)]
pub struct MongoProvider {
    db: Database,
}

impl MongoProvider {
    pub async fn new() -> Self {
        let env = config::get();
        let mut client_options = ClientOptions::parse(&env.mongo_url)
            .await
            .expect("Failed to parse MongoDB URI");

        client_options.app_name = Some("RustHexApp".to_string());

        let client =
            Client::with_options(client_options).expect("Failed to initialize MongoDB client");

        let db = client.database(&env.mongo_db);

        db.run_command(bson::doc! {"ping": 1})
            .await
            .expect("Failed to ping MongoDB");

        tracing::info!("Connected to MongoDB: {}", env.mongo_db);

        Self { db }
    }

    pub fn get_database(&self) -> Database {
        self.db.clone()
    }
}
