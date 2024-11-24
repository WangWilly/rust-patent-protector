use chrono::prelude::{DateTime, Utc};
use std::time::SystemTime;

// https://stackoverflow.com/questions/64146345/how-do-i-convert-a-systemtime-to-iso-8601-in-rust
pub fn iso8601(st: &SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

pub fn parse_iso8601(s: &str) -> SystemTime {
    let dt = DateTime::parse_from_rfc3339(s).expect("Invalid date time");
    dt.into()
}

pub fn unix_timestamp(st: &SystemTime) -> i64 {
    st.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}
