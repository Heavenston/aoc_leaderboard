use std::collections::HashMap;
use serde::{Serialize, Deserialize, de::{ self, Deserializer, Visitor }};
use std::time::Duration;
use chrono::prelude::*;
use chrono::{ TimeZone, Utc};

fn string_or_int<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    use std::marker::PhantomData;
    use std::fmt;

    struct StringOrInt(PhantomData<fn() -> u32>);
    impl<'de> Visitor<'de> for StringOrInt {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or int")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<u32, E>
        {
            Ok(value.parse::<u32>().unwrap())
        }

        fn visit_u32<E: de::Error>(self, value: u32) -> Result<u32, E> {
            Ok(value)
        }
        fn visit_u64<E: de::Error>(self, value: u64) -> Result<u32, E> {
            Ok(value as u32)
        }
    }

    deserializer.deserialize_any(StringOrInt(PhantomData))
}

fn current_time() -> u32 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AOCLeaderboard {
    #[serde(default = "current_time")]
    #[serde(alias = "cache_invalidation")]
    cache_creation: u32,
    event: String,
    owner_id: String,
    members: HashMap<String, AOCUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AOCUser {
    #[serde(deserialize_with = "string_or_int")]
    last_star_ts: u32,
    #[serde(deserialize_with = "string_or_int")]
    stars: u32,
    id: String,
    name: String,
    #[serde(deserialize_with = "string_or_int")]
    local_score: u32,
    #[serde(deserialize_with = "string_or_int")]
    global_score: u32,
    completion_day_level: HashMap<String, HashMap<String, AOCDayLevel>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct AOCDayLevel {
    #[serde(deserialize_with = "string_or_int")]
    get_star_ts: u32,
}

async fn get_leaderboard() -> Result<AOCLeaderboard, impl std::error::Error> {
    let session_token = std::env::var("SESSION_COOKIE").expect("No session cookie provided");
    reqwest::Client::new()
        .get("https://adventofcode.com/2021/leaderboard/private/view/1550660.json")
        .header("cookie",&format!("session={}", session_token))
        .send()
        .await?
        .json()
        .await
}

fn format_time(duration: Duration) -> String {
    let mut seconds = duration.as_secs();
    let mut mins = seconds / 60;
    seconds -= mins * 60;
    let hours = mins / 60;
    mins -= hours * 60;

    format!("{}h{}min{}s", hours, mins, seconds)
}

fn get_aoc_instant(day: u32) -> u32 {
    Utc.ymd(Utc::now().year(), 12, day).and_hms(5, 0, 0).timestamp() as u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let leaderboard = if let Ok(file) = std::fs::read_to_string("cache.json") {
        let leaderboard: AOCLeaderboard = serde_json::from_str(&file)?;
        if current_time() > leaderboard.cache_creation + 15 * 60 * 60 {
            get_leaderboard().await?
        }
        else { leaderboard }
    }
    else {
        get_leaderboard().await?
    };
    std::fs::write("cache.json", serde_json::to_string(&leaderboard)?)?;

    println!("Event Name: '{}'", leaderboard.event);
    println!("Members ({}):", leaderboard.members.len());

    let mut members = leaderboard.members.values().collect::<Vec<_>>();
    members.sort_by_key(|m| u32::MAX - m.local_score);
    for (i, member) in members.iter().enumerate() {
        let padding = " ".repeat(2 - (i as f32 + 1.).log10().floor() as usize);
        println!("{}({}) {}", padding, i + 1, member.name);
        println!("       Stars: {}", member.stars);
        println!("       Local Score: {}", member.local_score);
        let last_day_id = member.completion_day_level.keys().filter_map(|a| a.parse::<u32>().ok()).max().unwrap_or(0);
        println!("       Last Day: {}", last_day_id);
        let last_day = member.completion_day_level.get(&last_day_id.to_string());
        let st_first_star = last_day.map(|last_day|
            last_day.get("1").map(|a| a.get_star_ts)
        ).flatten();
        if let Some(sfs) = st_first_star {
            println!("       Time2First: {}", format_time(Duration::from_secs((sfs - get_aoc_instant(last_day_id)) as u64)));
        }
        let st_second_star = last_day.map(|last_day|
            last_day.get("2").map(|a| a.get_star_ts)
        ).flatten();
        match (st_first_star, st_second_star) {
            (Some(sfs), Some(sss)) => {
                println!("       Time2Second: {}", format_time(Duration::from_secs((sss - sfs) as u64)));
            }
            _ => ()
        }
    }

    Ok(())
}
