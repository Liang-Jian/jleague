use serde::Deserialize;
use std::{fs, path::Path};
use once_cell::sync::OnceCell;
use anyhow::Result;

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

static INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config {
    /// 从 YAML 文件加载配置并储存为全局单例
    pub fn load<P: AsRef<Path>>(path: P) -> Result<&'static Config> {
        let s = fs::read_to_string(path)?;
        let cfg: Config = serde_yaml::from_str(&s)?;
        INSTANCE.set(cfg)
            .map_err(|_| anyhow::anyhow!("Config already initialized"))?;
        Ok(INSTANCE.get().unwrap())
    }

    /// 获取全局配置引用
    pub fn get() -> &'static Config {
        INSTANCE.get().expect("Config not initialized")
    }
}