use dotenvy::dotenv;
use std::env;
use std::process;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Env {
    pub port: u16,
    pub app_env: String,
    pub service_name: String,
    #[allow(dead_code)]
    pub project_id: String,
    pub mongo_url: String,
    pub mongo_db: String,
    #[allow(dead_code)]
    pub redis_url: String,
    pub debug_level: String,
    #[allow(dead_code)]
    pub storage_bucket: String,
    pub cors_origins: String,
}

static CONFIG: OnceLock<Env> = OnceLock::new();

pub fn get() -> &'static Env {
    CONFIG.get_or_init(Env::load)
}

impl Env {
    fn load() -> Self {
        dotenv().ok();

        Self {
            port: parse_port(),
            service_name: require_env("SERVICE_NAME"),
            app_env: std::env::var("APP_ENV").unwrap_or_else(|_| "DEV".to_string()),
            project_id: require_env("PROJECT_ID"),
            mongo_url: require_env("MONGO_URL"),
            mongo_db: require_env("MONGO_DB"),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            debug_level: std::env::var("DEBUG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            storage_bucket: std::env::var("STORAGE_BUCKET").unwrap_or_default(),
            cors_origins: std::env::var("CORS_ORIGINS").unwrap_or_else(|_| "*".to_string()),
        }
    }
}

fn require_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        eprintln!(
            "CRITICAL ERROR: Missing required environment variable '{}'",
            name
        );
        process::exit(1);
    })
}

fn parse_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".into());
    port_str.parse().unwrap_or_else(|_| {
        eprintln!(
            "CRITICAL ERROR: PORT must be a valid number, got '{}'",
            port_str
        );
        process::exit(1);
    })
}
