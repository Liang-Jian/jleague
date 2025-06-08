// src/fetch.rs
use once_cell::sync::OnceCell;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use futures::future::join_all;
use crate::parse::parse_match;
use crate::db::insert_sql;

static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();

fn client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Client init failed")
    })
}

pub async fn fetch_match_urls() -> Result<Vec<String>> {
    let url = "https://www.jleague.jp/match/";
    let body = client().get(url).send().await?.text().await?;
    let doc = Html::parse_document(&body);
    let sel = Selector::parse("a.match-link").unwrap();
    let urls = doc
        .select(&sel)
        .filter_map(|e| {
            let txt = e.text().collect::<String>();
            if txt.trim() == "試合詳細" {
                e.value().attr("href").map(str::to_string)
            } else {
                None
            }
        })
        .collect();
    Ok(urls)
}

pub async fn process_match_page(url: &str, client: &Client) -> Result<()> {
    let body = client.get(url).send().await?.text().await?;
    let data = parse_match(&body)?;
    let sql = format!(
        "INSERT INTO matches (date, level, home_team, away_team, home_score, away_score) VALUES ('{}', '{}', '{}', '{}', {}, {})",
        data.date.format("%Y-%m-%d"),
        data.level,
        data.home_team,
        data.away_team,
        data.home_score,
        data.away_score
    );
    insert_sql(&sql);
    Ok(())
}

pub async fn process_all_matches() -> Result<()> {
    let urls = fetch_match_urls().await?;
    let semaphore = Arc::new(Semaphore::new(5));
    let client_ref = client().clone();
    let tasks = urls.into_iter().map(|url| {
        let client = client_ref.clone();
        let sem = Arc::clone(&semaphore);
        tokio::spawn(async move {
            let permit: OwnedSemaphorePermit = sem.acquire_owned().await.unwrap();
            let _ = process_match_page(&url, &client).await;
            drop(permit);
        })
    });
    join_all(tasks).await;
    Ok(())
}
