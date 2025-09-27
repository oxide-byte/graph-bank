use std::env;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Loki {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Tempo {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub loki: Loki,
    pub tempo: Tempo,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            // Local
            .add_source(File::with_name("graph-customer/config/default").required(false))
            .add_source(File::with_name(&format!("graph-customer/config/{run_mode}")).required(false))
            // Docker
            .add_source(File::with_name("/usr/local/bin/default").required(false))
            .add_source(File::with_name(&format!("/usr/local/bin/{run_mode}")).required(false))
            .build()?;

        s.try_deserialize()
    }
}