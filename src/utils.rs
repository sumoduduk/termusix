use std::time::{SystemTime, UNIX_EPOCH};

pub mod convert;
pub mod rand;

pub fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clockwork may gone backward")
        .as_secs()
}
