// src/config.rs
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct ControlConfig {
    pub year: u16,
    pub level: String,
    pub round: u8,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CssConfig {
    pub level_css: String,
    pub sj_css: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub control: ControlConfig,
    pub db: DbConfig,
    pub css: CssConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let s = fs::read_to_string(path)?;
        let cfg = serde_yaml::from_str(&s)?;
        Ok(cfg)
    }
}
