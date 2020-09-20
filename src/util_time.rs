use std::time::SystemTime;

pub fn get_current_secs() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0 /* SHOULD NOT HAPPEN */ )
}

pub fn get_current_millis() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0 /* SHOULD NOT HAPPEN */ )
}
