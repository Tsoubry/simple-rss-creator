use std::env;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub service_host: String,
    pub service_port: u16,
    pub workers: usize,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
}

pub fn get_settings() -> Settings {
    _ = dotenvy::dotenv();

    Settings {
        service_host: env::var("SERVICE_HOST").unwrap_or("0.0.0.0".to_string()),
        service_port: env::var("SERVICE_PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .expect("Wasn't able to parse SERVICE_PORT"),
        workers: env::var("WORKERS")
            .unwrap_or("1".to_string())
            .parse::<usize>()
            .expect("Wasn't able to parse WORKERS"),
        auth_username: env::var("AUTH_USERNAME").ok(),
        auth_password: env::var("AUTH_PASSWORD").ok(),
    }
}
