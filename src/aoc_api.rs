use crate::utils;

use std::collections::HashMap;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AOCLeaderboard {
    #[serde(default = "utils::current_time")]
    #[serde(alias = "cache_invalidation")]
    pub cache_creation: u32,
    pub event: String,
    pub owner_id: String,
    pub members: HashMap<String, AOCUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AOCUser {
    #[serde(deserialize_with = "utils::string_or_int")]
    pub last_star_ts: u32,
    #[serde(deserialize_with = "utils::string_or_int")]
    pub stars: u32,
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "utils::string_or_int")]
    pub local_score: u32,
    #[serde(deserialize_with = "utils::string_or_int")]
    pub global_score: u32,
    pub completion_day_level: HashMap<String, HashMap<String, AOCDayLevel>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AOCDayLevel {
    #[serde(deserialize_with = "utils::string_or_int")]
    pub get_star_ts: u32,
}

pub async fn get_leaderboard() -> Result<AOCLeaderboard, impl std::error::Error> {
    let session_token = std::env::var("SESSION_COOKIE").expect("No session cookie provided");
    reqwest::Client::new()
        .get("https://adventofcode.com/2021/leaderboard/private/view/1550660.json")
        .header("cookie",&format!("session={}", session_token))
        .send()
        .await?
        .json()
        .await
}

