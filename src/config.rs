use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

const DEFAULT_CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct YaatsConfig {
    pub listen_address: String,
    pub redis_url: String,
    pub authorization_url: Option<String>,
    pub request_interval: usize,
    pub worker_threads: Option<usize>,
}

impl YaatsConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();

        let config_path =
            env::var("YAATS_CONFIG").unwrap_or_else(|_| String::from(DEFAULT_CONFIG_PATH));

        if PathBuf::from(&config_path).exists() {
            cfg.merge(File::with_name(&config_path))?;
        }

        cfg.merge(Environment::with_prefix("YAATS"))?;

        cfg.try_into()
    }
}
