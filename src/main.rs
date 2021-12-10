mod utils;
mod aoc_api;

use aoc_api::*;

use std::time::Duration;
use std::io::{stdout, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    
    let leaderboard = if let Ok(file) = std::fs::read_to_string("cache.json") {
        let leaderboard: AOCLeaderboard = serde_json::from_str(&file)?;
        if utils::current_time() > leaderboard.cache_creation + 15 * 60 * 60 {
            get_leaderboard().await?
        }
        else { leaderboard }
    }
    else {
        get_leaderboard().await?
    };
    std::fs::write("cache.json", serde_json::to_string(&leaderboard)?)?;

    queue!(stdout,
        style::Print(format!("Event Name: '{}'\n", leaderboard.event)),
        style::Print(format!("Members ({}):\n", leaderboard.members.len())),
    )?;

    let mut members = leaderboard.members.values().collect::<Vec<_>>();
    members.sort_by_key(|m| u32::MAX - m.local_score);
    for (i, member) in members.iter().enumerate() {
        let padding = " ".repeat(2 - (i as f32 + 1.).log10().floor() as usize);
        queue!(stdout,
            style::Print(format!("{}({}) {}\n", padding, i + 1, member.name)),
            style::Print(format!("       Stars: {}\n", member.stars)),
            style::Print(format!("       Local Score: {}\n", member.local_score)),
        )?;
        let last_day_id = member.completion_day_level.keys().filter_map(|a| a.parse::<u32>().ok()).max().unwrap_or(0);
        queue!(stdout,
            style::Print(format!("       Last Day: {}\n", last_day_id))
        )?;
        let last_day = member.completion_day_level.get(&last_day_id.to_string());
        let st_first_star = last_day.map(|last_day|
            last_day.get("1").map(|a| a.get_star_ts)
        ).flatten();
        if let Some(sfs) = st_first_star {
            queue!(stdout,
                style::Print(format!("       Time2First: {}\n", utils::format_time(Duration::from_secs((sfs - utils::get_aoc_instant(last_day_id)) as u64))))
            )?;
        }
        let st_second_star = last_day.map(|last_day|
            last_day.get("2").map(|a| a.get_star_ts)
        ).flatten();
        match (st_first_star, st_second_star) {
            (Some(sfs), Some(sss)) => {
                queue!(stdout,
                    style::Print(format!("       Time2Second: {}\n", utils::format_time(Duration::from_secs((sss - sfs) as u64))))
                )?;
            }
            _ => ()
        }
    }

    stdout.flush()?;
    Ok(())
}
