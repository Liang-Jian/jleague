

use crate::config::get_config;
use crate::db::Db;
use crate::fetch::fetch_match_urls;
use chrono::NaiveDate;
use log::{info, warn, error};
use regex::Regex;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{thread, time::Duration};

/// 从 URL 中解析联赛等级（J1 / J2 / 天皇杯等）
fn get_level(url: &str) -> Option<&'static str> {
    let map = [
        ("j1", "A"), ("j2", "B"), ("j3", "C"),
        ("天皇杯", "K"), ("leaguecup", "L"),
    ];
    for (k, v) in &map {
        if url.contains(k) {
            return Some(v);
        }
    }
    None
}

/// 从单个比赛页面提取数据并生成 SQL
pub fn process_match_page(path: &str, db: &Db) {
    let full_url = format!("https://www.jleague.jp{}", path);
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    info!("开始请求: {}", full_url);
    let resp = match client.get(&full_url).send() {
        Ok(r) => r,
        Err(e) => {
            error!("请求失败: {}", e);
            return;
        }
    };

    let body = resp.text().unwrap_or_default();
    let document = Html::parse_document(&body);
    let span_sel = Selector::parse("p span").unwrap();

    let spans: Vec<_> = document.select(&span_sel).collect();
    if spans.len() < 15 {
        warn!("页面结构异常: {}", full_url);
        return;
    }

    let zhu = spans[0].inner_html();
    let ke = spans[spans.len() - 2].inner_html();
    let bc = format!("{}-{}", spans[3].inner_html(), spans[5].inner_html());
    let zc = spans[12].inner_html();
    let kc = spans[14].inner_html();

    // 正则提取轮次、联赛等级
    let round_re = Regex::new(r"第(.*?)節").unwrap();
    let league_sel = Selector::parse("span.matchVsTitle__league").unwrap();
    let round_text = document.select(&league_sel)
        .next().map(|e| e.inner_html()).unwrap_or_default();
    let round = round_re.captures(&round_text)
        .and_then(|cap| cap.get(1)).map(|m| m.as_str()).unwrap_or("0");

    // 比分
    let zj_sel = Selector::parse(".leagLeftScore").unwrap();
    let kj_sel = Selector::parse(".leagRightScore").unwrap();
    let zj = document.select(&zj_sel).next().map(|e| e.inner_html()).unwrap_or("0".into());
    let kj = document.select(&kj_sel).next().map(|e| e.inner_html()).unwrap_or("0".into());

    // 日期
    let date_re = Regex::new(r"2024/(\d{2}/\d{2})/live").unwrap();
    let date_cap = date_re.captures(path).and_then(|cap| cap.get(1)).map(|m| m.as_str());
    let date = match date_cap {
        Some(d) => NaiveDate::parse_from_str(&format!("2024/{}", d), "%Y/%m/%d").unwrap().to_string(),
        None => {
            warn!("未匹配到日期");
            return;
        }
    };

    // 开始时间
    let css_selector = get_config("control", "sj_css");
    let time_sel = Selector::parse(&css_selector).unwrap();
    let raw_text = document.select(&time_sel).next().map(|e| e.inner_html()).unwrap_or_default();
    let time_re = Regex::new(r"(\d{2}):(\d{2})").unwrap();
    let st = time_re.captures(&raw_text).map(|c| format!("{}{}", &c[1], &c[2])).unwrap_or("0000".into());

    let level = get_level(path).unwrap_or("X");
    let weather = "空";

    let sql = format!(
        "INSERT INTO `j24` VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '', '{}', '{}', '{}', '{}', '{}', '{}', '9.99', '9.99', '9.99');",
        date, st, level, round, weather, zhu, ke, bc, zj, kj, zc, kc
    );

    info!("构造 SQL: {}", sql);
    db.insert_match(&sql);
}

/// 遍历所有比赛链接并解析每一场
pub fn process_all_matches(db: &Db) {
    let urls = fetch_match_urls();
    for url in urls {
        process_match_page(&url, db);
        thread::sleep(Duration::from_millis(500)); // 限速防止被封
    }
}
