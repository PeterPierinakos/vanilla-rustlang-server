use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn generate_unixtime() -> Result<u64, SystemTimeError> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(time.as_secs())
}
