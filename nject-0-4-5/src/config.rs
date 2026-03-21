use anyhow::{Context, Result};

pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub nats_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
            redis_url: std::env::var("REDIS_URL").context("REDIS_URL is not set")?,
            nats_url: std::env::var("NATS_URL").context("NATS_URL is not set")?,
            server_port: std::env::var("SERVER_PORT")
                .context("SERVER_PORT is not set")?
                .parse()
                .context("SERVER_PORT is not a valid port number")?,
        })
    }
}
