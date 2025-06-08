

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::time::Duration;

/// 获取所有“試合詳細”链接（返回的是 `/match/.../live/`）
pub fn fetch_match_urls() -> Vec<String> {
    let url = "https://www.jleague.jp/match/";
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .danger_accept_invalid_certs(true) // 忽略 https 证书错误
        .build()
        .unwrap();

    let resp = match client.get(url).send() {
        Ok(resp) => resp,
        Err(err) => {
            log::error!("请求失败: {}", err);
            return vec![];
        }
    };

    let body = resp.text().unwrap_or_default();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("li a").unwrap();

    let mut urls = vec![];
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join("").trim().to_string();
        if text == "試合詳細" {
            if let Some(href) = element.value().attr("href") {
                urls.push(href.to_string());
            }
        }
    }

    if urls.is_empty() {
        log::warn!("未找到任何比赛详情链接");
    } else {
        log::info!("获取到 {} 个比赛链接", urls.len());
    }

    urls
}
