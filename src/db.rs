// src/db.rs
use mysql::{Pool, PooledConn};
use once_cell::sync::OnceCell;
use crate::config::DbConfig;
use anyhow::Result;
use mysql::prelude::*;
static DB_POOL: OnceCell<Pool> = OnceCell::new();

pub fn init_db(cfg: &DbConfig) -> anyhow::Result<()> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        cfg.user, cfg.password, cfg.host, cfg.port, cfg.dbname
    );
    // 这里把 String 转成 &str
    let pool = Pool::new(url.as_str())?;
    DB_POOL.set(pool)
        .map_err(|_| anyhow::anyhow!("DB pool already initialized"))?;
    Ok(())
}
pub fn get_conn() -> Result<PooledConn> {
    let pool = DB_POOL.get().ok_or_else(|| anyhow::anyhow!("DB not initialized"))?;
    Ok(pool.get_conn()?)
}

pub fn insert_sql(raw_sql: &str) {
    if let Ok(mut conn) = get_conn() {
        // 现在 exec_drop 就能被找到
        if let Err(e) = conn.exec_drop(raw_sql, ()) {
            log::error!("SQL 执行失败: {}\nSQL: {}", e, raw_sql);
        }
    }
}