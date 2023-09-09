use std::time::{Duration, SystemTime};

pub fn get_current_secs() -> u64 {
    get_secs(&SystemTime::now())
}

pub fn get_current_millis() -> u128 {
    get_millis(&SystemTime::now())
}

pub fn get_secs(system_time: &SystemTime) -> u64 {
    system_time.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0 /* SHOULD NOT HAPPEN */)
}

pub fn get_millis(system_time: &SystemTime) -> u128 {
    system_time.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0 /* SHOULD NOT HAPPEN */)
}

pub fn parse_duration(t: &str) -> Option<Duration> {
    if t.is_empty() {
        return None;
    }
    let parse_and_process_time = |mul: u32| {
        t[..t.len() - 1].parse::<f64>().map(|ti| Duration::from_millis((ti * mul as f64) as u64)).ok()
    };
    match t.to_ascii_lowercase().chars().last() {
        Some('s') => parse_and_process_time(1000),
        Some('m') => parse_and_process_time(60 * 1000),
        Some('h') => parse_and_process_time(60 * 60 * 1000),
        Some('d') => parse_and_process_time(24 * 60 * 60 * 1000),
        _ => t.parse::<u64>().map(Duration::from_millis).ok(),
    }
}

#[test]
fn test_get_current_secs() {
    assert_ne!(get_current_secs(), 0);
}

#[test]
fn test_get_current_millis() {
    assert_ne!(get_current_millis(), 0);
}

#[test]
fn test_parse_duration() {
    assert_eq!(None, parse_duration(""));
    assert_eq!(None, parse_duration("X"));
    assert_eq!(None, parse_duration("S"));
    assert_eq!(Duration::from_millis(1), parse_duration("1").unwrap());
    assert_eq!(Duration::from_millis(100), parse_duration("100").unwrap());
    assert_eq!(Duration::from_millis(1000), parse_duration("1s").unwrap());
    assert_eq!(Duration::from_millis(2000), parse_duration("2S").unwrap());
    assert_eq!(Duration::from_millis(1500), parse_duration("1.5s").unwrap());
    assert_eq!(Duration::from_millis(60000), parse_duration("1m").unwrap());
    assert_eq!(Duration::from_millis(3600000), parse_duration("1h").unwrap());
    assert_eq!(Duration::from_millis(1800000), parse_duration("0.5h").unwrap());
    assert_eq!(Duration::from_millis(24 * 3600000), parse_duration("1d").unwrap());
}