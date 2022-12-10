use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub frontend: Frontend,
    pub server: Server,
    pub challenge: Challenge,
    pub sentry_tunnel: Option<Sentry>,
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

#[derive(Deserialize, Clone, Debug)]
pub struct Server {
    pub endpoint: SocketAddr,
}

pub type Challenge = crate::api::ChallengeConfig;
pub type Sentry = crate::sentry_tunnel::Config;
