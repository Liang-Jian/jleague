

use mysql::*;
use mysql::prelude::*;
use std::sync::{Mutex, OnceLock};
use crate::config::Config;

/// 单例数据库连接池（使用 OnceLock）
static DB_POOL: OnceLock<Mutex<PooledConn>> = OnceLock::new();

pub fn init_db(config: &Config) {
    let url = format!(
        "mysql://{}:{}@{}/{}",
        config.get("user", "db").unwrap(),
        config.get("password", "db").unwrap(),
        config.get("host", "db").unwrap(),
        config.get("dbname", "db").unwrap()
    );
    let pool = Pool::new(url).expect("Failed to create MySQL pool");
    let conn = pool.get_conn().expect("Failed to get MySQL connection");

    DB_POOL.set(Mutex::new(conn)).unwrap_or_else(|_| {
        panic!("DB_POOL already initialized")
    });
}

pub fn search_date_exists(date: &str) -> bool {
    let query = format!("SELECT * FROM j24 WHERE date = '{}'", date);
    let mut conn = DB_POOL.get().unwrap().lock().unwrap();
    let result: Vec<Row> = conn.query(query).expect("Query failed");
    !result.is_empty()
}

pub fn insert_sql(raw_sql: &str) {
    let mut conn = DB_POOL.get().unwrap().lock().unwrap();
    if let Err(e) = conn.query_drop(raw_sql) {
        log::error!("SQL 执行失败: {}\nSQL: {}", e, raw_sql);
    } else {
        log::info!("✅ 插入成功！");
    }
}
