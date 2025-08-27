use time::{Date, OffsetDateTime, UtcOffset};

pub fn now_local_date() -> Date {
    match UtcOffset::current_local_offset() {
        Ok(offset) => OffsetDateTime::now_utc().to_offset(offset).date(),
        Err(_) => OffsetDateTime::now_utc().date(),
    }
}
