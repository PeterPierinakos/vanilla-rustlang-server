use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_unixtime() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
