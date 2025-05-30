use chrono::{DateTime, Local, Utc};
use core::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum EventTime {
    DateTime(TimeType),
    DateOnly(TimeType),
}
impl EventTime {
    #[cfg(test)]
    pub fn fixed_utc_time() -> DateTime<Utc> {
        let utc_str_z = "2025-05-30T17:51:11Z";
        chrono::DateTime::parse_from_rfc3339(utc_str_z)
            .unwrap()
            .to_utc()
    }
    #[cfg(test)]
    pub fn fixed_utc() -> Self {
        EventTime::DateTime(TimeType::Utc(Self::fixed_utc_time()))
    }
}
impl Default for EventTime {
    fn default() -> Self {
        Self::DateTime(Default::default())
    }
}
impl Display for EventTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventTime::DateTime(x) => {
                write!(f, "{}", x.to_datetime_string())
            }
            EventTime::DateOnly(x) => {
                write!(f, "{}", x.to_date_only_string())
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TimeType {
    Utc(DateTime<Utc>),
    Local(DateTime<Local>),
}
impl TimeType {
    pub fn to_date_only_string(&self) -> String {
        //  allDay: "YYYYMMDD"
        // Example: "20250530"
        match self {
            TimeType::Utc(x) => x.format("%Y%m%d").to_string(),
            TimeType::Local(x) => x.format("%Y%m%d").to_string(),
        }
    }
    pub fn to_datetime_string(&self) -> String {
        match self {
            TimeType::Utc(x) => {
                // dateTimeUTC: "YYYYMMDD[T]HHmmss[Z]"
                // Example: "20250530T175111Z" (UTC time)
                x.format("%Y%m%dT%H%M%SZ").to_string()
            }
            TimeType::Local(x) => {
                // dateTimeLocal: "YYYY-MM-DD[T]HH:mm:ss"
                // Example: "2025-05-30T19:51:11" (assuming CEST, which is UTC+2)
                x.format("%Y-%m-%dT%H:%M:%S").to_string()
            }
        }
    }
}
impl Default for TimeType {
    fn default() -> Self {
        Self::Utc(Utc::now())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_utc() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::DateTime(TimeType::Utc(d));
        assert_eq!(x.to_string(), "20250530T175111Z");

        let x = EventTime::DateOnly(TimeType::Utc(d));
        assert_eq!(x.to_string(), "20250530");
    }
    #[test]
    fn should_format_local() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::DateTime(TimeType::Local(d.into()));
        assert_eq!(x.to_string(), "2025-05-30T21:21:11"); // it may not pass on your system

        let x = EventTime::DateOnly(TimeType::Local(d.into()));
        assert_eq!(x.to_string(), "20250530");
    }
}
