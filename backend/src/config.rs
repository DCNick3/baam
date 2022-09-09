use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Config {
    pub frontend: Frontend,
    pub server: Server,
}

impl Config {
    pub fn load(environment: &str) -> Result<Config> {
        let config = config::Config::builder()
            .add_source(config::File::new("config.yml", config::FileFormat::Yaml).required(false))
            .add_source(
                config::File::new(
                    &format!("config.{}.yml", environment),
                    config::FileFormat::Yaml,
                )
                .required(false),
            )
            .add_source(config::Environment::with_prefix("config"))
            .build()
            .context("Building the config file")?;

        config
            .try_deserialize()
            .context("Deserializing config structure")
    }
}

pub type Frontend = baam_frontend::Config;

#[derive(Deserialize)]
pub struct Server {
    pub endpoint: SocketAddr,
}
