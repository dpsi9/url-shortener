// Settings::new() - Load configuration from env/files

// DatabaseSettings struct

// RedisSettings struct

// ServerSettings struct

// RateLimitingSettings struct
use std::env;

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn get_config() -> Self {
        Config {
            database_url: env::var("URI_POSTGRES").unwrap_or_else(|_| "UPDATE THIS LATER".into()),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "UPDATE THIS LATER".into()),
            server_addr: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8081".into()),
        }
    }
}
