use std::collections::HashMap;
use serde::{Serialize, Deserialize, de::{ self, Deserializer, Visitor }};

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
    cache_invalidate: u32,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let leaderboard = if let Ok(file) = std::fs::read_to_string("cache.json") {
        let leaderboard: AOCLeaderboard = serde_json::from_str(&file)?;
        if current_time() < leaderboard.cache_invalidate {
            get_leaderboard().await?
        }
        else { leaderboard }
    }
    else {
        get_leaderboard().await?
    };
    std::fs::write("cache.json", serde_json::to_string(&leaderboard)?)?;

    println!("{:#?}", leaderboard);
    Ok(())
}
