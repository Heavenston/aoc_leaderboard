use serde::de::{ self, Deserializer, Visitor };
use std::time::Duration;
use chrono::prelude::*;
use chrono::{ TimeZone, Utc };

pub fn string_or_int<'de, D>(deserializer: D) -> Result<u32, D::Error>
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

pub fn current_time() -> u32 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32
}

pub fn format_time(duration: Duration) -> String {
    let mut seconds = duration.as_secs();
    let mut mins = seconds / 60;
    seconds -= mins * 60;
    let hours = mins / 60;
    mins -= hours * 60;

    format!("{}h{}min{}s", hours, mins, seconds)
}

pub fn get_aoc_instant(day: u32) -> u32 {
    Utc.ymd(Utc::now().year(), 12, day).and_hms(5, 0, 0).timestamp() as u32
}
