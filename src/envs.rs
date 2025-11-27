use std::sync::OnceLock;

#[derive(Debug)]
pub struct EnvConfig {
    pub port: u16,
    pub service_name: String,
    pub project_id: Option<String>,
    pub mongo_uri: String,
    pub mongo_db: String,
    pub debug_level: String,
}

static CONFIG: OnceLock<EnvConfig> = OnceLock::new();
pub fn get() -> &'static EnvConfig {
    CONFIG.get_or_init(|| EnvConfig::new())
}

impl EnvConfig {
    fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            port: std::env::var("PORT")
                .unwrap_or("8080".to_string())
                .parse()
                .unwrap(),
            project_id: std::env::var("PROJECT_ID").ok(),
            service_name: std::env::var("SERVICE_NAME").expect("SERVICE_NAME is required"),
            mongo_uri: std::env::var("MONGO_URL").expect("MONGO_URL is required"),
            mongo_db: std::env::var("MONGO_DB").expect("MONGO_DB is required"),
            debug_level: std::env::var("DEBUG_LEVEL").unwrap_or("INFO".to_string()),
        }
    }
}
