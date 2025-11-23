use time::{Duration, PrimitiveDateTime};

/// Return the moment exactly one gigasecond after the provided start time.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + Duration::seconds(1_000_000_000)
}
