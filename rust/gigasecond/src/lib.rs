extern crate chrono;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let mut offset = 1;
    for _ in 0..9 {
        offset = offset * 10;
    }
    start + chrono::Duration::seconds(offset)
}
