use std::process;
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
    CONFIG.get_or_init(EnvConfig::load)
}

impl EnvConfig {
    fn load() -> Self {
        dotenv::dotenv().ok();

        Self {
            port: parse_port(),
            service_name: require_env("SERVICE_NAME"),
            project_id: std::env::var("PROJECT_ID").ok(),
            mongo_uri: require_env("MONGO_URL"),
            mongo_db: require_env("MONGO_DB"),
            debug_level: std::env::var("DEBUG_LEVEL").unwrap_or_else(|_| "INFO".into()),
        }
    }
}

fn require_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| {
        eprintln!("Error: Missing required environment variable '{}'", name);
        process::exit(1);
    })
}

fn parse_port() -> u16 {
    let port_str = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    port_str.parse().unwrap_or_else(|_| {
        eprintln!("Error: PORT must be a valid number, got '{}'", port_str);
        process::exit(1);
    })
}
