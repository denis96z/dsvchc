use std::collections::BTreeMap;
use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "server")]
    pub server: ServerConfig,

    #[serde(rename = "plugins")]
    pub plugins: BTreeMap<String, PluginConfig>,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    #[serde(rename = "port", default)]
    pub port: u16,
}

#[derive(Deserialize)]
pub struct PluginConfig {
    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "conf_path")]
    pub conf_path: String,

    #[serde(rename = "checks")]
    pub checks: BTreeMap<String, CheckConfig>,
}

#[derive(Deserialize)]
pub struct CheckConfig {
    #[serde(rename = "conf_path")]
    pub conf_path: String,
}

impl Config {
    pub fn load(conf_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let f = File::open(conf_path)?;
        let conf = serde_yaml::from_reader(f)?;
        Ok(conf)
    }
}
