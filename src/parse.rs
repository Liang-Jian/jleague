// src/parse.rs
use scraper::{Html, Selector};
use chrono::NaiveDate;
use anyhow::Result;

pub struct MatchData {
    pub date: NaiveDate,
    pub level: String,
    pub home_team: String,
    pub away_team: String,
    pub home_score: u8,
    pub away_score: u8,
}

pub fn parse_match(html: &str) -> Result<MatchData> {
    let doc = Html::parse_document(html);
    let date_sel = Selector::parse("div.date").unwrap();
    let level_sel = Selector::parse("div.level").unwrap();
    let teams_sel = Selector::parse("div.teams span.team-name").unwrap();
    let scores_sel = Selector::parse("div.teams span.score").unwrap();

    let date_str = doc.select(&date_sel).next().unwrap().text().collect::<String>();
    let date = NaiveDate::parse_from_str(date_str.trim(), "%Y/%m/%d")?;

    let level = doc.select(&level_sel).next().unwrap().text().collect::<String>().trim().to_string();

    let mut teams = doc.select(&teams_sel);
    let home_team = teams.next().unwrap().text().collect::<String>().trim().to_string();
    let away_team = teams.next().unwrap().text().collect::<String>().trim().to_string();

    let mut scores = doc.select(&scores_sel);
    let home_score = scores.next().unwrap().text().collect::<String>().trim().parse::<u8>()?;
    let away_score = scores.next().unwrap().text().collect::<String>().trim().parse::<u8>()?;

    Ok(MatchData { date, level, home_team, away_team, home_score, away_score })
}
