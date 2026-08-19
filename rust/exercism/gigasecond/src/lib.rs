use time::PrimitiveDateTime as DateTime;
use time::SignedDuration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = SignedDuration::seconds(1_000_000_000);
    start + gigasecond
}
